<script lang="ts">
  import { Server, Activity } from 'lucide-svelte';
  
  export let activeTab: string;
  export let onTabChange: (tab: string) => void;
  
  const navItems = [
    { id: 'nodes', label: 'Nodes', icon: Server },
    { id: 'diagnostics', label: 'Diagnostic Groups', icon: Activity },
  ];
</script>

<div class="w-64 bg-gray-800 border-r border-gray-700 min-h-screen flex flex-col">
  <!-- Logo/Brand -->
  <div class="p-6 border-b border-gray-700">
    <h1 class="text-xl font-bold text-white">NetFrog</h1>
    <p class="text-sm text-gray-400">Network Diagnostics</p>
  </div>
  
  <!-- Navigation -->
  <nav class="flex-1 p-4">
    <ul class="space-y-2">
      {#each navItems as item}
        <li>
          <button
            on:click={() => onTabChange(item.id)}
            class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors"
            class:bg-blue-600={activeTab === item.id}
            class:text-white={activeTab === item.id}
            class:text-gray-300={activeTab !== item.id}
            class:hover:text-white={activeTab !== item.id}
            class:hover:bg-gray-700={activeTab !== item.id}
          >
            <svelte:component this={item.icon} class="w-5 h-5" />
            {item.label}
          </button>
        </li>
      {/each}
    </ul>
  </nav>
  
  <!-- Footer -->
  <div class="p-4 border-t border-gray-700">
    <div class="flex items-center gap-2 text-xs text-gray-400">
      <div class="w-2 h-2 bg-green-500 rounded-full"></div>
      <span>Server Running</span>
    </div>
  </div>
</div>