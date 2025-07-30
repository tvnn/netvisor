<script lang="ts">
  import { Plus, Edit, Copy, Trash2, Download, Share, Upload, GitBranch } from 'lucide-svelte';
  import { topologies, topologyActions, createBlankTopology } from '../../stores/topologies';
  import { modalActions, notificationActions } from '../../stores/ui';
  import TopologyEditor from '../modals/TopologyEditor.svelte';
  import ConfirmDialog from '../modals/ConfirmDialog.svelte';
  import type { Topology } from '../../types';

  function createTopology() {
    const blankTopology = createBlankTopology();
    modalActions.open(TopologyEditor, {
      mode: 'create',
      topology: blankTopology
    }, 'Create New Topology');
  }

  function editTopology(topology: Topology) {
    modalActions.open(TopologyEditor, {
      mode: 'edit',
      topology: { ...topology }
    }, `Edit ${topology.name}`);
  }

  async function duplicateTopology(topology: Topology) {
    try {
      const duplicated = await topologyActions.duplicate(topology.id!);
      notificationActions.success(`Duplicated topology: ${topology.name}`);
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      notificationActions.error(`Failed to duplicate topology: ${errorMessage}`);
    }
  }

  function deleteTopology(topology: Topology) {
    modalActions.open(ConfirmDialog, {
      title: 'Delete Topology',
      message: `Are you sure you want to delete "${topology.name}"? This action cannot be undone.`,
      confirmText: 'Delete',
      cancelText: 'Cancel',
      danger: true,
      onConfirm: async () => {
        try {
          await topologyActions.delete(topology.id!);
          notificationActions.success(`Deleted topology: ${topology.name}`);
        } catch (error: unknown) {
          const errorMessage = error instanceof Error ? error.message : 'Unknown error';
          notificationActions.error(`Failed to delete topology: ${errorMessage}`);
        }
      }
    }, 'Confirm Deletion');
  }

  function exportTopology(topology: Topology) {
    const data = JSON.stringify(topology, null, 2);
    const filename = `${topology.name.toLowerCase().replace(/\s+/g, '-')}-topology.json`;
    
    // Create download link
    const blob = new Blob([data], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    
    notificationActions.success(`Exported topology: ${topology.name}`);
  }

  function importTopology() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = (event: Event) => {
      const target = event.target as HTMLInputElement;
      const file = target.files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = async (e: ProgressEvent<FileReader>) => {
          try {
            const result = e.target?.result;
            if (typeof result === 'string') {
              const topology = JSON.parse(result) as Topology;
              // Remove ID to create as new topology
              delete topology.id;
              delete topology.createdAt;
              delete topology.updatedAt;
              
              await topologyActions.add(topology);
              notificationActions.success(`Imported topology: ${topology.name}`);
            }
          } catch (error: unknown) {
            const errorMessage = error instanceof Error ? error.message : 'Unknown error';
            notificationActions.error(`Failed to import topology: ${errorMessage}`);
          }
        };
        reader.readAsText(file);
      }
    };
    input.click();
  }

  function getTopologyStats(topology: Topology) {
    const layerCount = topology.layers?.length || 0;
    const testCount = topology.layers?.reduce((sum, layer) => sum + (layer.tests?.length || 0), 0) || 0;
    return { layerCount, testCount };
  }
</script>

<div class="space-y-6">
  <!-- Header with actions -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold text-white">Test Topologies</h2>
      <p class="text-gray-400 mt-1">Manage your network testing configurations</p>
    </div>
    
    <div class="flex gap-2">
      <button
        class="flex items-center gap-2 px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors"
        on:click={importTopology}
      >
        <Upload class="w-4 h-4" />
        Import
      </button>
      <button
        class="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
        on:click={createTopology}
      >
        <Plus class="w-4 h-4" />
        Create Topology
      </button>
    </div>
  </div>

  <!-- Topologies Grid -->
  {#if $topologies.length > 0}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each $topologies as topology}
        {@const stats = getTopologyStats(topology)}
        
        <div class="bg-gray-800 rounded-xl border border-gray-700 p-6 hover:border-gray-600 transition-colors">
          <!-- Topology header -->
          <div class="flex items-start gap-3 mb-4">
            <div class="flex-1 min-w-0">
              <h3 class="font-semibold text-white text-lg mb-1 truncate">
                {topology.name}
              </h3>
              {#if topology.description}
                <p class="text-gray-400 text-sm line-clamp-2">
                  {topology.description}
                </p>
              {/if}
            </div>
          </div>

          <!-- Stats -->
          <div class="flex items-center gap-4 mb-4 text-sm text-gray-300">
            <span>{stats.layerCount} layers</span>
            <span>•</span>
            <span>{stats.testCount} tests</span>
            <span>•</span>
            <span>v{topology.version || '1.0'}</span>
          </div>

          <!-- Actions -->
          <div class="flex items-center justify-between">
            <!-- Primary actions -->
            <div class="flex gap-1">
              <button
                class="p-2 hover:bg-gray-700 rounded-lg text-gray-400 hover:text-white transition-colors"
                on:click={() => editTopology(topology)}
                title="Edit topology"
              >
                <Edit class="w-4 h-4" />
              </button>
              <button
                class="p-2 hover:bg-gray-700 rounded-lg text-gray-400 hover:text-white transition-colors"
                on:click={() => duplicateTopology(topology)}
                title="Duplicate topology"
              >
                <Copy class="w-4 h-4" />
              </button>
              <button
                class="p-2 hover:bg-gray-700 rounded-lg text-gray-400 hover:text-white transition-colors"
                on:click={() => exportTopology(topology)}
                title="Export topology"
              >
                <Download class="w-4 h-4" />
              </button>
            </div>

            <!-- Danger zone -->
            <button
              class="p-2 hover:bg-red-700 rounded-lg text-gray-400 hover:text-red-400 transition-colors"
              on:click={() => deleteTopology(topology)}
              title="Delete topology"
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <!-- Empty State -->
    <div class="text-center py-12">
      <GitBranch class="w-16 h-16 mx-auto text-gray-600 mb-4" />
      <h3 class="text-xl font-semibold text-gray-300 mb-2">No Test Topologies</h3>
      <p class="text-gray-400 mb-6 max-w-md mx-auto">
        Create your first topology to define network diagnostic tests and organize them into logical layers.
      </p>
      <button
        on:click={createTopology}
        class="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors mx-auto"
      >
        <Plus class="w-4 h-4" />
        Create Topology
      </button>
    </div>
  {/if}
</div>