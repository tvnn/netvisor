<script lang="ts" context="module">
  
  export const ServiceTypeDisplay: EntityDisplayComponent<TypeMetadata> = {
    getId: (serviceType: TypeMetadata) => serviceType.id,
    getLabel: (serviceType: TypeMetadata) => serviceType.name,
    getDescription: (serviceType: TypeMetadata) => serviceType.description,
    getIcon: (serviceType: TypeMetadata) => serviceDefinitions.getIconComponent(serviceType.id),
    getIconColor: (serviceType: TypeMetadata) => serviceDefinitions.getColorHelper(serviceType.id).icon,
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
	import { serviceDefinitions, type TypeMetadata } from '$lib/shared/stores/metadata';
	import { createIconComponent } from '$lib/shared/utils/styling';
	import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
  
  type $$Props = DisplayComponentProps<TypeMetadata>;

  export let item: TypeMetadata;
</script>

<ListSelectItem item={item} displayComponent={ServiceTypeDisplay} />