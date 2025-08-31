<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import { daemons } from '../daemons/store';
  import { nodes } from '../nodes/store';
  import { 
	sessions,
  } from './store';
  import { getDaemonDiscoveryState } from '../daemons/store';
	import RichSelect from '$lib/shared/components/forms/RichSelect.svelte';
	  import { getNodeTargetString } from "../nodes/store";
	import { nodeStatuses } from '$lib/shared/stores/registry';
	import DaemonDiscoveryStatus from './DaemonDiscoveryStatus.svelte';
	import { type SelectOption } from '$lib/shared/components/forms/types';

  let selectedDaemonId: string | null = null;  
  $: discoveryData = getDaemonDiscoveryState(selectedDaemonId);
  $: selectedDaemon = $daemons.find(daemon => daemon.id == selectedDaemonId);
  $: selectedNode = $nodes.find(node => node.id == selectedDaemon?.node_id);
  $: nodeStyle = nodeStatuses.getColor(selectedNode?.status || null);


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
                options={$daemons.map((d): SelectOption => {
                    let node = $nodes.find(n => n.id === d.node_id)
                    return {
                        value: d.id,
                        label: node?.name || `Daemon ${d.id.substring(0, 8)}`,
                        description: node ? `on ${getNodeTargetString(node?.target)}` : `Daemon ${d.id.substring(0, 8)}`,
                        metadata: {
                            status: node?.status
                        }
                    }
                })}
                getOptionTag={(option) => {
                    return {
                        text: option.metadata.status,
                        bgColor: nodeStyle.bg,
                        textColor: nodeStyle.text
                    }
                }}
                onSelect={handleDaemonSelect} />
        </div>
    {/if}
    
    <div class="flex">
      {#if selectedDaemon}
        <DaemonDiscoveryStatus
          daemon={selectedDaemon}
          showName={false}
          node={selectedNode}
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