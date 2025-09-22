<script lang="ts">
	import TabHeader from '$lib/shared/components/layout/TabHeader.svelte';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import TopologyViewer from './TopologyViewer.svelte';
	import { loadData } from '$lib/shared/utils/dataLoader';
	import { getTopology } from '../store';
	import { RefreshCcw } from 'lucide-svelte';

  const loading = loadData([getTopology]);
</script>

<div class="space-y-6">
  <!-- Header -->
   <TabHeader
    title="Topology"
    subtitle="Generate and view network topology"
    buttons={[{
      cta: "Reload",
      onClick: getTopology,
      IconComponent: RefreshCcw
    }]}
     />
    {#if $loading}
      <Loading/>  
    {:else}
      <TopologyViewer />
    {/if}
</div>