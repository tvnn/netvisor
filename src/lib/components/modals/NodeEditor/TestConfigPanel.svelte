<script lang="ts">
  import { X } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import type { AssignedTest, TestCriticality, NodeFormData } from "$lib/types/nodes";
  import type { Test } from "$lib/types/tests";
  import type { TestType } from "$lib/types/tests";
  import { getTestDisplay, getTestDescription, getTestTypes } from "$lib/config/tests/types";
  import { getNodeTargetString } from "$lib/config/nodes/targets";
  
  export let test: AssignedTest | null = null;
  export let node: NodeFormData;
  export let onCancel: () => void;
  export let onChange: (test: AssignedTest) => void;

  let mounted = false;

  onMount(() => {
    mounted = true;
  });
  
  let selectedTestType: TestType = 'Connectivity';
  let selectedCriticality: TestCriticality = 'Important';
  
  const testTypes: TestType[] = getTestTypes();
  const criticalityLevels: TestCriticality[] = ['Critical', 'Important', 'Informational'];

  // Initialize form when test changes
  $: if (test) {
    initializeFromTest(test);
  } else {
    resetToDefaults();
  }
  
  $: nodeTargetString = getNodeTargetString(node.target);
  $: isEditMode = test !== null;

  $: if (mounted && selectedTestType && onChange) {
    const updatedTest: AssignedTest = {
      test: createTestFromType(selectedTestType),
      criticality: selectedCriticality
    };
    onChange(updatedTest);
  }
  
  function createTestFromType(testType: TestType): Test {
    switch (testType) {
      case 'Connectivity':
        return { type: 'Connectivity', config: { timeout_ms: 30000 } };
      case 'DirectIp':
        return { type: 'DirectIp', config: { timeout_ms: 30000 } };
      case 'Ping':
        return { type: 'Ping', config: { packet_count: 4, timeout_ms: 5000 } };
      case 'ServiceHealth':
        return { type: 'ServiceHealth', config: { expected_status_code: 200, timeout_ms: 30000 } };
      case 'DnsResolution':
        return { type: 'DnsResolution', config: { domain: 'google.com', expected_ip: '8.8.8.8', timeout_ms: 5000 } };
      case 'DnsLookup':
        return { type: 'DnsLookup', config: { expected_ip: '192.168.1.1', timeout_ms: 5000 } };
      case 'DnsOverHttps':
        return { type: 'DnsOverHttps', config: { domain: 'google.com', expected_ip: '8.8.8.8', timeout_ms: 5000 } };
      case 'ReverseDns':
        return { type: 'ReverseDns', config: { expected_domain: 'example.com', timeout_ms: 5000 } };
      case 'VpnConnectivity':
        return { type: 'VpnConnectivity', config: { timeout_ms: 30000 } };
      case 'VpnTunnel':
        return { type: 'VpnTunnel', config: { expected_subnet: '10.100.0.0/24', timeout_ms: 30000 } };
      default:
        return { type: 'Connectivity', config: { timeout_ms: 30000 } };
    }
  }
  
  function initializeFromTest(assignedTest: AssignedTest) {
    selectedTestType = assignedTest.test.type as TestType;
    selectedCriticality = assignedTest.criticality;
  }
  
  function resetToDefaults() {
    selectedTestType = 'Connectivity';
    selectedCriticality = 'Important';
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
  
  <!-- Node Target Information -->
  <div class="p-3 bg-gray-700/50 rounded-lg border border-gray-600">
    <h4 class="text-sm font-medium text-gray-300 mb-1">Test Target</h4>
    <p class="text-sm text-gray-400">
      This test will target: <span class="text-white font-mono">{nodeTargetString}</span>
    </p>
    <p class="text-xs text-gray-500 mt-1">
      Tests target the node they're assigned to. Test parameters specify what/how to test, never where to test.
    </p>
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
  
  <!-- Criticality Selection -->
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
      Critical tests affect node status when they fail. Important tests show as degraded. Informational tests don't affect status.
    </p>
  </div>

  <!-- Test-specific Configuration Based on Selected Type -->
  {#if selectedTestType === 'DnsResolution'}
    <div class="space-y-3">
      <h4 class="text-sm font-medium text-gray-300">DNS Resolution Configuration</h4>
      <div>
        <label for="dns_domain" class="block text-sm font-medium text-gray-400 mb-1">
          Domain to Resolve
        </label>
        <input
          id="dns_domain"
          type="text"
          placeholder="google.com"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <div>
        <label for="expected_ip" class="block text-sm font-medium text-gray-400 mb-1">
          Expected IP Address (optional)
        </label>
        <input
          id="expected_ip"
          type="text"
          placeholder="8.8.8.8"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
    </div>
    
  {:else if selectedTestType === 'DnsLookup'}
    <div class="space-y-3">
      <h4 class="text-sm font-medium text-gray-300">DNS Lookup Configuration</h4>
      <div>
        <label for="lookup_expected_ip" class="block text-sm font-medium text-gray-400 mb-1">
          Expected IP Address
        </label>
        <input
          id="lookup_expected_ip"
          type="text"
          placeholder="192.168.1.100"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <p class="text-xs text-gray-400 mt-1">
          IP address this node's domain should resolve to
        </p>
      </div>
    </div>
    
  {:else if selectedTestType === 'ServiceHealth'}
    <div class="space-y-3">
      <h4 class="text-sm font-medium text-gray-300">Service Health Configuration</h4>
      <div>
        <label for="expected_status" class="block text-sm font-medium text-gray-400 mb-1">
          Expected Status Code
        </label>
        <input
          id="expected_status"
          type="number"
          min="100"
          max="599"
          placeholder="200"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
    </div>
    
  {:else if selectedTestType === 'Ping'}
    <div class="space-y-3">
      <h4 class="text-sm font-medium text-gray-300">Ping Configuration</h4>
      <div>
        <label for="packet_count" class="block text-sm font-medium text-gray-400 mb-1">
          Packet Count
        </label>
        <input
          id="packet_count"
          type="number"
          min="1"
          max="10"
          placeholder="4"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
    </div>
    
  {:else if selectedTestType === 'VpnTunnel'}
    <div class="space-y-3">
      <h4 class="text-sm font-medium text-gray-300">VPN Tunnel Configuration</h4>
      <div>
        <label for="expected_subnet" class="block text-sm font-medium text-gray-400 mb-1">
          Expected VPN Subnet
        </label>
        <input
          id="expected_subnet"
          type="text"
          placeholder="10.100.0.0/24"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
    </div>
  {/if}

  <!-- Common timeout configuration for all tests -->
  <div>
    <label for="timeout" class="block text-sm font-medium text-gray-400 mb-1">
      Timeout (milliseconds)
    </label>
    <input
      id="timeout"
      type="number"
      min="1000"
      max="300000"
      placeholder="30000"
      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
    <p class="text-xs text-gray-400 mt-1">
      How long to wait for the test to complete (1-300 seconds)
    </p>
  </div>
</div>