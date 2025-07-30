<script lang="ts">
  import { Activity, Server, GitBranch, Network } from 'lucide-svelte';
  import { activeTab } from '../stores/ui';

  const navigationItems = [
    {
      id: 'diagnostics',
      name: 'Diagnostics',
      icon: Activity,
      description: 'Run network tests'
    },
    {
      id: 'nodes',
      name: 'Nodes',
      icon: Server,
      description: 'Manage network resources'
    },
    {
      id: 'tests',
      name: 'Tests',
      icon: GitBranch,
      description: 'Configure network tests'
    }
  ];

  function switchTab(tabId: string) {
    activeTab.set(tabId);
  }
</script>

<div class="w-64 bg-gray-800 border-r border-gray-700 flex flex-col">
  <!-- Logo and Title -->
  <div class="p-6 border-b border-gray-700">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-gradient-to-br from-blue-600 to-purple-600 rounded-lg">
        <Network class="w-6 h-6 text-white" />
      </div>
      <div>
        <h1 class="text-xl font-bold text-white">Netzoot</h1>
      </div>
    </div>
  </div>

  <!-- Navigation -->
  <nav class="flex-1 p-4">
    <div class="space-y-2">
      {#each navigationItems as item}
        <button
          on:click={() => switchTab(item.id)}
          class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors"
          class:bg-blue-600={$activeTab === item.id}
          class:text-white={$activeTab === item.id}
          class:shadow-lg={$activeTab === item.id}
          class:text-gray-300={$activeTab !== item.id}
          class:hover:bg-gray-700={$activeTab !== item.id}
          class:hover:text-white={$activeTab !== item.id}
        >
          <div class="p-1.5 rounded {$activeTab === item.id ? 'bg-white bg-opacity-20' : 'bg-gray-600'}">
            <svelte:component this={item.icon} class="w-4 h-4" />
          </div>
          <div class="flex-1">
            <div class="font-medium text-sm">{item.name}</div>
            <div class="text-xs opacity-75">{item.description}</div>
          </div>
        </button>
      {/each}
    </div>
  </nav>

  <!-- Footer -->
  <div class="p-4 border-t border-gray-700">
    <div class="text-xs text-gray-500 text-center">
      v1.0.0
    </div>
  </div>
</div>