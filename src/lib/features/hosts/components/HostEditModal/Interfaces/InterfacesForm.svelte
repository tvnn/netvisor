<script lang="ts">
  import ListConfigEditor from '$lib/shared/components/forms/selection/ListConfigEditor.svelte';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
  import InterfaceConfigPanel from './InterfaceConfigPanel.svelte';
  import { subnets } from '$lib/features/subnets/store';
  import { type Host, type Interface } from '$lib/features/hosts/types/base';
  import { v4 as uuidv4 } from 'uuid';
	import { SubnetDisplay } from '$lib/shared/components/forms/selection/display/SubnetDisplay.svelte';
	import { InterfaceDisplay } from '$lib/shared/components/forms/selection/display/InterfaceDisplay.svelte';
  
  export let form: any;
  export let formData: Host;
  
  // Computed values
  $: interfaces = formData.interfaces || [];
  $: availableSubnets = $subnets.filter(s => 
    !interfaces.find(iface => iface.subnet_id === s.id)
  );
  
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
      is_primary: false
    };
    
    formData.interfaces = [...interfaces, newInterface];
  }
  
  function handleInterfaceChange(iface: Interface, index: number) {
    if (index >= 0 && index < interfaces.length) {
      const updatedInterfaces = [...interfaces];
      updatedInterfaces[index] = iface;
      formData.interfaces = updatedInterfaces;
    }
  }
  
  function handleRemoveInterface(index: number) {
    formData.interfaces = interfaces.filter((_, i) => i !== index);
  }
</script>

<ListConfigEditor
  {form}
  bind:items={formData.interfaces}
>
  <svelte:fragment slot="list" let:items let:onEdit let:highlightedIndex>
    <ListManager
      label="Interfaces"
      helpText="Configure network interfaces and addresses"
      placeholder="Select subnet to create interface with..."
      emptyMessage="No interfaces configured. Add one to get started."
      allowReorder={false}
      
      options={availableSubnets}
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
    {#if selectedItem}
      <InterfaceConfigPanel
        {form}
        iface={selectedItem}
        subnet={findSubnetById(selectedItem.subnet_id)}
        onChange={(updatedInterface) => onChange(updatedInterface)}
      />
    {/if}
  </svelte:fragment>
</ListConfigEditor>