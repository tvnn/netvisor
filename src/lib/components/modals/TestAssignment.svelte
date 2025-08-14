<script lang="ts">
  import { X, AlertTriangle, Trash2 } from 'lucide-svelte';
  import type { Node, AssignedTest } from "$lib/types/nodes";
  import type { TestType } from "$lib/types/tests";
  import type { TestCriticality } from "$lib/types/nodes";
  import { getTestTypeDisplayName } from "$lib/types/tests";
  import { getTestDescription } from "$lib/types/tests";
  
  export let node: Node | null = null;
  export let test: AssignedTest | null = null;
  export let isOpen = false;
  export let onAssigned: (node: Node, warning?: string) => void = () => {};
  export let onClose: () => void = () => {};
  
  let selectedTestType: TestType = 'Connectivity';
  let selectedCriticality: TestCriticality = 'Important';
  let monitorInterval = '5';
  let enabled = true;
  let testConfig = {
    port: '',
    domain: '',
    timeout: '30000',
    path: '/',
    target: '',
    expected_subnet: '10.100.0.0/24',
    attempts: '4',
    expected_result: 'Success'
  };
  let loading = false;
  let deleting = false;
  let compatibilityInfo: any = null;
  let loadingCompatibility = false;
  
  const testTypes: TestType[] = [
    'Connectivity', 'DirectIp', 'Ping', 'WellknownIp',
    'DnsResolution', 'DnsOverHttps',
    'VpnConnectivity', 'VpnTunnel',
    'ServiceHealth'
  ];
  
  const criticalityLevels: TestCriticality[] = ['Critical', 'Important', 'Informational'];
  
  $: existingTests = node?.assigned_tests.map(t => t.test_type) || [];
  $: isTestAlreadyAssigned = !test && existingTests.includes(selectedTestType);
  $: nodeTarget = node?.ip || node?.domain || node?.name || '';
  $: isEditMode = test !== null;
  
  // Load compatibility info when node changes and reset form with node's values
  $: if (node && isOpen) {
    loadCompatibilityInfo();
    resetForm();
  }
  
  async function loadCompatibilityInfo() {
    if (!node) return;
    
    loadingCompatibility = true;
    try {
      const response = await fetch(`/api/tests/compatibility/${node.id}`);
      if (response.ok) {
        compatibilityInfo = await response.json();
      }
    } catch (error) {
      console.error('Failed to load compatibility info:', error);
    } finally {
      loadingCompatibility = false;
    }
  }
  
  function resetForm() {
    if (test && node) {
      // Edit mode - populate with existing test data
      selectedTestType = test.test_type;
      selectedCriticality = test.criticality;
      monitorInterval = test.monitor_interval_minutes?.toString() || '';
      enabled = test.enabled;
      
      // Extract config fields from the flattened structure
      const config = extractConfigFromTest(test);
      testConfig = {
        ...testConfig,
        ...config
      };
    } else {
      // Create mode - reset to defaults
      selectedTestType = 'Connectivity';
      selectedCriticality = 'Important';
      monitorInterval = '5';
      enabled = true;
      testConfig = {
        port: node?.port?.toString() || '',
        domain: '',
        timeout: '30000',
        path: '/',
        target: nodeTarget,
        expected_subnet: '10.100.0.0/24',
        attempts: '4',
        expected_result: 'Success'
      };
    }
  }
  
  function extractConfigFromTest(test: AssignedTest): any {
    const config = test.test_config;
    
    // Extract fields from the nested test configuration
    if ('Connectivity' in config) {
      return {
        target: config.Connectivity.target,
        port: config.Connectivity.port?.toString() || '',
        timeout: config.Connectivity.timeout?.toString() || '30000',
        expected_result: config.Connectivity.expected_result
      };
    } else if ('DirectIp' in config) {
      return {
        target: config.DirectIp.target,
        port: config.DirectIp.port.toString(),
        timeout: config.DirectIp.timeout?.toString() || '30000',
        expected_result: config.DirectIp.expected_result
      };
    } else if ('Ping' in config) {
      return {
        target: config.Ping.target,
        port: config.Ping.port?.toString() || '',
        attempts: config.Ping.attempts?.toString() || '4',
        timeout: config.Ping.timeout?.toString() || '30000',
        expected_result: config.Ping.expected_result
      };
    } else if ('ServiceHealth' in config) {
      return {
        target: config.ServiceHealth.target,
        port: config.ServiceHealth.port?.toString() || '',
        path: config.ServiceHealth.path || '/',
        timeout: config.ServiceHealth.timeout?.toString() || '30000',
        expected_result: config.ServiceHealth.expected_result
      };
    } else if ('DnsResolution' in config) {
      return {
        domain: config.DnsResolution.domain,
        timeout: config.DnsResolution.timeout?.toString() || '30000',
        expected_result: config.DnsResolution.expected_result
      };
    } else if ('VpnConnectivity' in config) {
      return {
        target: config.VpnConnectivity.target,
        port: config.VpnConnectivity.port?.toString() || '51820',
        timeout: config.VpnConnectivity.timeout?.toString() || '30000',
        expected_result: config.VpnConnectivity.expected_result
      };
    } else if ('VpnTunnel' in config) {
      return {
        expected_subnet: config.VpnTunnel.expected_subnet,
        timeout: config.VpnTunnel.timeout?.toString() || '30000',
        expected_result: config.VpnTunnel.expected_result
      };
    }
    
    return {};
  }
  
  function getTestConfigForType(testType: TestType) {
    const baseConfig = {
      timeout: parseInt(testConfig.timeout) || 30000,
      expected_result: testConfig.expected_result || 'Success'
    };
    
    switch (testType) {
      case 'Connectivity':
        return {
          Connectivity: {
            ...baseConfig,
            target: testConfig.target || nodeTarget,
            port: testConfig.port ? parseInt(testConfig.port) : undefined,
            protocol: 'http'
          }
        };
      case 'DirectIp':
        return {
          DirectIp: {
            ...baseConfig,
            target: node?.ip || '',
            port: parseInt(testConfig.port) || 80
          }
        };
      case 'Ping':
        return {
          Ping: {
            ...baseConfig,
            target: testConfig.target || nodeTarget,
            port: testConfig.port ? parseInt(testConfig.port) : undefined,
            attempts: parseInt(testConfig.attempts) || 4
          }
        };
      case 'WellknownIp':
        return {
          WellknownIp: {
            ...baseConfig
          }
        };
      case 'DnsResolution':
        return {
          DnsResolution: {
            ...baseConfig,
            domain: testConfig.domain || 'example.com'
          }
        };
      case 'DnsOverHttps':
        return {
          DnsOverHttps: {
            ...baseConfig,
            target: 'https://1.1.1.1/dns-query',
            domain: testConfig.domain || 'example.com',
            service_type: 'cloudflare'
          }
        };
      case 'ServiceHealth':
        return {
          ServiceHealth: {
            ...baseConfig,
            target: testConfig.target || nodeTarget,
            port: parseInt(testConfig.port) || 80,
            path: testConfig.path || '/',
            expected_status: 200
          }
        };
      case 'VpnConnectivity':
        return {
          VpnConnectivity: {
            ...baseConfig,
            target: testConfig.target || nodeTarget,
            port: parseInt(testConfig.port) || 51820
          }
        };
      case 'VpnTunnel':
        return {
          VpnTunnel: {
            ...baseConfig,
            expected_subnet: testConfig.expected_subnet || '10.100.0.0/24'
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
        monitor_interval_minutes: monitorInterval ? parseInt(monitorInterval) : null,
        enabled: enabled
      };
      
      const endpoint = isEditMode ? '/api/tests/update-assignment' : '/api/tests/assign-test';
      
      const response = await fetch(endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(requestData),
      });
      
      if (response.ok) {
        const result = await response.json();
        onAssigned(result.data.node || result.data, result.data.warning);
        resetForm();
        onClose();
      } else {
        const error = await response.json();
        alert(`Failed to ${isEditMode ? 'update' : 'assign'} test: ${error.error || 'Unknown error'}`);
      }
    } catch (error) {
      console.error(`Error ${isEditMode ? 'updating' : 'assigning'} test:`, error);
      alert(`Failed to ${isEditMode ? 'update' : 'assign'} test. Please try again.`);
    } finally {
      loading = false;
    }
  }
  
  async function handleDelete() {
    if (!node || !test) return;
    
    if (!confirm(`Are you sure you want to remove the ${getTestTypeDisplayName(test.test_type)} test from ${node.name}?`)) {
      return;
    }
    
    deleting = true;
    
    try {
      const response = await fetch('/api/tests/unassign-test', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          node_id: node.id,
          test_type: test.test_type
        }),
      });
      
      if (response.ok) {
        const result = await response.json();
        onAssigned(result.data); // Pass the updated node
        onClose();
      } else {
        const error = await response.json();
        alert(`Failed to remove test: ${error.error || 'Unknown error'}`);
      }
    } catch (error) {
      console.error('Error removing test:', error);
      alert('Failed to remove test. Please try again.');
    } finally {
      deleting = false;
    }
  }
  
  function handleClose() {
    resetForm();
    onClose();
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-gray-800 rounded-lg p-6 w-full max-w-md max-h-[90vh] overflow-y-auto">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-xl font-semibold text-white">
          {isEditMode ? `Edit ${node?.name || 'Node'} ${test?.test_type ? getTestTypeDisplayName(test.test_type) : ''} Test` : `Assign Test to '${node?.name || 'Node'}'`}
        </h2>
        <button
          on:click={handleClose}
          class="text-gray-400 hover:text-white"
        >
          <X size={24} />
        </button>
      </div>
      
      {#if loadingCompatibility}
        <div class="flex items-center justify-center py-4">
          <div class="text-gray-400">Loading compatibility info...</div>
        </div>
      {:else}
        <form on:submit|preventDefault={handleSubmit} class="space-y-4">
          <!-- Test Type Selection (disabled in edit mode) -->
          <div>
            <label for="test_type" class="block text-sm font-medium text-gray-300 mb-1">
              Test Type
            </label>
            <select
              id="test_type"
              bind:value={selectedTestType}
              disabled={isEditMode}
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-800 disabled:cursor-not-allowed"
            >
              {#each testTypes as testType}
                <option value={testType}>
                  {getTestTypeDisplayName(testType)}
                  {#if isTestAlreadyAssigned && testType === selectedTestType}
                    (Already Assigned)
                  {/if}
                </option>
              {/each}
            </select>

            <!-- Test Description -->
            <div class="mt-2 p-3 bg-gray-700/50 rounded-lg border border-gray-600">
              <p class="text-sm text-gray-300 mb-2">
                {getTestDescription(selectedTestType).shortDescription}
              </p>
              <p class="text-xs text-gray-400">
                {getTestDescription(selectedTestType).detailedDescription}
              </p>
            </div>
            
            <!-- Compatibility Status -->
            {#if !loadingCompatibility && compatibilityInfo?.data}
              {@const testInfo = compatibilityInfo.data.recommended_tests.find((t: any) => t.test_type === selectedTestType) || 
                                compatibilityInfo.data.other_tests.find((t: any) => t.test_type === selectedTestType)}
              {#if testInfo && testInfo.warning}
                <div class="mt-2 flex items-start space-x-2 p-2 bg-yellow-900/20 border border-yellow-500/30 rounded">
                  <div class="text-sm text-yellow-300">
                    {testInfo.warning}
                  </div>
                </div>
              {/if}
              
              {#if isTestAlreadyAssigned}
                <div class="mt-2 flex items-start space-x-2 p-2 bg-blue-900/20 border border-blue-500/30 rounded">
                  <AlertTriangle size={16} class="text-blue-400 mt-0.5 flex-shrink-0" />
                  <div class="text-sm text-blue-300">
                    This test is already assigned to this node.
                  </div>
                </div>
              {/if}
            {/if}
          </div>
          
          <!-- Enabled Toggle -->
          <div>
            <label class="flex items-center space-x-2">
              <input
                type="checkbox"
                bind:checked={enabled}
                class="rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
              />
              <span class="text-sm font-medium text-gray-300">Enabled</span>
            </label>
            <p class="text-xs text-gray-400 mt-1">
              Disabled tests will not run during monitoring or diagnostics
            </p>
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
            <p class="text-xs text-gray-400 mt-1">
              Determines whether test failure will mark node as Failed (Critical), Degraded (Important), or have no effect on node status (Informational)
            </p>
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
              placeholder="5"
            />
            <p class="text-xs text-gray-400 mt-1">
              Leave empty to only run during diagnostics
            </p>
          </div>
          
          <!-- Test-specific Configuration -->
          <div class="space-y-3">
            <h3 class="text-sm font-medium text-gray-300">Configuration</h3>
            
            <!-- Target field for tests that need it -->
            {#if ['Connectivity', 'Ping', 'ServiceHealth', 'VpnConnectivity'].includes(selectedTestType)}
              <div>
                <label for="target" class="block text-sm font-medium text-gray-400 mb-1">
                  Target {selectedTestType === 'DirectIp' ? '(IP Address)' : '(Host/IP)'}
                </label>
                <input
                  id="target"
                  bind:value={testConfig.target}
                  type="text"
                  class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="Enter target..."
                />
              </div>
            {/if}
            
            <!-- Port for connection-based tests -->
            {#if ['Connectivity', 'DirectIp', 'ServiceHealth', 'VpnConnectivity'].includes(selectedTestType)}
              <div>
                <label for="port" class="block text-sm font-medium text-gray-400 mb-1">
                  Port
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
            
            <!-- Path for service health -->
            {#if selectedTestType === 'ServiceHealth'}
              <div>
                <label for="path" class="block text-sm font-medium text-gray-400 mb-1">
                  Path (default: /)
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
            {#if ['DnsResolution', 'DnsOverHttps'].includes(selectedTestType)}
              <div>
                <label for="domain" class="block text-sm font-medium text-gray-400 mb-1">
                  Domain to resolve
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
            
            <!-- Expected subnet for VPN tunnel -->
            {#if selectedTestType === 'VpnTunnel'}
              <div>
                <label for="expected_subnet" class="block text-sm font-medium text-gray-400 mb-1">
                  Expected VPN Subnet
                </label>
                <input
                  id="expected_subnet"
                  bind:value={testConfig.expected_subnet}
                  type="text"
                  class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="10.100.0.0/24"
                />
              </div>
            {/if}
            
            <!-- Attempts for ping -->
            {#if selectedTestType === 'Ping'}
              <div>
                <label for="attempts" class="block text-sm font-medium text-gray-400 mb-1">
                  Ping Attempts (default: 4)
                </label>
                <input
                  id="attempts"
                  bind:value={testConfig.attempts}
                  type="number"
                  min="1"
                  max="20"
                  class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="4"
                />
              </div>
            {/if}
            
            <!-- Timeout -->
            <div>
              <label for="timeout" class="block text-sm font-medium text-gray-400 mb-1">
                Timeout (milliseconds)
              </label>
              <input
                id="timeout"
                bind:value={testConfig.timeout}
                type="number"
                min="1000"
                max="300000"
                class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="30000"
              />
            </div>

            <!-- Expected Result -->
            <div>
              <label for="expected_result" class="block text-sm font-medium text-gray-400 mb-1">
                Expected Result
              </label>
              <input
                id="expected_result"
                bind:value={testConfig.expected_result}
                type="text"
                class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="Success"
              />
              <p class="text-xs text-gray-400 mt-1">
                Used to determine if the test passed or failed
              </p>
            </div>
          </div>
          
          <!-- Action Buttons -->
          <div class="flex justify-between pt-4">
            <!-- Delete button (only in edit mode) -->
            <div>
              {#if isEditMode}
                <button
                  type="button"
                  on:click={handleDelete}
                  disabled={deleting}
                  class="flex items-center gap-2 px-4 py-2 text-red-300 hover:text-red-200 border border-red-600 rounded-md hover:border-red-500 transition-colors disabled:opacity-50"
                >
                  <Trash2 size={16} />
                  {deleting ? 'Removing...' : 'Remove Test'}
                </button>
              {/if}
            </div>
            
            <!-- Save/Cancel buttons -->
            <div class="flex space-x-3">
              <button
                type="button"
                on:click={handleClose}
                class="px-4 py-2 text-gray-300 hover:text-white border border-gray-600 rounded-md hover:border-gray-500 transition-colors"
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={loading || (!isEditMode && isTestAlreadyAssigned)}
                class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed transition-colors"
              >
                {loading ? (isEditMode ? 'Updating...' : 'Assigning...') : (isEditMode ? 'Update Test' : 'Assign Test')}
              </button>
            </div>
          </div>
        </form>
      {/if}
    </div>
  </div>
{/if}