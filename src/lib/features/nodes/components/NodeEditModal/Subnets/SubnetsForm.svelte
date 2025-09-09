<script lang="ts">
  import ListConfigEditor from '$lib/shared/components/forms/ListConfigEditor.svelte';
  import SubnetConfigPanel from './SubnetConfigPanel.svelte';
  import { subnets } from '$lib/features/subnets/store';
  import { type Node } from '$lib/features/nodes/types/base';
  
  export let form: any;
  export let formData: Node;
  
  // Computed values
  $: nodeSubnetMemberships = formData.subnets || [];
  $: availableSubnets = $subnets.filter(s => 
    !nodeSubnetMemberships.find(membership => membership.subnet_id === s.id)
  );
  
  // Helper function to find subnet by ID
  function findSubnetById(subnetId: string): Subnet | null {
    return $subnets.find(s => s.id === subnetId) || null;
  }
    
  // Event handlers
  function handleAddSubnet(subnetId: string) {
    const subnet = findSubnetById(subnetId);
    if (!subnet) return;
    
    const newMembership: NodeSubnetMembership = {
      subnet_id: subnetId,
      ip_address: undefined,
      mac_address: undefined,
      default: false
    };
    
    formData.subnets = [...nodeSubnetMemberships, newMembership];
  }
  
  function handleSubnetChange(membership: NodeSubnetMembership, index: number) {
    if (index >= 0 && index < nodeSubnetMemberships.length) {
      const updatedMemberships = [...nodeSubnetMemberships];
      updatedMemberships[index] = membership;
      formData.subnets = updatedMemberships;
    }
  }
  
  function handleRemoveSubnet(index: number) {
    formData.subnets = nodeSubnetMemberships.filter((_, i) => i !== index);
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
  
  // Display functions for items (current memberships)
  function getItemId(membership: NodeSubnetMembership): string {
    return membership.subnet_id;
  }
  
  function getItemLabel(membership: NodeSubnetMembership): string {
    const subnet = findSubnetById(membership.subnet_id);
    return subnet?.name || 'Unknown Subnet';
  }
  
  function getItemDescription(membership: NodeSubnetMembership): string {
    const parts = [membership.ip_address];
    if (membership.mac_address) {
      parts.push(membership.mac_address);
    } else {
      parts.push('No MAC');
    }
    return parts.join(' • ');
  }
  
  function getItemTags(membership: NodeSubnetMembership) {
    const subnet = findSubnetById(membership.subnet_id);
    const tags = [];
    if (membership.default) {
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
  bind:items={formData.subnets}
  options={availableSubnets}
  label="Subnets"
  helpText="Configure network memberships and addresses"
  emptyMessage="No subnets configured. Add one to get started."
  
  allowReorder={false}
  placeholder="Select subnet to add..."

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
  
  onAdd={handleAddSubnet}
  onRemove={handleRemoveSubnet}
  onChange={handleSubnetChange}
>
  <SubnetConfigPanel
  slot="config"
  let:selectedItem
  let:selectedIndex
  let:onChange
  
  {form}
  membership={selectedItem}
  subnet={selectedItem ? (() => findSubnetById(selectedItem.subnet_id))() : null}
  onChange={(updatedMembership) => onChange(updatedMembership)}
/>
</ListConfigEditor>