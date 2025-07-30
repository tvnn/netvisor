<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { 
    Play, 
    Square, 
    RotateCcw, 
    CheckCircle, 
    XCircle, 
    Clock, 
    Zap, 
    AlertTriangle, 
    Info,
    ChevronDown,
    ChevronRight,
    Download,
    Share,
    Server,
    GitBranch
  } from 'lucide-svelte';
  import { commands } from '../../tauri-commands';
  import { tests } from '../../stores/tests';
  import { nodes } from '../../stores/nodes';
  import { loadingActions, notificationActions } from '../../stores/ui';
  import type { Test, DiagnosticResults, NetworkNode, CheckResult } from '../../types';

  // Selected test for running diagnostics
  let selectedTestId: string | null = null;
  
  // Execution state
  let isRunning: boolean = false;
  let currentResults: DiagnosticResults | null = null;
  let executionProgress = {
    currentLayer: null as string | null,
    currentCheck: null as string | null,
    layersCompleted: 0,
    checksCompleted: 0,
    totalLayers: 0,
    totalChecks: 0,
    startTime: null as number | null,
    elapsedTime: 0
  };

  // UI state
  let expandedLayers: Set<string> = new Set<string>();
  let expandedChecks: Set<string> = new Set<string>();
  let autoScroll: boolean = true;
  let progressTimer: number | null = null;

  // Reactive computations
  $: selectedTest = selectedTestId ? $tests.find(t => t.id === selectedTestId) : null;
  $: hasSelectedTest = selectedTest !== null;
  $: canRun = hasSelectedTest && !isRunning;
  $: interpolatedTest = selectedTest ? interpolateTest(selectedTest, $nodes) : null;
  $: overallSuccess = currentResults ? currentResults.success : null;
  $: progressPercentage = executionProgress.totalLayers > 0 
    ? Math.round((executionProgress.layersCompleted / executionProgress.totalLayers) * 100) 
    : 0;

  // Auto-select first test if none selected
  $: if (!selectedTestId && $tests.length > 0) {
    const firstTestId = $tests[0]?.id;
    if (firstTestId) {
      selectedTestId = firstTestId;
    }
  }

  // Helper function to interpolate node references in test
  function interpolateTest(test: Test, availableNodes: NetworkNode[]): Test {
    const nodeMap = new Map(availableNodes.map(node => [node.id, node]));
    
    function interpolateValue(value: any): any {
      if (typeof value === 'string' && value.startsWith('{{') && value.endsWith('}}')) {
        const match = value.match(/^\{\{([^.}]+)(?:\.([^}]+))?\}\}$/);
        if (match) {
          const [, nodeId, field] = match;
          const node = nodeMap.get(nodeId);
          if (node) {
            switch (field) {
              case 'domain': return node.domain || '';
              case 'ip': return node.ip || '';
              case 'defaultPort': return node.defaultPort?.toString() || '';
              case 'path': return node.path || '';
              default: return node.domain || node.ip || '';
            }
          }
        }
        return value; // Return as-is if can't interpolate
      }
      
      if (typeof value === 'object' && value !== null) {
        if (Array.isArray(value)) {
          return value.map(interpolateValue);
        } else {
          const result: any = {};
          for (const [key, val] of Object.entries(value)) {
            result[key] = interpolateValue(val);
          }
          return result;
        }
      }
      
      return value;
    }
    
    return interpolateValue(test) as Test;
  }

  onMount(async () => {
    // Load any previous results
    try {
      const results = await commands.getDiagnosticResults();
      if (results) {
        currentResults = results;
        expandAllOnResults();
      }
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      console.warn('No previous diagnostic results found:', errorMessage);
    }
  });

  onDestroy(() => {
    if (progressTimer) {
      clearInterval(progressTimer);
    }
  });

  function expandAllOnResults(): void {
    if (currentResults) {
      currentResults.layers.forEach(layer => {
        expandedLayers.add(layer.id);
        layer.checks.forEach((_, checkIndex) => {
          expandedChecks.add(`${layer.id}-${checkIndex}`);
        });
      });
      expandedLayers = expandedLayers;
      expandedChecks = expandedChecks;
    }
  }

  function toggleLayer(layerId: string): void {
    if (expandedLayers.has(layerId)) {
      expandedLayers.delete(layerId);
      // Also collapse all tests in this layer
      if (currentResults) {
        const layer = currentResults.layers.find(l => l.id === layerId);
        if (layer) {
          layer.checks.forEach((_, checkIndex) => {
            expandedChecks.delete(`${layerId}-${checkIndex}`);
          });
        }
      }
    } else {
      expandedLayers.add(layerId);
    }
    expandedLayers = expandedLayers;
    expandedChecks = expandedChecks;
  }

  function toggleCheck(layerId: string, checkIndex: number): void {
    const checkId = `${layerId}-${checkIndex}`;
    if (expandedChecks.has(checkId)) {
      expandedChecks.delete(checkId);
    } else {
      expandedChecks.add(checkId);
    }
    expandedChecks = expandedChecks;
  }

  async function runDiagnostics(): Promise<void> {
    if (!interpolatedTest || isRunning) return;

    isRunning = true;
    currentResults = null;
    loadingActions.setDiagnostics(true);
    
    // Initialize progress tracking
    executionProgress = {
      currentLayer: null,
      currentCheck: null,
      layersCompleted: 0,
      checksCompleted: 0,
      totalLayers: interpolatedTest.layers.length,
      totalChecks: interpolatedTest.layers.reduce((sum, layer) => sum + layer.checks.length, 0),
      startTime: Date.now(),
      elapsedTime: 0
    };
    
    // Start progress timer
    progressTimer = setInterval(() => {
      if (executionProgress.startTime) {
        executionProgress.elapsedTime = Date.now() - executionProgress.startTime;
      }
    }, 100);
    
    try {
      notificationActions.info(`Starting diagnostics for ${selectedTest?.name}...`);
      
      const results = await commands.runDiagnostics(interpolatedTest);
      
      currentResults = results;
      expandAllOnResults();
      
      if (results.success) {
        notificationActions.success(`Diagnostics completed successfully in ${formatDuration(results.totalDuration)}`);
      } else {
        notificationActions.warning(`Diagnostics completed with failures in ${formatDuration(results.totalDuration)}`);
      }
      
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      console.error('Diagnostics failed:', error);
      notificationActions.error(`Diagnostics failed: ${errorMessage}`);
    } finally {
      isRunning = false;
      loadingActions.setDiagnostics(false);
      
      if (progressTimer) {
        clearInterval(progressTimer);
        progressTimer = null;
      }
    }
  }

  function stopDiagnostics(): void {
    isRunning = false;
    loadingActions.setDiagnostics(false);
    
    if (progressTimer) {
      clearInterval(progressTimer);
      progressTimer = null;
    }
    
    notificationActions.info('Diagnostics stopped by user');
  }

  function clearResults(): void {
    currentResults = null;
    expandedLayers.clear();
    expandedChecks.clear();
    expandedLayers = expandedLayers;
    expandedChecks = expandedChecks;
  }

  function formatDuration(ms: number): string {
    if (ms < 1000) return `${ms}ms`;
    return `${(ms / 1000).toFixed(1)}s`;
  }

  function getStatusColor(success: boolean): string {
    return success ? 'text-green-400' : 'text-red-400';
  }

  function getStatusIcon(success: boolean) {
    return success ? CheckCircle : XCircle;
  }

  function exportResults(): void {
    if (!currentResults) return;
    
    const filename = `diagnostic-results-${new Date().toISOString().split('T')[0]}.json`;
    commands.exportData('diagnostics', currentResults, filename)
      .then(() => {
        notificationActions.success('Results exported successfully');
      })
      .catch((error: unknown) => {
        const errorMessage = error instanceof Error ? error.message : 'Unknown error';
        notificationActions.error(`Export failed: ${errorMessage}`);
      });
  }
</script>

<div class="space-y-6">
  <!-- Header with Test Selector -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold text-white">Diagnostics</h2>
      <p class="text-gray-400 mt-1">Run network tests</p>
    </div>
  </div>

  <!-- Test Selector and Controls -->
  <div class="bg-gray-800 rounded-xl border border-gray-700 p-6">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-semibold text-white">Select Test Configuration</h3>
      
      <!-- Action Buttons -->
      <div class="flex items-center gap-3">
        {#if currentResults}
          <button
            on:click={exportResults}
            class="flex items-center gap-2 px-3 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors"
          >
            <Download class="w-4 h-4" />
            Export
          </button>
          <button
            on:click={clearResults}
            class="flex items-center gap-2 px-3 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors"
          >
            <RotateCcw class="w-4 h-4" />
            Clear
          </button>
        {/if}
        
        {#if isRunning}
          <button
            on:click={stopDiagnostics}
            class="flex items-center gap-2 px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-colors"
          >
            <Square class="w-4 h-4" />
            Stop
          </button>
        {:else}
          <button
            on:click={runDiagnostics}
            disabled={!canRun}
            class="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:opacity-50 text-white rounded-lg transition-colors"
          >
            <Play class="w-4 h-4" />
            Run
          </button>
        {/if}
      </div>
    </div>

    <!-- Test Selector -->
    {#if $tests.length > 0}
      <div class="space-y-4">
        <div>
          <label for="test-selector" class="block text-sm font-medium text-gray-300 mb-2">
            Test
          </label>
          <select
            id="test-selector"
            bind:value={selectedTestId}
            disabled={isRunning}
            class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50"
          >
            {#each $tests as test}
              <option value={test.id}>
                {test.name} ({test.layers?.length || 0} layers, {test.layers?.reduce((sum, layer) => sum + (layer.checks?.length || 0), 0) || 0} tests)
              </option>
            {/each}
          </select>
        </div>

        <!-- Selected Test Preview -->
        {#if selectedTest}
          <div class="bg-gray-700/50 rounded-lg p-4">
            <div class="flex items-start gap-3">
              <div class="p-2 bg-purple-600/20 border border-purple-600/30 rounded-lg">
                <GitBranch class="w-5 h-5 text-purple-400" />
              </div>
              <div class="flex-1 min-w-0">
                <h4 class="font-semibold text-white mb-1">{selectedTest.name}</h4>
                {#if selectedTest.description}
                  <p class="text-gray-400 text-sm mb-2">{selectedTest.description}</p>
                {/if}
                <div class="flex items-center gap-4 text-sm text-gray-300">
                  <span>Version: {selectedTest.version || '1.0'}</span>
                  <span>•</span>
                  <span>{selectedTest.layers?.length || 0} layers</span>
                  <span>•</span>
                  <span>{selectedTest.layers?.reduce((sum, layer) => sum + (layer.checks?.length || 0), 0) || 0} total tests</span>
                </div>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {:else}
      <div class="text-center py-8">
        <GitBranch class="w-12 h-12 mx-auto text-gray-600 mb-3" />
        <h4 class="text-lg font-semibold text-gray-300 mb-2">No Tests</h4>
        <p class="text-gray-400 mb-4">Create a test in the Tests tab to run diagnostics.</p>
      </div>
    {/if}
  </div>

  <!-- Progress Indicator -->
  {#if isRunning}
    <div class="bg-gray-800 rounded-xl border border-gray-700 p-6">
      <div class="flex items-center gap-4 mb-4">
        <div class="animate-spin">
          <Clock class="w-5 h-5 text-blue-400" />
        </div>
        <div class="flex-1">
          <div class="flex items-center justify-between mb-2">
            <span class="text-white font-medium">Running Diagnostics...</span>
            <span class="text-gray-400 text-sm">{progressPercentage}%</span>
          </div>
          <div class="w-full bg-gray-700 rounded-full h-2">
            <div class="bg-blue-600 h-2 rounded-full transition-all duration-300" style="width: {progressPercentage}%"></div>
          </div>
        </div>
      </div>
      <div class="text-sm text-gray-400">
        {executionProgress.layersCompleted} of {executionProgress.totalLayers} layers completed
        • {Math.round(executionProgress.elapsedTime / 1000)}s elapsed
      </div>
    </div>
  {/if}

  <!-- Results Display -->
  {#if currentResults}
    <div class="bg-gray-800 rounded-xl border border-gray-700 overflow-hidden">
      <!-- Results Header -->
      <div class="p-6 border-b border-gray-700">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="flex items-center gap-2">
              {#if overallSuccess !== null}
                <svelte:component this={getStatusIcon(overallSuccess)} class="w-5 h-5 {getStatusColor(overallSuccess)}" />
                <h3 class="text-lg font-semibold text-white">
                  {overallSuccess ? 'All Tests Passed' : 'Tests Failed'}
                </h3>
              {:else}
                <h3 class="text-lg font-semibold text-white">Test Results</h3>
              {/if}
            </div>
            {#if overallSuccess !== null}
              <span class="px-2 py-1 rounded text-xs font-medium {overallSuccess ? 'bg-green-600/20 text-green-400' : 'bg-red-600/20 text-red-400'}">
                {overallSuccess ? 'SUCCESS' : 'FAILURE'}
              </span>
            {/if}
          </div>
          <div class="text-sm text-gray-400">
            Completed in {formatDuration(currentResults.totalDuration)}
          </div>
        </div>
        
        <div class="mt-4 text-sm text-gray-300">
          <span class="font-medium">{currentResults.test}</span>
          <span class="mx-2">•</span>
          <span>{new Date(currentResults.timestamp).toLocaleString()}</span>
          <span class="mx-2">•</span>
          <span>{currentResults.layers.length} layers tested</span>
        </div>
      </div>

      <!-- Layer Results -->
      <div class="divide-y divide-gray-700">
        {#each currentResults.layers as layer, layerIndex}
          <div class="p-4">
            <!-- Layer Header -->
            <button
              class="w-full flex items-center justify-between p-2 hover:bg-gray-700/50 rounded-lg transition-colors"
              on:click={() => toggleLayer(layer.id)}
            >
              <div class="flex items-center gap-3">
                <svelte:component this={expandedLayers.has(layer.id) ? ChevronDown : ChevronRight} class="w-4 h-4 text-gray-400" />
                <svelte:component this={getStatusIcon(layer.success)} class="w-5 h-5 {getStatusColor(layer.success)}" />
                <div class="text-left">
                  <h4 class="font-medium text-white">{layer.name}</h4>
                  <p class="text-sm text-gray-400">{layer.description}</p>
                </div>
              </div>
              <div class="text-right text-sm text-gray-400">
                <div>{formatDuration(layer.duration)}</div>
                <div>{layer.checks.filter(t => t.success).length}/{layer.checks.length} passed</div>
              </div>
            </button>

            <!-- Layer Details -->
            {#if expandedLayers.has(layer.id)}
              <div class="mt-4 ml-6 space-y-3">
                <!-- Test Results -->
                {#each layer.checks as check, checkIndex}
                  <div class="border border-gray-600 rounded-lg overflow-hidden">
                    <button
                      class="w-full flex items-center justify-between p-3 hover:bg-gray-700/30 transition-colors"
                      on:click={() => toggleCheck(layer.id, checkIndex)}
                    >
                      <div class="flex items-center gap-3">
                        <svelte:component this={expandedChecks.has(`${layer.id}-${checkIndex}`) ? ChevronDown : ChevronRight} class="w-3 h-3 text-gray-400" />
                        <svelte:component this={getStatusIcon(check.success)} class="w-4 h-4 {getStatusColor(check.success)}" />
                        <div class="text-left">
                          <span class="font-medium text-white">{check.type.replace('_', ' ').toUpperCase()}</span>
                          {#if check.config.target}
                            <span class="text-gray-400 ml-2">{check.config.target}</span>
                          {/if}
                        </div>
                      </div>
                      <div class="text-right text-sm">
                        <div class="{getStatusColor(check.success)}">{check.success ? 'PASS' : 'FAIL'}</div>
                        <div class="text-gray-400">{formatDuration(check.duration)}</div>
                      </div>
                    </button>

                    <!-- Check Details -->
                    {#if expandedChecks.has(`${layer.id}-${checkIndex}`)}
                      <div class="p-3 bg-gray-700/30 border-t border-gray-600">
                        <div class="space-y-2 text-sm">
                          <div>
                            <span class="text-gray-400">Message:</span>
                            <span class="text-white ml-2">{check.message}</span>
                          </div>
                          {#if check.error}
                            <div>
                              <span class="text-gray-400">Error:</span>
                              <span class="text-red-400 ml-2">{check.error}</span>
                            </div>
                          {/if}
                          {#if check.details}
                            <div>
                              <span class="text-gray-400">Details:</span>
                              <pre class="text-xs text-gray-300 mt-1 bg-gray-800 p-2 rounded overflow-x-auto">{JSON.stringify(check.details, null, 2)}</pre>
                            </div>
                          {/if}
                        </div>
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Empty State -->
  {#if hasSelectedTest && !currentResults && !isRunning}
    <div class="text-center py-12">
      <Zap class="w-16 h-16 mx-auto text-gray-600 mb-4" />
      <h3 class="text-xl font-semibold text-gray-300 mb-2">Ready to Run Diagnostics</h3>
      <p class="text-gray-400 mb-6 max-w-md mx-auto">
        Click "Run Diagnostics" to test your network configuration with the selected test.
      </p>
    </div>
  {/if}
</div>