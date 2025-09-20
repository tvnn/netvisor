<script lang="ts" context="module">
  import { Network } from 'lucide-svelte';
  import type { Host } from '$lib/features/hosts/types/base';
  import { serviceTypes } from '$lib/shared/stores/registry';

  export const DaemonDisplay: EntityDisplayComponent<Daemon> = {
    getId: (daemon: Daemon) => daemon.id,
    getLabel: (daemon: Daemon) => getDaemonHost(daemon.id)?.name || "Unknown Daemon",
    getDescription: (daemon: Daemon) => getDaemonHost(daemon.id)?.description || "",
    getIcon: () => Network,
    getIconColor: () => createColorHelper("purple").icon,
    getTags: (daemon: Daemon) => [],
    getIsDisabled: () => false,
    getCategory: () => null
  };
</script>

<script lang="ts">
	import { getServicesForHost } from '$lib/features/services/store';
	import { get } from 'svelte/store';
	import type { DisplayComponentProps, EntityDisplayComponent } from '../types';
	import ListSelectItem from '../ListSelectItem.svelte';
	import { getHostTargetString } from '$lib/features/hosts/store';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { getDaemonHost } from '$lib/features/daemons/store';
	import { createColorHelper } from '$lib/shared/utils/styling';
  
  type $$Props = DisplayComponentProps<Daemon>;
  
  export let item: Daemon;
</script>

<ListSelectItem item={item} displayComponent={DaemonDisplay} />