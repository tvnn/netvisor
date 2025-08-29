<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import { daemons } from '../daemons/store';
  import { nodes } from '../nodes/store';
  import { 
    session_id, 
    discoveryStatus, 
    completed, 
    total, 
    discovered_count, 
    error, 
    loading,
    daemonSessions,
    discoveryActions 
  } from './store';
  import DaemonDiscoveryStatus from '../daemons/DaemonDiscoveryStatus.svelte';
	import RichSelect from '../common/RichSelect.svelte';
	import { getNodeTargetString } from '../nodes/types';
	import { getNodeStatus, getNodeStatusColor, nodeStatuses } from '$lib/api/registry';
	import { getBgColor, getTextColor } from '../common/colors';

  let selectedDaemonId: string | null = null;

  // Reactive statements to get data for selected daemon
  $: selectedDaemon = selectedDaemonId ? $daemons.find(d => d.id === selectedDaemonId) : null;
  $: selectedNode = selectedDaemon ? $nodes.find(n => n.id === selectedDaemon.node_id) : null;
  
  // Make sessionId reactive to daemonSessions store changes
  $: sessionId = selectedDaemonId ? $daemonSessions.get(selectedDaemonId) || null : null;
  
  // Only show discovery data if this daemon's session is the currently active one
  $: discoveryData = sessionId && $session_id === sessionId ? {
    phase: $discoveryStatus || 'Unknown',
    completed: $completed,
    total: $total,
    discovered_count: $discovered_count,
    error: $error
  } : null;

  // Auto-select daemon logic: prioritize daemon with active session, fallback to first daemon
  $: if (!selectedDaemonId && $daemons.length > 0) {
    // First try to find a daemon that has an active session
    const daemonWithActiveSession = $daemons.find(daemon => 
      $daemonSessions.get(daemon.id)
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
    const daemonSessionId = discoveryActions.getDaemonSessionId(daemonId);
    
    if (daemonSessionId) {
      // Switch to monitoring this daemon's session
      discoveryActions.switchToDaemonSession(daemonId);
    }
  }
</script>

<div class="flex flex-col gap-3 p-3">
     <!-- border border-gray-700 rounded-md bg-gray-800"> -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
    <div class="lg:col-span-1">
        <RichSelect
            selectedValue={selectedDaemonId}
            showDescriptionInClosedDropdown={true}
            options={$daemons.map(d => {
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
                let color = $getNodeStatusColor(option.metadata.status)
                return {
                    text: option.metadata.status,
                    bgColor: getBgColor(color),
                    textColor: getTextColor(color)
                }
            }}
            onSelect={handleDaemonSelect} />
    </div>
    
    <div class="lg:col-span-2">
      {#if selectedDaemon}
        <DaemonDiscoveryStatus
          daemon={selectedDaemon}
          showName={false}
          node={selectedNode}
          sessionId={sessionId}
          discoveryData={discoveryData}
          loading={$loading}
        />
      {:else if $daemons.length === 0}
        <div class="flex items-center justify-center p-6 border border-gray-600 rounded-md bg-gray-800">
          <p class="text-gray-400">No daemons available for discovery</p>
        </div>
      {/if}
    </div>
  </div>
</div>