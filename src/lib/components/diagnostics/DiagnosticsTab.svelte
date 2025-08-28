<script lang="ts">
  import { onMount } from 'svelte';
  import { Eye, Trash2, AlertTriangle, CheckCircle, Clock, Loader2 } from 'lucide-svelte';
  import SvelteTable from 'svelte-table';
  import { diagnosticExecutions, loading, error, diagnosticsActions, formatTimestamp, formatDuration, diagnosticsActionsStore } from './store';
  import { nodeGroups } from '../node_groups/store';
  import type { DiagnosticExecution } from './types';
  import DiagnosticDetailsModal from './DiagnosticDetailsModal.svelte';

  // Define cell components
  import StatusCell from './StatusCell.svelte';
  import ActionCell from './ActionCell.svelte';
	import TabHeader from '../common/TabHeader.svelte';
	import Error from '../common/Error.svelte';
	import Loading from '../common/Loading.svelte';
	import EmptyState from '../common/EmptyState.svelte';

  // Component state
  let selectedExecution: DiagnosticExecution | null = null;
  let showDetailsModal = false;
  let deletingId: string | null = null;

  onMount(() => {
    diagnosticsActionsStore.set({
      handleViewDetails,
      handleDelete,
      deletingId
    });
    
    diagnosticsActions.loadExecutions();

  });

  $: diagnosticsActionsStore.update(current => ({ 
    ...current, 
    deletingId 
  }));

  // Table configuration
  $: columns = [
    {
      key: 'status',
      title: 'Status',
      value: (row: DiagnosticExecution) => row.status,
      sortable: true,
      headerClass: 'px-6 py-3 text-center text-xs font-medium text-gray-300 uppercase tracking-wider',
      class: 'text-center',
      renderComponent: {
        component: StatusCell,
        props: (row: DiagnosticExecution) => ({ 
          execution: row
        })
      }
    },
    {
      key: 'group_id',
      title: 'Group',
      value: (row: DiagnosticExecution) => getGroupName(row.group_id),
      sortable: true,
      class: 'text-center',
      headerClass: 'px-6 py-3 text-center text-xs font-medium text-gray-300 uppercase tracking-wider',
    },
    {
      key: 'trigger_reason',
      title: 'Trigger',
      value: (row: DiagnosticExecution) => row.trigger_reason,
      sortable: true,
      headerClass: 'px-6 py-3 text-center text-xs font-medium text-gray-300 uppercase tracking-wider',
      class: 'max-w-xs text-center'
    },
    {
      key: 'nodes',
      title: 'Nodes',
      value: (row: DiagnosticExecution) => getNodesSummary(row),
      sortable: false,
      headerClass: 'px-6 py-3 text-center text-xs font-medium text-gray-300 uppercase tracking-wider',
      class: 'text-center'
    },
    {
      key: 'started_at',
      title: 'Started',
      value: (row: DiagnosticExecution) => formatTimestamp(row?.started_at || ''),
      sortable: true,
      class: 'text-center',
      headerClass: 'px-6 py-3 text-center text-xs font-medium text-gray-300 uppercase tracking-wider',
      parseHTML: false
    },
    {
      key: 'duration',
      title: 'Duration',
      value: (row: DiagnosticExecution) => formatDuration(row?.started_at || '', row?.completed_at || ''),
      sortable: false,
      headerClass: 'px-6 py-3 text-center text-xs font-medium text-gray-300 uppercase tracking-wider',
      class: 'text-center'
    },
    {
      key: 'actions',
      title: 'Actions',
      value: () => '',
      sortable: false,
      headerClass: 'px-6 py-3 text-center text-xs font-medium text-gray-300 uppercase tracking-wider',
      class: 'text-center',
      renderComponent: ActionCell
    }
  ];

  function handleViewDetails(execution: DiagnosticExecution) {
    selectedExecution = execution;
    showDetailsModal = true;
  }

  function handleCloseModal() {
    showDetailsModal = false;
    selectedExecution = null;
  }

  async function handleDelete(execution: DiagnosticExecution) {
    if (confirm(`Are you sure you want to delete the diagnostic execution?`)) {
      deletingId = execution.id;
      const success = await diagnosticsActions.deleteExecution(execution.id);
      deletingId = null;
    }
  }

  function getNodesSummary(execution: DiagnosticExecution) {
    if (!execution || !execution.node_results || execution.node_results.length === 0) {
      return 'No nodes';
    }

    const total = execution.node_results.length;
    // For API data, derive status from node_status and test results
    const passed = execution.node_results.filter(n => {
      if (n?.node_status === 'Healthy') return true;
      if (n?.test_results && n.test_results.length > 0) {
        return n.test_results.every(t => t.success);
      }
      return false;
    }).length;
    
    const failed = total - passed;

    if (failed > 0) {
      return `${passed}/${total} passed, ${failed} failed`;
    }
    return `${passed}/${total} passed`;
  }

  // Get group name from group ID using nodeGroups store
  function getGroupName(groupId: string): string {
    const group = $nodeGroups.find(g => g.id === groupId);
    return group?.name || groupId; // Fallback to ID if name not found
  }
</script>

<div class="space-y-6">
  <!-- Header -->
   <TabHeader
    title="Diagnostics"
    subtitle="Monitor and manage diagnostic executions"
    buttons={[
      {
        onclick: diagnosticsActions.loadExecutions,
        IconComponent: Loader2,
        disabled: $loading,
        cta: "Reload"
      }
    ]}
    />

  <!-- Error display -->
  <Error error={$error} onClear={diagnosticsActions.clearError} />

  <!-- Loading state -->
  {#if $loading}
    <Loading />
  {:else if $diagnosticExecutions.length === 0 && !$loading}
    <!-- Empty state -->
     <EmptyState 
        title="No diagnostic runs found" 
        subtitle="Diagnostic runs will appear here when node monitoring detects issues or when manually executed." />
  {:else}
    <!-- Table -->
    <div class="bg-gray-800 border border-gray-700 rounded-lg overflow-hidden">
      <SvelteTable 
        {columns} 
        rows={$diagnosticExecutions}
        classNameTable="w-full"
        classNameThead="bg-gray-700/50"
        classNameTbody=""
        classNameCell="px-6 py-4 text-sm text-gray-300"
        classNameRow="hover:bg-gray-700/30 transition-colors border-b border-gray-700 last:border-b-0"
        sortBy="started_at"
        sortOrder={-1}
        rowKey="id"
      />
    </div>
  {/if}
</div>

<!-- Details Modal -->
<DiagnosticDetailsModal
  execution={selectedExecution}
  isOpen={showDetailsModal}
  onClose={handleCloseModal}
/>