<script lang="ts">
	import { createSubnet, deleteSubnet, getSubnets, subnets, updateSubnet } from '../store';
	import SubnetCard from './SubnetCard.svelte';
	import SubnetEditModal from './SubnetEditModal/SubnetEditModal.svelte';
	import TabHeader from '$lib/shared/components/layout/TabHeader.svelte';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import { loadData } from '$lib/shared/utils/dataLoader';
	import { getHosts } from '$lib/features/hosts/store';
	import { getServices } from '$lib/features/services/store';
	import type { Subnet } from '../types/base';

	let showSubnetEditor = false;
	let editingSubnet: Subnet | null = null;

	const loading = loadData([getSubnets, getHosts, getServices]);

	$: sortedSubnets = [...$subnets].sort((a, b) =>
		a.created_at.localeCompare(b.created_at, undefined, { sensitivity: 'base' })
	);

	function handleCreateSubnet() {
		editingSubnet = null;
		showSubnetEditor = true;
	}

	function handleEditSubnet(subnet: Subnet) {
		editingSubnet = subnet;
		showSubnetEditor = true;
	}

	function handleDeleteSubnet(subnet: Subnet) {
		if (confirm(`Are you sure you want to delete "${subnet.name}"?`)) {
			deleteSubnet(subnet.id);
		}
	}

	async function handleSubnetCreate(data: Subnet) {
		const result = await createSubnet(data);
		if (result?.success) {
			showSubnetEditor = false;
			editingSubnet = null;
		}
	}

	async function handleSubnetUpdate(_id: string, data: Subnet) {
		const result = await updateSubnet(data);
		if (result?.success) {
			showSubnetEditor = false;
			editingSubnet = null;
		}
	}

	function handleCloseSubnetEditor() {
		showSubnetEditor = false;
		editingSubnet = null;
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<TabHeader
		title="Subnets"
		subtitle="Manage network subnets and IP ranges"
		buttons={[
			{
				onClick: handleCreateSubnet,
				cta: 'Create Subnet'
			}
		]}
	/>

	<!-- Loading state -->
	{#if $loading}
		<Loading />
	{:else if $subnets.length === 0}
		<!-- Empty state -->
		<EmptyState
			title="No subnets configured yet"
			subtitle=""
			onClick={handleCreateSubnet}
			cta="Create your first subnet"
		/>
	{:else}
		<!-- Subnets grid -->
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
			{#each sortedSubnets as subnet (subnet.id)}
				<SubnetCard {subnet} onEdit={handleEditSubnet} onDelete={handleDeleteSubnet} />
			{/each}
		</div>
	{/if}
</div>

<SubnetEditModal
	isOpen={showSubnetEditor}
	subnet={editingSubnet}
	onCreate={handleSubnetCreate}
	onUpdate={handleSubnetUpdate}
	onClose={handleCloseSubnetEditor}
/>
