<script lang="ts">
  import type { AssignedTest, NodeFormData } from "$lib/components/nodes/types";
  import ListManager from '$lib/components/common/ListManager.svelte';
  import TestConfigPanel from './TestConfigPanel.svelte';
  import { getCriticalityColor, getCriticalityDisplay, getTestDisplay } from "$lib/api/registry";
  import { getBgColor, getTextColor } from "$lib/components/common/colors";
  
  export let tests: AssignedTest[];
  export let node: NodeFormData;
  
  // State for which test is being edited
  let editingIndex = -1;
  
  $: editingTest = editingIndex >= 0 ? tests[editingIndex] : null;
  
  function getTestDisplayName(assigned: AssignedTest): string {
    return $getTestDisplay(assigned.test.type);
  }
  
  function getTestDisplayDetails(test: AssignedTest): string {
    const details = [];
    const config = test.test.config;
    
    const importantFields = {
      domain: 'Domain',
      expected_ip: 'Expected IP', 
      expected_status_code: 'Status',
      packet_count: 'Packets',
      expected_subnet: 'Subnet',
      port: 'Port',
      protocol: 'Protocol',
      dns_resolver: 'DNS Server',
      expected_domain: 'Expected Domain',
    };
    
    Object.entries(importantFields).forEach(([fieldKey, label]) => {
      const value = config[fieldKey];
      if (value !== undefined && value !== null && value !== '') {
        let displayValue = String(value);
        if (fieldKey === 'protocol') {
          displayValue = displayValue.toUpperCase();
        }
        details.push(`${label}: ${displayValue}`);
      }
    });
    
    if (config.timeout_ms && config.timeout_ms !== 30000) {
      details.push(`Timeout: ${config.timeout_ms}ms`);
    }
    
    return details.join(' • ') || 'Default configuration';
  }
    
  function getTestDisplayBadges(test: AssignedTest) {
    return [{
      text: $getCriticalityDisplay(test.criticality),
      color: getTextColor($getCriticalityColor(test.criticality)),
      bgColor: getBgColor($getCriticalityColor(test.criticality))
    }];
  }
  
  function handleCreateTest() {
    // Create a new test with default values and add it to the list
    const newTest: AssignedTest = {
      test: {
        type: 'Connectivity', // Default test type
        config: {}
      },
      criticality: 'Important'
    };
    
    tests = [...tests, newTest];
    editingIndex = tests.length - 1; // Select the new test for editing
  }
  
  function handleEditTest(test: AssignedTest, index: number) {
    editingIndex = index;
  }
  
  function handleCloseConfig() {
    editingIndex = -1;
  }
  
  // Real-time update handler
  function handleTestUpdate(updatedTest: AssignedTest) {
    if (editingIndex >= 0) {
      tests[editingIndex] = updatedTest;
      tests = [...tests]; // Trigger reactivity
    }
  }
</script>

<!-- Side-by-side layout -->
<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
  <!-- Left side: Test list -->
  <div class="space-y-4">
    <ListManager
      label="Tests"
      helpText="Tests target this node using its configured connection method."
      bind:items={tests}
      availableOptions={[]}
      placeholder="Add Test"
      highlightedIndex={editingIndex}
      allowReorder={true}
      allowEdit={true}
      allowDirectAdd={false}
      getDisplayName={getTestDisplayName}
      getDisplayDetails={getTestDisplayDetails}
      getDisplayBadges={getTestDisplayBadges}
      onEdit={handleEditTest}
      onAdd={handleCreateTest}
      emptyMessage="No tests assigned. Click 'Add Test' to create your first test."
    />
  </div>
  
  <!-- Right side: Test configuration -->
  <div class="space-y-4">
    <TestConfigPanel
      test={editingTest}
      {node}
      onClose={handleCloseConfig}
      onChange={handleTestUpdate}
    />
  </div>
</div>