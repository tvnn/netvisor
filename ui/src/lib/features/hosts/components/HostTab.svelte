<script lang="ts">
	import HostCard from './HostCard.svelte';
	import type { Host, HostWithServicesRequest } from '../types/base';
	import TabHeader from '$lib/shared/components/layout/TabHeader.svelte';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import { getDaemons, hostDaemonMap } from '$lib/features/daemons/store';
	import type { Daemon } from '$lib/features/daemons/types/base';
	import { initiateDiscovery, sessions } from '$lib/features/discovery/store';
	import HostEditor from './HostEditModal/HostEditor.svelte';
	import HostConsolidationModal from './HostConsolidationModal.svelte';
	import { consolidateHosts, createHost, deleteHost, getHosts, hosts, updateHost } from '../store';
	import { getGroups, groups } from '$lib/features/groups/store';
	import { loadData } from '$lib/shared/utils/dataLoader';
	import { getServices, services } from '$lib/features/services/store';
	import { getSubnets } from '$lib/features/subnets/store';

	const loading = loadData([getHosts, getGroups, getServices, getSubnets, getDaemons]);

	let showHostEditor = false;
	let editingHost: Host | null = null;

	let otherHost: Host | null = null;
	let showHostConsolidationModal = false;

	$: discoveryIsRunning = $sessions.size > 0;

	$: sortedHosts = [...$hosts].sort((a, b) =>
		a.created_at.localeCompare(b.created_at, undefined, { sensitivity: 'base' })
	);

	$: hostGroups = new Map(
		$hosts.map((host) => {
			const foundGroups = $groups.filter((g) => {
				return g.service_bindings.some((b) => {
					// Use $services instead of getServiceForBinding to maintain reactivity
					let service = $services.find((s) => s.bindings.map((sb) => sb.id).includes(b));
					if (service) return host.services.includes(service.id);
					return false;
				});
			});

			return [host.id, foundGroups];
		})
	);

	function handleCreateHost() {
		editingHost = null;
		showHostEditor = true;
	}

	function handleEditHost(host: Host) {
		editingHost = host;
		showHostEditor = true;
	}

	function handleRunDiscovery(daemon: Daemon) {
		initiateDiscovery({ daemon_id: daemon.id });
	}

	function handleStartConsolidate(host: Host) {
		otherHost = host;
		showHostConsolidationModal = true;
	}

	function handleDeleteHost(host: Host) {
		if (confirm(`Are you sure you want to delete "${host.name}"?`)) {
			deleteHost(host.id);
		}
	}

	async function handleHostCreate(data: HostWithServicesRequest) {
		const result = await createHost(data);
		if (result?.success) {
			showHostEditor = false;
			editingHost = null;
		}
	}

	async function handleHostUpdate(data: HostWithServicesRequest) {
		const result = await updateHost(data);
		if (result?.success) {
			showHostEditor = false;
			editingHost = null;
		}
	}

	async function handleConsolidateHosts(destination_host_id: string, other_host_id: string) {
		const result = await consolidateHosts(destination_host_id, other_host_id);
		if (result?.success) {
			showHostConsolidationModal = false;
			otherHost = null;
		}
	}

	function handleCloseHostEditor() {
		showHostEditor = false;
		editingHost = null;
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<TabHeader
		title="Hosts"
		subtitle="Manage hosts on the network"
		buttons={[
			{
				onClick: handleCreateHost,
				cta: 'Create Host'
			}
		]}
	/>

	<!-- Loading state -->
	{#if $loading}
		<Loading />
	{:else if $hosts.length === 0}
		<!-- Empty state -->
		<EmptyState
			title="No hosts configured yet"
			subtitle=""
			onClick={handleCreateHost}
			cta="Create your first host"
		/>
	{:else}
		<!-- Hosts grid -->
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
			{#each sortedHosts as host (host.id)}
				<HostCard
					{host}
					daemon={$hostDaemonMap.get(host.id) || null}
					hostGroups={hostGroups.get(host.id)}
					{discoveryIsRunning}
					onEdit={handleEditHost}
					onDelete={handleDeleteHost}
					onDiscovery={handleRunDiscovery}
					onConsolidate={handleStartConsolidate}
				/>
			{/each}
		</div>
	{/if}
</div>

<HostEditor
	isOpen={showHostEditor}
	host={editingHost}
	onCreate={handleHostCreate}
	onUpdate={handleHostUpdate}
	onClose={handleCloseHostEditor}
/>

<HostConsolidationModal
	isOpen={showHostConsolidationModal}
	{otherHost}
	onConsolidate={handleConsolidateHosts}
	onClose={() => (showHostConsolidationModal = false)}
/>
