<script lang="ts">
  import { daemons } from '../daemons/store';
  import { nodes } from '../nodes/store';
  import { 
	sessions,
  } from './store';
  import { getDaemonDiscoveryState } from '../daemons/store';
	import RichSelect from '$lib/shared/components/forms/RichSelect.svelte';
	  import { getNodeTargetString } from "../nodes/store";
	import DaemonDiscoveryStatus from './DaemonDiscoveryStatus.svelte';
	import { type TagProps } from '$lib/shared/components/data/types';
	import { get } from 'svelte/store';

  let selectedDaemonId: string | null = null;  
  $: discoveryData = getDaemonDiscoveryState(selectedDaemonId, get(sessions));
  $: selectedDaemon = $daemons.find(daemon => daemon.id == selectedDaemonId);

  // Auto-select daemon logic: prioritize daemon with active session, fallback to first daemon
  $: if (!selectedDaemonId && $daemons.length > 0) {
    // First try to find a daemon that has an active session
    const daemonWithActiveSession = $daemons.find(daemon => 
      $sessions.get(daemon.id)
    );
    
    if (daemonWithActiveSession) {
      selectedDaemonId = daemonWithActiveSession.id;
    } else {
      // Fallback to first daemon if no active sessions
      selectedDaemonId = $daemons[0].id;
    }
  }

  function handleDaemonSelect(daemonId: string) {
    selectedDaemonId = daemonId;
  }
</script>

<div class="flex justify-end items-center">
    {#if $daemons.length > 0 && selectedDaemonId == null}
        <div class="flex">
            <RichSelect
                selectedValue={selectedDaemonId}
                options={$daemons.map((d) => {
                    let node = $nodes.find(n => n.id === d.node_id)
                    return {
                        value: d.id,
                        label: node?.name || `Daemon ${d.id.substring(0, 8)}`,
                        description: node ? `on ${getNodeTargetString(node?.target)}` : `Daemon ${d.id.substring(0, 8)}`,
                    }
                })}
                getOptionId={(option) => option.value}
                onSelect={handleDaemonSelect} />
        </div>
    {/if}
    
    <div class="flex">
      {#if selectedDaemon}
        <DaemonDiscoveryStatus
          daemon={selectedDaemon}
          discoveryData={discoveryData}
        />
      {/if}
    </div>
    {#if $daemons.length == 0}
        <div class="flex items-center justify-center p-3 border border-gray-600 rounded-md bg-gray-800">
          <p class="text-gray-400">No daemons available for discovery.</p>
        </div>
    {/if}
</div>