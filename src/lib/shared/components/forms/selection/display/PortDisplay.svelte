<script lang="ts" context="module">
  import { Network } from 'lucide-svelte';
  import PortInlineEditor from './PortInlineEditor.svelte';
  
  export const PortDisplay: EntityDisplayComponent<Port> = {
    getId: (port: Port) => `${port.number}-${port.protocol}`,
    getLabel: (port: Port) => `Port ${port.number}`,
    getDescription: (port: Port) => `${port.protocol.toUpperCase()} protocol`,
    getIcon: () => Network,
    getIconColor: () => 'text-green-400',
    getTags: (port: Port) => [{
      label: port.protocol.toUpperCase(),
      color: port.protocol === 'Tcp' ? 'blue' : 'purple'
    }],
    getIsDisabled: () => false,
    getCategory: () => null,
    supportsInlineEdit: true,
    renderInlineEdit: (port: Port, onUpdate: (updates: Partial<Port>) => void) => {
      return {
        component: PortInlineEditor,
        props: { port, onUpdate }
      };
    }
  };
</script>

<script lang="ts">
  import type { Port } from '$lib/features/services/types/base';
	import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
	import ListSelectItem from '../ListSelectItem.svelte';
  
  type $$Props = DisplayComponentProps<Port>;
  
  export let item: Port;
</script>

<ListSelectItem item={item} displayComponent={PortDisplay} />