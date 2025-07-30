// src/lib/tauri-commands.ts (Refactored - single command)
import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import type { NetworkNode, Test, DiagnosticResults, CheckResult, CheckConfig } from './types';
import { CHECK_TYPES } from './stores/checks';

// Enhanced logging for desktop debugging
function debugLog(level: 'info' | 'warn' | 'error', message: string, data?: any) {
  const timestamp = new Date().toISOString().slice(11, 23);
  const prefix = `[${timestamp}] [TAURI-${level.toUpperCase()}]`;
  
  if (data) {
    console[level](`${prefix} ${message}`, data);
  } else {
    console[level](`${prefix} ${message}`);
  }
}

// Simple wrapper around Tauri's invoke function with enhanced debugging
export async function invoke<T>(command: string, args: Record<string, unknown> = {}): Promise<T> {
  const startTime = performance.now();
  
  debugLog('info', `Calling command: ${command}`, args);
  
  try {
    const result = await tauriInvoke(command, args);
    const duration = Math.round(performance.now() - startTime);
    
    debugLog('info', `Command ${command} succeeded in ${duration}ms`, {
      result: typeof result === 'object' && result ? `${Object.keys(result).length} properties` : result,
      duration
    });
    
    return result as T;
  } catch (error) {
    const duration = Math.round(performance.now() - startTime);
    
    debugLog('error', `Command ${command} failed after ${duration}ms`, {
      error: error instanceof Error ? error.message : error,
      args,
      duration
    });
    
    throw error;
  }
}

// Universal check function - all checks use the single execute_check command
async function executeCheck(checkType: string, config: CheckConfig): Promise<CheckResult> {
  const checkInfo = CHECK_TYPES[checkType];
  if (!checkInfo) {
    throw new Error(`Unknown check type: ${checkType}`);
  }

  debugLog('info', `Running ${checkInfo.name}`, config);
  
  const result = await invoke<CheckResult>('execute_check', { 
    check_type: checkType, 
    config: config as Record<string, unknown> 
  });
  
  debugLog('info', `${checkInfo.name} ${result.success ? 'PASSED' : 'FAILED'}`, {
    duration: result.duration,
    message: result.message
  });
  
  return result;
}

// Generate check functions dynamically from CHECK_TYPES - all use the same executeCheck
const checkFunctions = Object.keys(CHECK_TYPES).reduce((acc, checkType) => {
  acc[checkType] = (config: CheckConfig) => executeCheck(checkType, config);
  return acc;
}, {} as Record<string, (config: CheckConfig) => Promise<CheckResult>>);

// Static command wrappers (non-check related)
const staticCommands = {
  // Node operations
  getNodes: async () => {
    debugLog('info', 'Fetching all nodes...');
    const result = await invoke<NetworkNode[]>('get_nodes');
    debugLog('info', `Retrieved ${result.length} nodes`);
    return result;
  },

  saveNode: async (node: NetworkNode) => {
    debugLog('info', `Saving node: ${node.name} (ID: ${node.id})`);
    const result = await invoke<boolean>('save_node', { node });
    debugLog('info', `Node save result: ${result}`);
    return result;
  },

  updateNode: async (id: string, node: NetworkNode) => {
    debugLog('info', `Updating node: ${id} -> ${node.name}`);
    const result = await invoke<boolean>('update_node', { id, node });
    debugLog('info', `Node update result: ${result}`);
    return result;
  },

  deleteNode: async (id: string) => {
    debugLog('info', `Deleting node: ${id}`);
    const result = await invoke<boolean>('delete_node', { id });
    debugLog('info', `Node delete result: ${result}`);
    return result;
  },

  // Test operations
  getTests: async () => {
    debugLog('info', 'Fetching all tests...');
    const result = await invoke<Test[]>('get_tests');
    debugLog('info', `Retrieved ${result.length} tests`);
    return result;
  },

  saveTest: async (test: Test) => {
    debugLog('info', `Saving test: ${test.name}`, {
      id: test.id,
      layers: test.layers.length,
      totalChecks: test.layers.reduce((sum, layer) => sum + layer.checks.length, 0)
    });
    const result = await invoke<boolean>('save_test', { test });
    debugLog('info', `Test save result: ${result}`);
    return result;
  },

  updateTest: async (id: string, test: Test) => {
    debugLog('info', `Updating test: ${id} -> ${test.name}`);
    const result = await invoke<boolean>('update_test', { id, test });
    debugLog('info', `Test update result: ${result}`);
    return result;
  },

  deleteTest: async (id: string) => {
    debugLog('info', `Deleting test: ${id}`);
    const result = await invoke<boolean>('delete_test', { id });
    debugLog('info', `Test delete result: ${result}`);
    return result;
  },

  // Diagnostic operations
  runDiagnostics: async (test: Test) => {
    debugLog('info', `Running diagnostics for test: ${test.name}`, {
      layers: test.layers.length,
      totalChecks: test.layers.reduce((sum, layer) => sum + layer.checks.length, 0)
    });
    
    const startTime = performance.now();
    const result = await invoke<DiagnosticResults>('run_diagnostics', { test });
    const duration = Math.round(performance.now() - startTime);
    
    debugLog('info', `Diagnostics completed in ${duration}ms`, {
      success: result.success,
      totalDuration: result.totalDuration,
      layersCompleted: result.layers.length,
      failedLayers: result.layers.filter(l => !l.success).length
    });
    
    return result;
  },

  getDiagnosticResults: async () => {
    debugLog('info', 'Fetching diagnostic results...');
    const result = await invoke<DiagnosticResults | null>('get_diagnostic_results');
    debugLog('info', result ? 'Found previous diagnostic results' : 'No previous diagnostic results');
    return result;
  },

  // File operations
  exportData: async (type: string, data: any, filename: string) => {
    debugLog('info', `Exporting ${type} data to ${filename}`);
    const result = await invoke<boolean>('export_data', { type, data, filename });
    debugLog('info', `Export result: ${result}`);
    return result;
  },

  importData: async (type: string, filepath: string) => {
    debugLog('info', `Importing ${type} data from ${filepath}`);
    const result = await invoke<{ success: boolean; data: any }>('import_data', { type, filepath });
    debugLog('info', `Import result: ${result.success}`);
    return result;
  }
};

// Combine static commands with dynamically generated check functions
export const commands = {
  ...staticCommands,
  ...checkFunctions
} as typeof staticCommands & Record<string, (config: CheckConfig) => Promise<CheckResult>>;

// Add debug helper to window for manual debugging
if (typeof window !== 'undefined') {
  (window as any).tauriDebug = {
    commands,
    executeCommand: async (command: string, args: any = {}) => {
      debugLog('info', `Manual command: ${command}`, args);
      try {
        const result = await invoke(command, args);
        debugLog('info', `Manual command result:`, result);
        return result;
      } catch (error) {
        debugLog('error', `Manual command failed:`, error);
        throw error;
      }
    },
    executeCheck: (checkType: string, config: CheckConfig) => executeCheck(checkType, config),
    enableVerbose: () => {
      console.log('Verbose Tauri debugging enabled');
      (window as any).TAURI_DEBUG = true;
    },
    listAvailableChecks: () => {
      console.log('Available check functions:', Object.keys(checkFunctions));
      console.log('Available check types:', Object.keys(CHECK_TYPES));
    }
  };
}