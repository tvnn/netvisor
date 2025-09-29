<script lang="ts" context="module">
	import { entities } from '$lib/shared/stores/metadata';

	export const DaemonDisplay: EntityDisplayComponent<Daemon> = {
		getId: (daemon: Daemon) => daemon.id,
		getLabel: (daemon: Daemon) => getDaemonHost(daemon.id)?.name || 'Unknown Daemon',
		getDescription: (daemon: Daemon) => getDaemonHost(daemon.id)?.description || '',
		getIcon: () => entities.getIconComponent('Daemon'),
		getIconColor: () => entities.getColorHelper('Daemon').icon,
		getTags: () => [],
		getIsDisabled: () => false,
		getCategory: () => null
	};
</script>

<script lang="ts">
	import type { EntityDisplayComponent } from '../types';
	import ListSelectItem from '../ListSelectItem.svelte';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { getDaemonHost } from '$lib/features/daemons/store';

	export let item: Daemon;
</script>

<ListSelectItem {item} displayComponent={DaemonDisplay} />
