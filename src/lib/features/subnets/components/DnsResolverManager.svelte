<script lang="ts">
  import { nodes } from '$lib/features/nodes/store';
  import { getNodeTargetString } from '$lib/features/nodes/store';
  import ListManager from '$lib/shared/components/forms/ListManager.svelte';
  import type { Node } from '$lib/features/nodes/types/base';
	import { services } from '$lib/shared/stores/registry';
	import type { Service } from '$lib/features/services/types/base';
  
  export let form: any;
  export let resolverIds: string[] = [];
  
  // Get nodes that can act as DNS resolvers (have DNS or AdBlock services)
  $: dnsCapableNodes = $nodes.filter((node: Node) => 
    node.services.some(service => services.getMetadata(service.type).can_be_dns_resolver)
  );
  
  // Get available nodes (not already selected)
  $: availableNodes = dnsCapableNodes.filter(node => 
    !resolverIds.includes(node.id)
  );
  
  // Convert resolver IDs to node objects for display
  $: selectedResolvers = resolverIds.map(id => 
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
    return getNodeTargetString(node.target);
  }
  
  function getOptionTags(node: Node) {
    const dnsServices: Service[] = node.services.filter(service => services.getMetadata(service.type).can_be_dns_resolver)
    
    return dnsServices.map(service => ({
      label: service.type,
      color: 'blue'
    }));
  }

  // Event handlers
  function handleAdd(nodeId: string) {
    if (!resolverIds.includes(nodeId)) {
      resolverIds = [...resolverIds, nodeId];
    }
  }
  
  function handleRemove(index: number) {
    resolverIds = resolverIds.filter((_, i) => i !== index);
  }
</script>

<ListManager
  label="DNS Resolvers"
  helpText="Select nodes that provide DNS resolution services for this subnet"
  placeholder="Select a DNS server to add..."
  emptyMessage="No DNS resolvers configured. DNS capable nodes will appear here."
  allowReorder={true}
  
  options={availableNodes}
  items={selectedResolvers}
  allowItemEdit={(item) => false}
  
  {getOptionId}
  {getOptionLabel}
  {getOptionDescription}
  getOptionIcon={() => services.getIconComponent('DNS')}
  getOptionIconColor={() => 'text-blue-400'}
  {getOptionTags}
  
  getItemId={getOptionId}
  getItemLabel={getOptionLabel}
  getItemDescription={getOptionDescription}
  getItemIcon={() => services.getIconComponent('DNS')}
  getItemIconColor={() => 'text-green-400'}
  getItemTags={getOptionTags}
  
  onAdd={handleAdd}
  onRemove={handleRemove}
  onEdit={() => {}}
/>