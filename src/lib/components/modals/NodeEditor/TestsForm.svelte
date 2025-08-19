<script lang="ts">
  import type { AssignedTest } from "$lib/types/nodes";
  import { getTestDisplay } from "$lib/config/tests/types";
  import { getCriticalityDisplay, getCriticalityColor } from "$lib/config/nodes/criticality";
  import ListManager from '$lib/components/common/ListManager.svelte'
  
  export let tests: AssignedTest[];
  export let editingIndex: number = -1;
  export let onEditTest: (test: AssignedTest, index: number) => void;
  export let onCreateTest: () => void;
  
  function getTestDisplayName(test: AssignedTest): string {
    return getTestDisplay(test.test.type);
  }
  
  function getTestDisplayDetails(test: AssignedTest): string {
    const details = [];
    
    // Add test-specific details based on test type
    switch (test.test.type) {
      case 'DnsResolution':
        if (test.test.config.domain) {
          details.push(`Domain: ${test.test.config.domain}`);
        }
        break;
      case 'DnsLookup':
        if (test.test.config.expected_ip) {
          details.push(`Expected IP: ${test.test.config.expected_ip}`);
        }
        break;
      case 'ServiceHealth':
        details.push(`Status: ${test.test.config.expected_status_code}`);
        break;
      case 'Ping':
        if (test.test.config.packet_count) {
          details.push(`Packets: ${test.test.config.packet_count}`);
        }
        break;
      case 'VpnTunnel':
        if (test.test.config.expected_subnet) {
          details.push(`Subnet: ${test.test.config.expected_subnet}`);
        }
        break;
    }
    
    // Add timeout if not default
    if (test.test.config.timeout_ms && test.test.config.timeout_ms !== 30000) {
      details.push(`Timeout: ${test.test.config.timeout_ms}ms`);
    }
    
    return details.join(' • ') || 'Default configuration';
  }
  
  function getTestDisplayBadges(test: AssignedTest) {
    const badges = [];
    
    // Criticality badge
    badges.push({
      text: getCriticalityDisplay(test.criticality),
      color: getCriticalityColor(test.criticality),
      bgColor: 'bg-gray-800'
    });
    
    return badges;
  }
  
  function handleEdit(test: AssignedTest, index: number) {
    // Create a deep copy to avoid reference issues
    const testCopy = JSON.parse(JSON.stringify(test));
    onEditTest(testCopy, index);
  }
  
  function handleAdd() {
    onCreateTest();
  }
</script>

<ListManager
  label="Tests"
  helpText="Tests target this node using its configured connection method. Configure what/how to test, not where to test."
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
  onEdit={handleEdit}
  onAdd={handleAdd}
  emptyMessage="No tests assigned. Tests will target this node using its configured connection method."
/>