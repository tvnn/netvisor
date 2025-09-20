<script lang="ts">
  import { hosts } from '$lib/features/hosts/store';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
  import type { Host } from '$lib/features/hosts/types/base';
	import { HostDisplay } from '$lib/shared/components/forms/selection/display/HostDisplay.svelte';
  
  export let form: any;
  export let hostIds: string[] = [];
  
  // Get available hosts that aren't already selected
  $: availableHosts = $hosts.filter(host => !hostIds.includes(host.id));
  
  // Convert hostIds to host objects for ListManager items
  $: selectedHosts = hostIds.map(id => 
    $hosts.find(h => h.id === id)
  ).filter(Boolean) as Host[];
  
  // Event handlers
  function handleAdd(hostId: string) {
    if (!hostIds.includes(hostId)) {
      hostIds = [...hostIds, hostId];
    }
  }
  
  function handleRemove(index: number) {
    hostIds = hostIds.filter((_, i) => i !== index);
  }
  
  function handleMoveUp(fromIndex: number, toIndex: number) {
    const newHostIds = [...hostIds];
    const [movedHost] = newHostIds.splice(fromIndex, 1);
    newHostIds.splice(toIndex, 0, movedHost);
    hostIds = newHostIds;
  }
  
  function handleMoveDown(fromIndex: number, toIndex: number) {
    const newHostIds = [...hostIds];
    const [movedHost] = newHostIds.splice(fromIndex, 1);
    newHostIds.splice(toIndex, 0, movedHost);
    hostIds = newHostIds;
  }
</script>

<ListManager
  label="Hosts"
  helpText="Select hosts to include in this group. You can reorder hosts using the arrow buttons."
  placeholder="Search and select hosts..."
  emptyMessage="No hosts selected. Search and select hosts to add them to this group."
  allowReorder={true}
  allowDuplicates={false}
  
  options={availableHosts}
  items={selectedHosts}
  allowItemEdit={() => false}
  
  optionDisplayComponent={HostDisplay}
  itemDisplayComponent={HostDisplay}
  
  onAdd={handleAdd}
  onRemove={handleRemove}
  onMoveUp={handleMoveUp}
  onMoveDown={handleMoveDown}
  onEdit={() => {}}
/>