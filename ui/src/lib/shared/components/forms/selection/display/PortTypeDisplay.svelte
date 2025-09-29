<script lang="ts" context="module">
  
  export const PortTypeDisplay: EntityDisplayComponent<TypeMetadata> = {
    getId: (portType: TypeMetadata) => portType.id,
    getLabel: (portType: TypeMetadata) => portType.name,
    getDescription: (portType: TypeMetadata) => portType.description,
    getIcon: (portType: TypeMetadata) => createIconComponent(portType.icon),
    getIconColor: (portType: TypeMetadata) => entities.getColorHelper("Port").icon,
    getTags: (portType: TypeMetadata) => [
        {
            label: portType.metadata.protocol,
            color: portType.metadata.protocol == 'Tcp' ? 'blue' : 'purple'
        },
        {
            label: portType.metadata.number,
            color: 'purple'
        }
    ],
    getIsDisabled: () => false,
    getCategory: (portType: TypeMetadata) => portType.category
  };
</script>

<script lang="ts">
  import ListSelectItem from '$lib/shared/components/forms/selection/ListSelectItem.svelte';
	import { entities, type TypeMetadata } from '$lib/shared/stores/metadata';
	import { createIconComponent } from '$lib/shared/utils/styling';
	import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
  
  type $$Props = DisplayComponentProps<TypeMetadata>;

  export let item: TypeMetadata;
</script>

<ListSelectItem item={item} displayComponent={PortTypeDisplay} />