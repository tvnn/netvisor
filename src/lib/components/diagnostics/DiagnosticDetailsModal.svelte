<script lang="ts">
  import { 
    CheckCircle, 
    XCircle, 
    Pause, 
    Loader2,
    AlertTriangle,
    Clock,
    Server,

	CircleX

  } from 'lucide-svelte';
  import EditModal from '../common/EditModal.svelte';
  import type { DiagnosticExecution } from './types';
  import type { TestResult } from '../tests/types';
  import { nodeGroups } from '../node_groups/store';
	import { formatDuration, formatTimestamp } from './store';
	import { getDiagnosticStatusColor, getDiagnosticStatusIcon } from '$lib/api/registry';
	import type { NodeResult } from '../nodes/types';
	import { getBgColor, getTextColor } from '../common/colors';
	import { getIcon } from '../common/icons';
	import { nodes } from '../nodes/store';
	import Tag from '../common/Tag.svelte';
	import JsonContainer from '../common/JsonContainer.svelte';

  export let execution: DiagnosticExecution | null = null;
  export let isOpen = false;
  export let onClose: () => void;

  // Get group name from group ID using nodeGroups store
  function getGroupName(groupId: string): string {
    const group = $nodeGroups.find(g => g.id === groupId);
    return group?.name || groupId;
  }

  $: title = execution ? `Diagnostic Details - ${getGroupName(execution.group_id)}` : 'Diagnostic Details';
  $: sortedNodes = execution?.node_results ? sortNodesBySequence(execution.node_results) : [];

  function sortNodesBySequence(nodes: NodeResult[]) {
      return [...nodes].sort((a, b) => {
          if (a.executed_at !== undefined && b.executed_at !== undefined) {
              return new Date(a.executed_at).getTime() - new Date(b.executed_at).getTime();
          }
          if (a.executed_at === undefined) return 1;
          if (b.executed_at === undefined) return -1;
          return 0
      });
  }

  function countNodeTests(nodeResult: NodeResult) {
    if (!nodeResult.test_results) return { passed: 0, total: 0 };
    
    const total = nodeResult.test_results.length;
    const passed = nodeResult.test_results.filter(test => test.success).length;
    
    return { passed, total };
  }

  function handleSubmit() {}

  function handleCancel() {
    onClose();
  }
</script>

<EditModal
  {isOpen}
  {title}
  loading={false}
  submitLabel="Close"
  cancelLabel=""
  showCancel={false}
  {onClose}
  onCancel={handleCancel}
  onSubmit={handleSubmit}
>
  {#if execution}
    <div class="space-y-6">
      <!-- Execution Summary -->
      <div class="bg-gray-700/30 rounded-lg p-6 border border-gray-600">
        <h3 class="text-lg font-semibold text-white mb-4">Execution Summary</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <div class="space-y-3 text-sm">
              <div class="flex justify-between">
                <span class="text-gray-400">Status:</span>
                <div class="flex items-center gap-2">
                  <Tag
                    bgColor={getBgColor($getDiagnosticStatusColor(execution.status))}
                    textColor={getTextColor($getDiagnosticStatusColor(execution.status))}
                    label={execution.status} />
                </div>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-400">Group:</span>
                <span class="text-white">{getGroupName(execution.group_id)}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-400">Trigger Reason:</span>
                <span class="text-white">{execution.trigger_reason}</span>
              </div>
            </div>
          </div>
          <div class="space-y-3 text-sm">
            <div class="flex justify-between">
                <span class="text-gray-400">Started:</span>
                <span class="text-white">{formatTimestamp(execution.started_at)}</span>
              </div>
              {#if execution.completed_at}
                <div class="flex justify-between">
                  <span class="text-gray-400">Completed:</span>
                  <span class="text-white">{formatTimestamp(execution.completed_at)}</span>
                </div>
              {/if}
              <div class="flex justify-between">
                <span class="text-gray-400">Duration:</span>
                <span class="text-white">{formatDuration(execution.started_at, execution?.completed_at || '')}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-400">Execution ID:</span>
                <span class="text-white">{execution.id}</span>
              </div>
          </div>
        </div>
      </div>

      <!-- Node Results -->
      <div>
        <h3 class="text-lg font-semibold text-white mb-4 flex items-center gap-2">
          <Server size={20} />
          Node Results ({sortedNodes.length})
        </h3>
        
        {#if sortedNodes.length === 0}
          <div class="text-center py-8 bg-gray-800 border border-gray-700 rounded-lg">
            <p class="text-gray-400">No node results available</p>
          </div>
        {:else}
          <div class="space-y-3">
            {#each sortedNodes as nodeResult (nodeResult.node_id || nodeResult.executed_at)}
              {#if nodeResult}
                {#each [nodeResult.node_status] as nodeStatus}
                  {#each [countNodeTests(nodeResult)] as testCounts}
                      
                      <div class="border rounded-lg overflow-hidden">
                        <!-- Node Header -->
                        <div class="px-4 py-3 border-b border-gray-700 flex items-center gap-3">
                          <!-- <svelte:component 
                            this={getStatusIcon(nodeStatus)} 
                            size={16} 
                            class="{getStatusClass(nodeStatus)} flex-shrink-0"
                          /> -->
                          <div class="flex-1">
                            <div class="flex items-center gap-2">
                              <span class="font-semibold text-white">
                                {$nodes.filter(n => n.id = nodeResult.node_id)[0].name}
                              </span>
                            </div>
                            <div class="text-sm text-gray-400 mt-1">
                              {testCounts.passed}/{testCounts.total} checks passed
                            </div>
                          </div>
                          <span class="text-xs text-gray-400">
                            Executed at {new Date(nodeResult.executed_at).toLocaleTimeString()} for {nodeResult.duration_ms} ms
                          </span>
                        </div>

                        <!-- Test Results -->
                        {#if nodeResult.test_results && nodeResult.test_results.length > 0}
                          <div class="px-4 py-3 space-y-2">
                            {#each nodeResult.test_results as test}
                              <div class="flex items-start gap-3 text-sm">
                                <svelte:component 
                                  this={test.success ? CheckCircle : XCircle} 
                                  size={14} 
                                  class="flex-shrink-0 mt-0.5"
                                />
                                <div class="flex-1 min-w-0">
                                  <div class="font-medium {test.success ? 'text-gray-300' : 'text-red-300'}">
                                    {test.message}
                                    {#if test.duration_ms}
                                      • {test.duration_ms}ms
                                    {/if}
                                  </div>
                                  <JsonContainer data={test.details} title="Test Details"/>
                                </div>
                              </div>
                            {/each}
                          </div>
                        {:else if nodeStatus === 'Running'}
                          <div class="px-4 py-3">
                            <div class="flex items-center gap-2 text-blue-400">
                              <Loader2 size={14} class="animate-spin" />
                              <span class="text-sm">Running checks...</span>
                            </div>
                          </div>
                        {:else if nodeStatus !== 'Skipped'}
                          <div class="px-4 py-3">
                            <span class="text-gray-500 text-sm">No test results available</span>
                          </div>
                        {/if}
                      </div>

                  {/each}
                {/each}
              {/if}
            {/each}
          </div>
        {/if}
      </div>
    </div>
  {:else}
    <div class="text-center py-8">
      <p class="text-gray-400">No execution data available</p>
    </div>
  {/if}
</EditModal>