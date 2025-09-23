<script lang="ts">
  import ListConfigEditor from '$lib/shared/components/forms/selection/ListConfigEditor.svelte';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
  import InterfaceConfigPanel from './InterfaceConfigPanel.svelte';
  import { subnets } from '$lib/features/subnets/store';
  import { type Host, type Interface } from '$lib/features/hosts/types/base';
  import { v4 as uuidv4 } from 'uuid';
	import { SubnetDisplay } from '$lib/shared/components/forms/selection/display/SubnetDisplay.svelte';
	import { InterfaceDisplay } from '$lib/shared/components/forms/selection/display/InterfaceDisplay.svelte';
	import type { FormApi, FormType } from '$lib/shared/components/forms/types';
  
  export let formApi: FormApi;
  export let formData: Host;
  
  // Computed values
  $: interfaces = formData.interfaces || [];
  
  // Helper function to find subnet by ID
  function findSubnetById(subnetId: string) {
    return $subnets.find(s => s.id === subnetId) || null;
  }
    
  // Event handlers
  function handleAddInterface(subnetId: string) {
    const subnet = findSubnetById(subnetId);
    if (!subnet) return;
    
    const newInterface: Interface = {
      id: uuidv4(),
      name: "",
      subnet_id: subnetId,
      ip_address: undefined,
      mac_address: undefined,
    };
    
    formData.interfaces = [...interfaces, newInterface];
  }
  
  function handleRemoveInterface(index: number) {
    formData.interfaces = interfaces.filter((_, i) => i !== index);
  }
</script>

<ListConfigEditor
  bind:items={formData.interfaces}
>
  <svelte:fragment slot="list" let:items let:onEdit let:highlightedIndex>
    <ListManager
      label="Interfaces"
      helpText="Configure network interfaces and addresses"
      placeholder="Select subnet to create interface with..."
      emptyMessage="No interfaces configured. Add one to get started."
      allowReorder={false}
      
      options={$subnets}
      {items}
      
      optionDisplayComponent={SubnetDisplay}
      itemDisplayComponent={InterfaceDisplay}
      
      onAdd={handleAddInterface}
      onRemove={handleRemoveInterface}
      {onEdit}
      {highlightedIndex}
    />
  </svelte:fragment>
  
  <svelte:fragment slot="config" let:selectedItem let:onChange>
    {@const subnet = selectedItem ? findSubnetById(selectedItem.subnet_id) : null}
    {#if selectedItem && subnet}
      <InterfaceConfigPanel
        {formApi}
        iface={selectedItem}
        subnet={subnet}
        onChange={(updatedInterface) => onChange(updatedInterface)}
      />
    {:else}
      <div class="flex-1 min-h-0 flex items-center justify-center text-gray-400">
        <div class="text-center">
          <div class="text-lg mb-2">No interface selected</div>
          <div class="text-sm">Select an interface from the list to configure it</div>
        </div>
      </div>
    {/if}
  </svelte:fragment>
</ListConfigEditor>