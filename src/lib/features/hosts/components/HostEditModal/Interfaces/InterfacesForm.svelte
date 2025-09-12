<script lang="ts">
  import ListConfigEditor from '$lib/shared/components/forms/ListConfigEditor.svelte';
  import InterfaceConfigPanel from './InterfaceConfigPanel.svelte';
  import { subnets } from '$lib/features/subnets/store';
  import { type Host, type Interface } from '$lib/features/hosts/types/base';
  import { v4 as uuidv4 } from 'uuid';
  
  export let form: any;
  export let formData: Host;
  
  // Computed values
  $: interfaces = formData.interfaces || [];
  $: availableSubnets = $subnets.filter(s => 
    !interfaces.find(iface => iface.subnet_id === s.id)
  );
  
  // Helper function to find subnet by ID
  function findSubnetById(subnetId: string): Subnet | null {
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
  
  function handleInterfaceChange(membership: Interface, index: number) {
    if (index >= 0 && index < interfaces.length) {
      const updatedInterfaces = [...interfaces];
      updatedInterfaces[index] = membership;
      formData.interfaces = updatedInterfaces;
    }
  }
  
  function handleRemoveInterface(index: number) {
    formData.interfaces = interfaces.filter((_, i) => i !== index);
  }
  
  // Display functions for options (available subnets)
  function getOptionId(subnet: Subnet): string {
    return subnet.id;
  }
  
  function getOptionLabel(subnet: Subnet): string {
    return subnet.name;
  }
  
  function getOptionDescription(subnet: Subnet): string {
    return subnet.cidr;
  }
  
  function getOptionTags(subnet: Subnet) {
    // return [
    //   {
    //     label: subnet.cidr,
    //     color: "yellow"
    //   }
    // ];
    return []
  }
  
  // Display functions for items (current interfaces)
  function getItemId(iface: Interface): string {
    return iface.subnet_id;
  }
  
  function getItemLabel(iface: Interface): string {
    const subnet = findSubnetById(iface.subnet_id);
    return subnet?.name || 'Unknown Subnet';
  }
  
  function getItemDescription(membership: Interface): string {
    const parts = [membership.ip_address];
    if (membership.mac_address) {
      parts.push(membership.mac_address);
    } else {
      parts.push('No MAC');
    }
    return parts.join(' • ');
  }
  
  function getItemTags(membership: Interface) {
    const subnet = findSubnetById(membership.subnet_id);
    const tags = [];
    if (membership.is_primary) {
      tags.push({
        label: "Default",
        color: "green"
      })
    }
    if (subnet) {
      tags.push({
        label: subnet.cidr,
        color: "yellow"
      })
    }
    return tags;
  }
</script>

<ListConfigEditor
  {form}
  bind:items={formData.interfaces}
  options={availableSubnets}
  label="Interfaces"
  helpText="Configure network interfaces and addresses"
  emptyMessage="No interfaces configured. Add one to get started."
  
  allowReorder={false}
  placeholder="Select subnet to create interface with..."

  {getOptionId}
  {getOptionLabel}
  {getOptionDescription}
  getOptionIcon={() => null}
  getOptionIconColor={() => ''}
  {getOptionTags}
  
  {getItemId}
  {getItemLabel}
  {getItemDescription}
  getItemIcon={() => null}
  getItemIconColor={() => ''}
  {getItemTags}
  
  onAdd={handleAddInterface}
  onRemove={handleRemoveInterface}
  onChange={handleInterfaceChange}
>
  <InterfaceConfigPanel
  slot="config"
  let:selectedItem
  let:selectedIndex
  let:onChange
  
  {form}
  iface={selectedItem}
  subnet={selectedItem ? (() => findSubnetById(selectedItem.subnet_id))() : null}
  onChange={(updatedMembership) => onChange(updatedMembership)}
/>
</ListConfigEditor>