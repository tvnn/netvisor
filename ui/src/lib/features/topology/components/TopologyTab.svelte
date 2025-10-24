<script lang="ts">
	import TabHeader from '$lib/shared/components/layout/TabHeader.svelte';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import TopologyViewer from './TopologyViewer.svelte';
	import TopologyOptionsPanel from './TopologyOptionsPanel.svelte';
	import { loadData } from '$lib/shared/utils/dataLoader';
	import { exportToPNG, getTopology, topologyOptions } from '../store';
	import { Download, RefreshCcw } from 'lucide-svelte';
	import { getHosts } from '$lib/features/hosts/store';
	import { getServices } from '$lib/features/services/store';
	import { getSubnets } from '$lib/features/subnets/store';

	const loading = loadData([getHosts, getServices, getSubnets, getTopology]);

	// Watch for option changes and reload topology
	$: if ($topologyOptions) {
		getTopology();
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<TabHeader
		title="Topology"
		subtitle="Generate and view network topology"
		buttons={[
			{ cta: 'Export', onClick: exportToPNG, IconComponent: Download },
			{ cta: 'Reload', onClick: getTopology, IconComponent: RefreshCcw }
		]}
	/>
	{#if $loading}
		<Loading />
	{:else}
		<div class="relative">
			<TopologyOptionsPanel />
			<TopologyViewer />
		</div>
	{/if}
</div>
