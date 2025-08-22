<script lang="ts">
  import { 
    CheckCircle, 
    XCircle, 
    Pause, 
    Loader2,
    AlertTriangle,
    Clock,
    Server
  } from 'lucide-svelte';
  import EditModal from '../common/EditModal.svelte';
  import type { DiagnosticExecution, NodeResult } from './types';
  import type { TestResult } from '../tests/types';
  import { nodeGroups } from '../node_groups/store';
	import { formatDuration, formatTimestamp, getNodeBackgroundClass, getStatusClass, getStatusIcon } from './store';

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
      if (a.sequence !== undefined && b.sequence !== undefined) {
        return a.sequence - b.sequence;
      }
      // Fallback to executed_at since node_name might not be available
      return (a.executed_at || '').localeCompare(b.executed_at || '');
    });
  }

  function countNodeTests(nodeResult: NodeResult) {
    if (!nodeResult.test_results) return { passed: 0, total: 0 };
    
    const total = nodeResult.test_results.length;
    const passed = nodeResult.test_results.filter(test => test.success).length;
    
    return { passed, total };
  }

  function getSkipReason(nodeResult: NodeResult) {
    if (nodeResult.skip_reason) {
      return nodeResult.skip_reason;
    }
    
    if (nodeResult.status === 'Skipped') {
      return 'Skipped due to dependency failure';
    }
    
    return '';
  }

  // Derive node status from the API data
  function getNodeStatus(nodeResult: NodeResult): string {
    if (nodeResult.status) return nodeResult.status;
    
    // Derive from node_status and test results
    if (nodeResult.node_status === 'Healthy') {
      if (nodeResult.test_results && nodeResult.test_results.length > 0) {
        const allPassed = nodeResult.test_results.every(test => test.success);
        return allPassed ? 'Passed' : 'Failed';
      }
      return 'Passed';
    }
    
    return 'Failed';
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
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <h3 class="text-lg font-semibold text-white mb-4">Execution Summary</h3>
            <div class="space-y-3 text-sm">
              <div class="flex justify-between">
                <span class="text-gray-400">Status:</span>
                <div class="flex items-center gap-2">
                  {#if execution}
                    <svelte:component 
                      this={getStatusIcon(execution.status)} 
                      size={14} 
                      class={getStatusClass(execution.status)}
                    />
                    <span class="text-white">{execution.status}</span>
                  {/if}
                </div>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-400">Group:</span>
                <span class="text-white">{getGroupName(execution.group_id)}</span>
              </div>
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
            </div>
          </div>
          
          <div>
            <h3 class="text-lg font-semibold text-white mb-4">Trigger Information</h3>
            <div class="space-y-3 text-sm">
              <div>
                <span class="text-gray-400 block mb-1">Trigger Reason:</span>
                <span class="text-white">{execution.trigger_reason || 'Unknown'}</span>
              </div>
              {#if execution.id}
                <div>
                  <span class="text-gray-400 block mb-1">Execution ID:</span>
                  <span class="text-white font-mono text-xs">{execution.id}</span>
                </div>
              {/if}
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
                {#each [getNodeStatus(nodeResult)] as nodeStatus}
                  {#each [countNodeTests(nodeResult)] as testCounts}
                    {#each [getSkipReason(nodeResult)] as skipReason}
                      
                      <div class="border rounded-lg overflow-hidden {getNodeBackgroundClass(nodeStatus)}">
                        <!-- Node Header -->
                        <div class="px-4 py-3 border-b border-gray-700 flex items-center gap-3">
                          <svelte:component 
                            this={getStatusIcon(nodeStatus)} 
                            size={16} 
                            class="{getStatusClass(nodeStatus)} flex-shrink-0"
                          />
                          <div class="flex-1">
                            <div class="flex items-center gap-2">
                              <span class="font-semibold text-white">
                                {nodeResult.node_name || `Node (${nodeResult.node_status})`}
                              </span>
                              {#if nodeResult.sequence !== undefined}
                                <span class="text-xs text-gray-400 bg-gray-700 px-2 py-1 rounded">
                                  #{nodeResult.sequence}
                                </span>
                              {/if}
                            </div>
                            <div class="text-sm text-gray-400 mt-1">
                              {#if nodeStatus === 'Skipped'}
                                {skipReason}
                              {:else if testCounts.total > 0}
                                {testCounts.passed}/{testCounts.total} checks passed
                              {:else if nodeStatus === 'Running'}
                                Running checks...
                              {:else}
                                No checks performed
                              {/if}
                            </div>
                          </div>
                          {#if nodeResult.completed_at || nodeResult.executed_at}
                            <span class="text-xs text-gray-400">
                              Completed {new Date(nodeResult.completed_at || nodeResult.executed_at).toLocaleTimeString()}
                            </span>
                          {/if}
                        </div>

                        <!-- Test Results -->
                        {#if nodeResult.test_results && nodeResult.test_results.length > 0}
                          <div class="px-4 py-3 space-y-2">
                            {#each nodeResult.test_results as test}
                              <div class="flex items-start gap-3 text-sm">
                                <svelte:component 
                                  this={getStatusIcon(test.success ? 'Passed' : 'Failed')} 
                                  size={14} 
                                  class="{getStatusClass(test.success ? 'Passed' : 'Failed')} flex-shrink-0 mt-0.5"
                                />
                                <div class="flex-1 min-w-0">
                                  <div class="font-medium {test.success ? 'text-gray-300' : 'text-red-300'}">
                                    {test.details || 'Test check'}
                                  </div>
                                  <div class="text-xs text-gray-400 mt-1">
                                    {test.message}
                                    {#if test.duration_ms}
                                      • {test.duration_ms}ms
                                    {/if}
                                  </div>
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

                        <!-- Error Message -->
                        {#if nodeResult.error_message}
                          <div class="px-4 py-2 bg-red-500/20 border-t border-red-500/30 flex items-center gap-2">
                            <AlertTriangle size={14} class="text-red-400 flex-shrink-0" />
                            <span class="text-red-300 text-sm">{nodeResult.error_message}</span>
                          </div>
                        {/if}
                      </div>

                    {/each}
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