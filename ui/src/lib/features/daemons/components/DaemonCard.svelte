<script lang="ts">
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { getDaemonIsRunningDiscovery, getDaemonDiscoveryData } from '$lib/features/daemons/store';
	import DaemonDiscoveryStatus from '$lib/features/discovery/DaemonDiscoveryStatus.svelte';
	import { sessions } from '$lib/features/discovery/store';
	import { entities } from '$lib/shared/stores/metadata';
	import { networks } from '$lib/features/networks/store';
	import { formatTimestamp } from '$lib/shared/utils/formatting';
	import { getHostFromId } from '$lib/features/hosts/store';

	export let daemon: Daemon;
	export let onDiscovery: (daemon: Daemon) => void = () => {};
	export let discoveryIsRunning: boolean;

	$: hostStore = getHostFromId(daemon.host_id);
	$: host = $hostStore;

	$: daemonIsRunningDiscovery =
		discoveryIsRunning && getDaemonIsRunningDiscovery(daemon.id, $sessions);
	$: discoveryData = daemonIsRunningDiscovery ? getDaemonDiscoveryData(daemon.id, $sessions) : null;

	// Build card data
	$: cardData = {
		title: 'Daemon on ' + (host ? host.name : daemon.ip + ':' + daemon.port),
		iconColor: entities.getColorHelper('Daemon').icon,
		icon: entities.getIconComponent('Daemon'),
		sections: [
			{
				label: 'Network',
				value: $networks.find((n) => n.id == daemon.network_id)?.name || 'Unknown Network'
			},
			{
				label: 'Registered',
				value: formatTimestamp(daemon.registered_at)
			},
			{
				label: 'Last Seen',
				value: formatTimestamp(daemon.last_seen)
			}
		],
		lists: [],
		actions: [
			{
				label: 'Run Discovery',
				icon: entities.getIconComponent('Discovery'),
				class: daemonIsRunningDiscovery ? 'btn-icon-success' : 'btn-icon',
				onClick: !daemonIsRunningDiscovery ? () => onDiscovery(daemon) : () => {},
				animation: daemonIsRunningDiscovery ? 'animate-spin' : '',
				disabled: daemonIsRunningDiscovery
			}
		],

		// Add footer when discovery is running
		footerComponent: daemonIsRunningDiscovery && daemon ? DaemonDiscoveryStatus : null,
		footerProps:
			daemonIsRunningDiscovery && daemon
				? {
						daemon,
						discoveryData
					}
				: {}
	};
</script>

<GenericCard {...cardData} />
