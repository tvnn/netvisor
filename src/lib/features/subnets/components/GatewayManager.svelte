<script lang="ts">
  import { Router } from 'lucide-svelte';
  import { nodes } from '$lib/features/nodes/store';
  import { getNodeTargetString } from '$lib/features/nodes/store';
  import ListManager from '$lib/shared/components/forms/ListManager.svelte';
  import type { Node } from '$lib/features/nodes/types/base';
	import { services } from '$lib/shared/stores/registry';
	import type { TagProps } from '$lib/shared/components/data/types';
  
  export let form: any;
  export let gatewayIds: string[] = [];
  
  // Get nodes that can act as gateways (have Router services or gateway-like IPs)
  $: gatewayCapableNodes = $nodes.filter((node: Node) => 
    node.services.some(service => services.getMetadata(service.type).can_be_gateway));
  
  // Get available nodes (not already selected)
  $: availableNodes = gatewayCapableNodes.filter(node => 
    !gatewayIds.includes(node.id)
  );
  
  // Convert gateway IDs to node objects for display
  $: selectedGateways = gatewayIds.map(id => 
    $nodes.find(node => node.id === id)
  ).filter(Boolean) as Node[];
  
  // Display functions for available nodes (dropdown options)
  function getOptionId(node: Node): string {
    return node.id;
  }
  
  function getOptionLabel(node: Node): string {
    return node.name;
  }
  
  function getOptionDescription(node: Node): string {
    return getNodeTargetString(node.target)
  }
  
  function getOptionTags(node: Node) {
    const tags: TagProps[] = [];
    
    const gatewayServices = node.services.filter(service => services.getMetadata(service.type).can_be_gateway);
    
    gatewayServices.forEach(service => {
      tags.push({
        label: service.type,
        color: 'blue'
      });
    });
    
    return tags;
  }
  
  // Event handlers
  function handleAdd(nodeId: string) {
    if (!gatewayIds.includes(nodeId)) {
      gatewayIds = [...gatewayIds, nodeId];
    }
  }
  
  function handleRemove(index: number) {
    gatewayIds = gatewayIds.filter((_, i) => i !== index);
  }
</script>

<ListManager
  label="Gateways"
  helpText="Select nodes that provide gateway/routing services for this subnet"
  placeholder="Select a gateway to add..."
  emptyMessage="No gateways configured. Router-capable nodes will appear here."
  allowReorder={true}
  
  options={availableNodes}
  items={selectedGateways}
  allowItemEdit={(item) => false}
  
  {getOptionId}
  {getOptionLabel}
  {getOptionDescription}
  getOptionIcon={() => Router}
  getOptionIconColor={() => 'text-orange-400'}
  {getOptionTags}
  
  getItemId={getOptionId}
  getItemLabel={getOptionLabel}
  getItemDescription={getOptionDescription}
  getItemIcon={() => Router}
  getItemIconColor={() => 'text-green-400'}
  getItemTags={getOptionTags}
  
  onAdd={handleAdd}
  onRemove={handleRemove}
  onEdit={() => {}}
/>