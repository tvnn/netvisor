<script lang="ts" context="module">
  
  export const ServiceTypeDisplay: EntityDisplayComponent<TypeMetadata> = {
    getId: (serviceType: TypeMetadata) => serviceType.id,
    getLabel: (serviceType: TypeMetadata) => serviceType.display_name,
    getDescription: (serviceType: TypeMetadata) => serviceType.description,
    getIcon: (serviceType: TypeMetadata) => serviceType.icon,
    getIconColor: (serviceType: TypeMetadata) => serviceType.color,
    getTags: (serviceType: TypeMetadata) => [{
      label: serviceType.category,
      color: serviceType.color
    }],
    getIsDisabled: () => false,
    getCategory: (serviceType: TypeMetadata) => serviceType.category
  };
</script>

<script lang="ts">
  import ListSelectItem from '$lib/shared/components/forms/selection/ListSelectItem.svelte';
	import type { TypeMetadata } from '$lib/shared/stores/registry';
	import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
  
  type $$Props = DisplayComponentProps<TypeMetadata>;
  
  export let item: TypeMetadata;
</script>

<ListSelectItem item={item} displayComponent={ServiceTypeDisplay} />