<script lang="ts" context="module">
  import { Network } from 'lucide-svelte';
  import { subnets } from '$lib/features/subnets/store';
  import { get } from 'svelte/store';
  
  // Helper function to find subnet by ID
  function findSubnetById(subnetId: string) {
    return get(subnets).find(s => s.id === subnetId) || null;
  }
  
  export const InterfaceDisplay: EntityDisplayComponent<Interface> = {
    getId: (iface: Interface) => iface.id,
    getLabel: (iface: Interface) => {
      const subnet = findSubnetById(iface.subnet_id);
      return subnet?.name || 'Unknown Subnet';
    },
    getDescription: (iface: Interface) => {
      const parts = [iface.ip_address];
      if (iface.mac_address) {
        parts.push(iface.mac_address);
      } else {
        parts.push('No MAC');
      }
      return parts.join(' • ');
    },
    getIcon: () => null,
    getIconColor: () => 'text-purple-400',
    getTags: (iface: Interface) => {
      const subnet = findSubnetById(iface.subnet_id);
      const tags = [];
      if (iface.is_primary) {
        tags.push({
          label: "Default",
          color: "green"
        });
      }
      if (subnet) {
        tags.push({
          label: subnet.cidr,
          color: "yellow"
        });
      }
      return tags;
    },
    getIsDisabled: () => false,
    getCategory: () => null
  };
</script>

<script lang="ts">
  import ListSelectItem from '$lib/shared/components/forms/selection/ListSelectItem.svelte';
  import type { Interface } from '$lib/features/hosts/types/base';
	import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
  
  type $$Props = DisplayComponentProps<Interface>;
  
  export let item: Interface;
</script>

<ListSelectItem item={item} displayComponent={InterfaceDisplay} />