<script lang="ts">
	import TabHeader from '$lib/shared/components/layout/TabHeader.svelte';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import { loading } from '$lib/shared/stores/feedback';
	import TopologyViewer from './TopologyViewer.svelte';
</script>

<div class="space-y-6">
  <!-- Header -->
   <TabHeader
    title="Topology"
    subtitle="Generate and view network topology"
    buttons={[]}
     />

    <TopologyViewer />
</div>