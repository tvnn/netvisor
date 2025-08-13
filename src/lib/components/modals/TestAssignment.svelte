<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { X, AlertTriangle } from 'lucide-svelte';
  import type { Node, TestType, TestCriticality } from '../../types';
  import { getTestTypeDisplayName } from '../../types';
  
  export let node: Node | null = null;
  export let isOpen = false;
  
  const dispatch = createEventDispatcher();
  
  let selectedTestType: TestType = 'Connectivity';
  let selectedCriticality: TestCriticality = 'Important';
  let monitorInterval = '';
  let testConfig = {
    port: '',
    domain: '',
    timeout: '30000',
    path: '/'
  };
  let loading = false;
  
  const testTypes: TestType[] = [
    'Connectivity', 'DirectIp', 'Ping', 'WellknownIp',
    'DnsResolution', 'DnsOverHttps',
    'VpnConnectivity', 'VpnTunnel',
    'ServiceHealth'
  ];
  
  const criticalityLevels: TestCriticality[] = ['Critical', 'Important', 'Informational'];
  
  $: existingTests = node?.assigned_tests.map(t => t.test_type) || [];
  $: isTestAlreadyAssigned = existingTests.includes(selectedTestType);
  $: nodeTarget = node?.ip || node?.domain || node?.name || '';
  
  function resetForm() {
    selectedTestType = 'Connectivity';
    selectedCriticality = 'Important';
    monitorInterval = '';
    testConfig = {
      port: '',
      domain: '',
      timeout: '30000',
      path: '/'
    };
  }
  
  function getTestConfigForType(testType: TestType) {
    const baseConfig = {
      timeout: parseInt(testConfig.timeout) || 30000,
      expected_result: 'Success'
    };
    
    switch (testType) {
      case 'Connectivity':
        return {
          Connectivity: {
            base: baseConfig,
            target: nodeTarget,
            port: testConfig.port ? parseInt(testConfig.port) : undefined,
            protocol: 'http'
          }
        };
      case 'DirectIp':
        return {
          DirectIp: {
            base: baseConfig,
            target: node?.ip || '',
            port: parseInt(testConfig.port) || 80
          }
        };
      case 'Ping':
        return {
          Ping: {
            base: baseConfig,
            target: nodeTarget,
            attempts: 4
          }
        };
      case 'WellknownIp':
        return {
          WellknownIp: {
            base: baseConfig
          }
        };
      case 'DnsResolution':
        return {
          DnsResolution: {
            base: baseConfig,
            domain: testConfig.domain || 'example.com'
          }
        };
      case 'ServiceHealth':
        return {
          ServiceHealth: {
            base: baseConfig,
            target: nodeTarget,
            port: parseInt(testConfig.port) || 80,
            path: testConfig.path || '/',
            expected_status: 200
          }
        };
      case 'VpnConnectivity':
        return {
          VpnConnectivity: {
            base: baseConfig,
            target: nodeTarget,
            port: parseInt(testConfig.port) || 51820
          }
        };
      case 'VpnTunnel':
        return {
          VpnTunnel: {
            base: baseConfig,
            expected_subnet: '10.100.0.0/24'
          }
        };
      default:
        return {};
    }
  }
  
  async function handleSubmit() {
    if (!node) return;
    
    loading = true;
    
    try {
      const requestData = {
        node_id: node.id,
        test_type: selectedTestType,
        test_config: getTestConfigForType(selectedTestType),
        criticality: selectedCriticality,
        monitor_interval_minutes: monitorInterval ? parseInt(monitorInterval) : undefined
      };
      
      dispatch('assign', requestData);
      handleClose();
    } catch (error) {
      console.error('Failed to assign test:', error);
    } finally {
      loading = false;
    }
  }
  
  function handleClose() {
    dispatch('close');
    resetForm();
  }
  
  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      handleClose();
    }
  }
</script>

{#if isOpen}
  <div 
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    on:click={handleBackdropClick}
    on:keydown={(e) => e.key === 'Escape' && handleClose()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
      <!-- Header -->
      <div class="flex items-center justify-between p-6 border-b border-gray-700">
        <h2 class="text-xl font-semibold text-white">
          Assign Test to {node?.name}
        </h2>
        <button
          on:click={handleClose}
          class="text-gray-400 hover:text-white transition-colors"
        >
          <X class="w-6 h-6" />
        </button>
      </div>
      
      <!-- Form -->
      <form on:submit|preventDefault={handleSubmit} class="p-6 space-y-4">
        <!-- Target Info -->
        <div class="bg-gray-700/50 rounded-lg p-3">
          <div class="text-sm text-gray-400 mb-1">Test Target:</div>
          <div class="text-white font-medium">{nodeTarget}</div>
          <div class="text-xs text-gray-500">
            Tests will automatically target this node
          </div>
        </div>
        
        <!-- Test Type -->
        <div>
          <label for="test_type" class="block text-sm font-medium text-gray-300 mb-1">
            Test Type
          </label>
          <select
            id="test_type"
            bind:value={selectedTestType}
            class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {#each testTypes as testType}
              <option value={testType}>{getTestTypeDisplayName(testType)}</option>
            {/each}
          </select>
          
          {#if isTestAlreadyAssigned}
            <div class="flex items-center gap-2 mt-2 p-2 bg-yellow-600/20 border border-yellow-600/50 rounded">
              <AlertTriangle class="w-4 h-4 text-yellow-400" />
              <span class="text-yellow-300 text-xs">This test is already assigned to this node</span>
            </div>
          {/if}
        </div>
        
        <!-- Criticality -->
        <div>
          <label for="criticality" class="block text-sm font-medium text-gray-300 mb-1">
            Criticality
          </label>
          <select
            id="criticality"
            bind:value={selectedCriticality}
            class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {#each criticalityLevels as level}
              <option value={level}>{level}</option>
            {/each}
          </select>
        </div>
        
        <!-- Monitor Interval -->
        <div>
          <label for="monitor_interval" class="block text-sm font-medium text-gray-300 mb-1">
            Monitor Interval (minutes)
          </label>
          <input
            id="monitor_interval"
            bind:value={monitorInterval}
            type="number"
            min="1"
            class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Leave empty for diagnostic-only"
          />
          <p class="text-xs text-gray-400 mt-1">
            Leave empty to only run during diagnostics
          </p>
        </div>
        
        <!-- Test-specific Configuration -->
        <div class="space-y-3">
          <h3 class="text-sm font-medium text-gray-300">Configuration</h3>
          
          <!-- Port for connection-based tests -->
          {#if ['Connectivity', 'DirectIp', 'ServiceHealth', 'VpnConnectivity'].includes(selectedTestType)}
            <div>
              <label for="port" class="block text-sm font-medium text-gray-400 mb-1">
                Port {selectedTestType === 'VpnConnectivity' ? '(default: 51820)' : '(default: 80)'}
              </label>
              <input
                id="port"
                bind:value={testConfig.port}
                type="number"
                min="1"
                max="65535"
                class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder={selectedTestType === 'VpnConnectivity' ? '51820' : '80'}
              />
            </div>
          {/if}
          
          <!-- Path for ServiceHealth -->
          {#if selectedTestType === 'ServiceHealth'}
            <div>
              <label for="path" class="block text-sm font-medium text-gray-400 mb-1">
                Path
              </label>
              <input
                id="path"
                bind:value={testConfig.path}
                type="text"
                class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="/"
              />
            </div>
          {/if}
          
          <!-- Domain for DNS tests -->
          {#if selectedTestType === 'DnsResolution'}
            <div>
              <label for="domain" class="block text-sm font-medium text-gray-400 mb-1">
                Domain to Resolve
              </label>
              <input
                id="domain"
                bind:value={testConfig.domain}
                type="text"
                class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="example.com"
              />
            </div>
          {/if}
          
          <!-- Timeout -->
          <div>
            <label for="timeout" class="block text-sm font-medium text-gray-400 mb-1">
              Timeout (ms)
            </label>
            <input
              id="timeout"
              bind:value={testConfig.timeout}
              type="number"
              min="1000"
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="30000"
            />
          </div>
        </div>
        
        <!-- Actions -->
        <div class="flex justify-end gap-3 pt-4 border-t border-gray-700">
          <button
            type="button"
            on:click={handleClose}
            class="px-4 py-2 text-gray-300 border border-gray-600 rounded-md hover:bg-gray-700 transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={loading}
            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {#if loading}
              Assigning...
            {:else}
              Assign Test
            {/if}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}