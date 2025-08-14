<script lang="ts">
  import { X } from 'lucide-svelte';
  import type { AssignedTest } from "$lib/types/nodes";
      import type { TestCriticality } from "$lib/types/nodes";
  import { 
    type TestConfiguration, 
    extractConfigFromTest
  } from "$lib/types/tests";
  import type { TestType } from "$lib/config/tests/types";
  import { getTestDisplay, getTestDescription } from "$lib/config/tests/types";
  
  export let test: AssignedTest | null = null;
  export let node: any;
  export let onCancel: () => void;
  export let onChange: (test: AssignedTest) => void;
  
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
  
  const testTypes: TestType[] = [
    'Connectivity', 'DirectIp', 'Ping', 'WellknownIp',
    'DnsResolution', 'DnsOverHttps',
    'VpnConnectivity', 'VpnTunnel',
    'ServiceHealth'
  ];
  
  const criticalityLevels: TestCriticality[] = ['Critical', 'Important', 'Informational'];
  
  $: nodeTarget = node?.ip || node?.domain || node?.name || '';
  $: isEditMode = test !== null;

  $: if (selectedTestType && onChange) {
    const updatedTest: AssignedTest = {
      test_type: selectedTestType,
      test_config: getTestConfigForType(selectedTestType),
      criticality: selectedCriticality,
      monitor_interval_minutes: monitorInterval ? parseInt(monitorInterval) : undefined,
      enabled: enabled
    };
    onChange(updatedTest);
  }
  
  // Initialize form when test changes
  $: if (test) {
    initializeFromTest(test);
  } else {
    resetToDefaults();
  }
  
  function initializeFromTest(assignedTest: AssignedTest) {
    selectedTestType = assignedTest.test_type;
    selectedCriticality = assignedTest.criticality;
    monitorInterval = assignedTest.monitor_interval_minutes?.toString() || '';
    enabled = assignedTest.enabled;
    
    // Extract config fields from the test configuration
    const config = extractConfigFromTest(assignedTest);
    testConfig = {
      ...testConfig,
      ...config
    };
  }
  
  function resetToDefaults() {
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
  
  // FIXED: Use new discriminated union structure
  function getTestConfigForType(testType: TestType): TestConfiguration {
    const baseConfig = {
      timeout: parseInt(testConfig.timeout) || 30000,
      expected_result: (testConfig.expected_result || 'Success') as 'Success' | 'Failure'
    };
    
    switch (testType) {
      case 'Connectivity':
        return {
          type: 'Connectivity',
          config: {
            target: testConfig.target || nodeTarget,
            port: testConfig.port ? parseInt(testConfig.port) : undefined,
            protocol: 'tcp' as 'tcp' | 'udp',
            ...baseConfig
          }
        };
        
      case 'DirectIp':
        return {
          type: 'DirectIp',
          config: {
            target: node?.ip || '',
            port: parseInt(testConfig.port) || 80,
            ...baseConfig
          }
        };
        
      case 'Ping':
        return {
          type: 'Ping',
          config: {
            target: testConfig.target || nodeTarget,
            attempts: parseInt(testConfig.attempts) || 4,
            ...baseConfig
          }
        };
        
      case 'WellknownIp':
        return {
          type: 'WellknownIp',
          config: {
            ...baseConfig
          }
        };
        
      case 'DnsResolution':
        return {
          type: 'DnsResolution',
          config: {
            domain: testConfig.domain || 'example.com',
            ...baseConfig
          }
        };
        
      case 'DnsOverHttps':
        return {
          type: 'DnsOverHttps',
          config: {
            target: 'https://1.1.1.1/dns-query',
            domain: testConfig.domain || 'example.com',
            service_type: 'cloudflare',
            ...baseConfig
          }
        };
        
      case 'ServiceHealth':
        return {
          type: 'ServiceHealth',
          config: {
            target: testConfig.target || nodeTarget,
            port: parseInt(testConfig.port) || 80,
            path: testConfig.path || '/',
            expected_status: 200,
            ...baseConfig
          }
        };
        
      case 'VpnConnectivity':
        return {
          type: 'VpnConnectivity',
          config: {
            target: testConfig.target || nodeTarget,
            port: parseInt(testConfig.port) || 51820,
            ...baseConfig
          }
        };
        
      case 'VpnTunnel':
        return {
          type: 'VpnTunnel',
          config: {
            expected_subnet: testConfig.expected_subnet || '10.100.0.0/24',
            ...baseConfig
          }
        };
        
      default:
        throw new Error(`Unsupported test type: ${testType}`);
    }
  }
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <h3 class="text-lg font-medium text-white">
      {isEditMode ? 'Edit Test' : 'Add Test'}
    </h3>
    <button
      type="button"
      on:click={onCancel}
      class="text-gray-400 hover:text-white"
    >
      <X size={20} />
    </button>
  </div>
  
  <!-- Test Type Selection -->
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
        <option value={testType}>
          {getTestDisplay(testType)}
        </option>
      {/each}
    </select>

    <!-- Test Description -->
    <div class="mt-2 p-3 bg-gray-700/50 rounded-lg border border-gray-600">
      <p class="text-sm text-gray-300 mb-2">
        {getTestDescription(selectedTestType).short}
      </p>
      <p class="text-xs text-gray-400">
        {getTestDescription(selectedTestType).detailed}
      </p>
    </div>
  </div>
  
  <!-- Basic Settings -->
  <div class="grid grid-cols-2 gap-4">
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
    <h4 class="text-sm font-medium text-gray-300">Configuration</h4>
    
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
          Ping Attempts
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
      <select
        id="expected_result"
        bind:value={testConfig.expected_result}
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        <option value="Success">Success</option>
        <option value="Failure">Failure</option>
      </select>
      <p class="text-xs text-gray-400 mt-1">
        Used to determine if the test passed or failed
      </p>
    </div>
  </div>
</div>