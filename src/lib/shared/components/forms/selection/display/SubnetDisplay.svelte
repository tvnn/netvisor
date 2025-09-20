<script lang="ts" context="module">
  import { Network } from 'lucide-svelte';
  
  export const SubnetDisplay: EntityDisplayComponent<Subnet> = {
    getId: (subnet: Subnet) => subnet.id,
    getLabel: (subnet: Subnet) => subnet.name,
    getDescription: (subnet: Subnet) => subnet.cidr,
    getIcon: () => Network,
    getIconColor: () => 'text-orange-400',
    getTags: (subnet: Subnet) => [{
      label: subnet.cidr,
      color: "yellow"
    }],
    getIsDisabled: () => false,
    getCategory: () => null
  };
</script>

<script lang="ts">
  import ListSelectItem from '$lib/shared/components/forms/selection/ListSelectItem.svelte';
	import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
  
  type $$Props = DisplayComponentProps<Subnet>;
  
  export let item: Subnet;
</script>

<ListSelectItem item={item} displayComponent={SubnetDisplay} />