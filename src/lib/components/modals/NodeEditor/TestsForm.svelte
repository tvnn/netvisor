<script lang="ts">
  import type { AssignedTest } from "$lib/types/nodes";
  import { getTestDisplay } from "$lib/config/tests/types";
  import { getCriticalityDisplay, getCriticalityColor } from "$lib/config/nodes/criticality";
  import { getTestTarget } from "$lib/types/tests";
  import ListManager from '$lib/components/common/ListManager.svelte'
  
  export let tests: AssignedTest[];
  export let editingIndex: number = -1;
  export let onEditTest: (test: AssignedTest, index: number) => void;
  export let onCreateTest: () => void;
  
  function getTestDisplayName(test: AssignedTest): string {
    return getTestDisplay(test.test_type);
  }
  
  function getTestDisplayDetails(test: AssignedTest): string {
    const target = getTestTarget(test.test_config);
    const interval = test.monitor_interval_minutes ? `${test.monitor_interval_minutes}m` : 'Manual';
    
    const details = [];
    if (target) {
      details.push(`Target: ${target}`);
    }
    details.push(`Interval: ${interval}`);
    
    return details.join(' • ');
  }
  
  function getTestDisplayBadges(test: AssignedTest) {
    const badges = [];
    
    // Criticality badge
    badges.push({
      text: getCriticalityDisplay(test.criticality),
      color: getCriticalityColor(test.criticality),
      bgColor: 'bg-gray-800'
    });
    
    // Disabled badge
    if (!test.enabled) {
      badges.push({
        text: 'Disabled',
        color: 'text-gray-400',
        bgColor: 'bg-gray-800'
      });
    }
    
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
  helpText="Tests will be executed in the order shown. Use the arrow buttons to reorder."
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
  emptyMessage="No tests assigned"
/>