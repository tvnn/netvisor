<script lang="ts">
	import { Edit, Radar, Replace, Trash2 } from 'lucide-svelte';
	import {
		formatInterface,
		getHostFromId,
		getHostIsVirtualized,
		getHostTargetString,
		getHostVirtualizerService
	} from '../store';
	import type { Host } from '../types/base';
	import GenericCard from '$lib/shared/components/data/GenericCard.svelte';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { getDaemonIsRunningDiscovery, getDaemonDiscoveryData } from '$lib/features/daemons/store';
	import DaemonDiscoveryStatus from '$lib/features/discovery/DaemonDiscoveryStatus.svelte';
	import { sessions } from '$lib/features/discovery/store';
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
	import type { Group } from '$lib/features/groups/types/base';
	import { getServiceById, getServicesForHostReactive } from '$lib/features/services/store';

	export let host: Host;
	export let daemon: Daemon | null;
	export let hostGroups: Group[] = [];
	export let onEdit: (host: Host) => void = () => {};
	export let onDelete: (host: Host) => void = () => {};
	export let onDiscovery: (daemon: Daemon) => void = () => {};
	export let onConsolidate: (host: Host) => void = () => {};
	export let discoveryIsRunning: boolean;

	$: hostIsRunningDiscovery =
		discoveryIsRunning && daemon !== null
			? getDaemonIsRunningDiscovery(daemon.id, $sessions)
			: false;
	$: discoveryData =
		hostIsRunningDiscovery && daemon ? getDaemonDiscoveryData(daemon.id, $sessions) : null;

	$: hostServicesStore = getServicesForHostReactive(host.id);
	$: hostServices = $hostServicesStore;

	$: vms = hostServices
		.flatMap((sv) => sv.vms.map((h_id) => getHostFromId(h_id)))
		.filter((h) => h != undefined);
	$: containers = hostServices
		.flatMap((sv) => sv.containers.map((s_id) => getServiceById(s_id)))
		.filter((s) => s != undefined);
	$: containerIds = containers.map((s) => s.id);

	// Build card data
	$: cardData = {
		title: host.name,
		link: host.target.type != 'None' ? `http://${getHostTargetString(host)}` : undefined,
		iconColor: entities.getColorHelper('Host').icon,
		icon:
			serviceDefinitions.getIconComponent(hostServices[0]?.service_definition) ||
			entities.getIconComponent('Host'),
		sections: [
			...(getHostIsVirtualized(host.id)
				? [
						{
							label: 'VM Managed By',
							value: getHostVirtualizerService(host.id)?.name || 'Unknown Service'
						}
					]
				: [])
		],
		lists: [
			{
				label: 'Groups',
				items: hostGroups.map((group: Group) => ({
					id: group.id,
					label: group.name,
					color: entities.getColorHelper('Group').string
				})),
				emptyText: 'No groups assigned'
			},
			...(vms.length > 0
				? [
						{
							label: 'VMs',
							items: vms.map((h) => {
								return {
									id: h.id,
									label: h.name,
									color: entities.getColorHelper('Virtualization').string
								};
							}),
							emptyText: 'No VMs assigned'
						}
					]
				: []),
			{
				label: 'Services',
				items: hostServices
					.filter((sv) => !containerIds.includes(sv.id))
					.map((sv) => {
						return {
							id: sv.id,
							label: sv.name,
							color: entities.getColorHelper('Service').string
						};
					})
					.sort((a) => (containerIds.includes(a.id) ? 1 : -1)),
				emptyText: 'No services assigned'
			},
			...(containers.length > 0
				? [
						{
							label: 'Containers',
							items: containers
								.map((c) => {
									return {
										id: c.id,
										label: c.name,
										color: entities.getColorHelper('Virtualization').string
									};
								})
								.sort((a) => (containerIds.includes(a.id) ? 1 : -1)),
							emptyText: 'No services assigned'
						}
					]
				: []),
			{
				label: 'Interfaces',
				items: host.interfaces.map((i) => {
					return {
						id: i.id,
						label: formatInterface(i),
						color: entities.getColorHelper('Interface').string
					};
				}),
				emptyText: 'No subnets assigned'
			}
		],

		actions: [
			{
				label: 'Delete Host',
				icon: Trash2,
				class: 'btn-icon-danger',
				onClick: () => onDelete(host)
			},
			...(daemon == null
				? [
						{
							label: 'Consolidate',
							icon: Replace,
							onClick: () => onConsolidate(host)
						}
					]
				: []),
			...(daemon !== null
				? [
						{
							label: 'Run Discovery',
							icon: Radar,
							class: hostIsRunningDiscovery ? 'btn-icon-success' : 'btn-icon',
							onClick: !hostIsRunningDiscovery ? () => onDiscovery(daemon) : () => {},
							animation: hostIsRunningDiscovery ? 'animate-spin' : '',
							disabled: hostIsRunningDiscovery
						}
					]
				: []),
			{
				label: 'Edit Host',
				icon: Edit,
				onClick: () => onEdit(host)
			}
		],

		// Add footer when discovery is running
		footerComponent: hostIsRunningDiscovery && daemon ? DaemonDiscoveryStatus : null,
		footerProps:
			hostIsRunningDiscovery && daemon
				? {
						daemon,
						discoveryData
					}
				: {}
	};
</script>

<GenericCard {...cardData} />
