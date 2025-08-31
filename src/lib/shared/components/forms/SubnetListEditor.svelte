<script lang="ts">
  import { Plus, Trash2 } from 'lucide-svelte';
  
  export let subnets: string[] = [];
  
  let newSubnet = '';
  let error = '';
  
  function validateCIDR(cidr: string): boolean {
    // Basic CIDR validation: IP/prefix
    return /^(\d{1,3}\.){3}\d{1,3}\/\d{1,2}$/.test(cidr);
  }
  
  function addSubnet() {
    const trimmed = newSubnet.trim();
    if (!trimmed) return;
    
    if (!validateCIDR(trimmed)) {
      error = 'Invalid CIDR format (e.g., 192.168.1.0/24)';
      return;
    }
    
    if (subnets.includes(trimmed)) {
      error = 'Subnet already added';
      return;
    }
    
    subnets = [...subnets, trimmed];
    newSubnet = '';
    error = '';
  }
  
  function removeSubnet(index: number) {
    subnets = subnets.filter((_, i) => i !== index);
  }
</script>

<div>
  <label class="block text-sm font-medium text-gray-300 mb-2">
    Network Subnets
  </label>
  
  <!-- Add new subnet -->
  <div class="flex gap-2 mb-3">
    <input
      type="text"
      bind:value={newSubnet}
      placeholder="192.168.1.0/24"
      class="flex-1 px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      class:border-red-500={error}
      on:keydown={(e) => {
        if (e.key === 'Enter') {
          e.preventDefault();
          addSubnet();
        }
      }}
    />
    <button
      type="button"
      on:click={addSubnet}
      disabled={!newSubnet.trim()}
      class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
    >
      <Plus size={16} />
      Add
    </button>
  </div>
  
  {#if error}
    <p class="text-red-400 text-xs mb-3">{error}</p>
  {/if}
  
  <!-- Current subnets -->
  {#if subnets.length > 0}
    <div class="space-y-2 mb-3">
      {#each subnets as subnet, index}
        <div class="flex items-center gap-3 p-3 rounded-lg border bg-gray-700/50 border-gray-600">
          <span class="flex-1 font-mono text-sm text-white">{subnet}</span>
          <button
            type="button"
            on:click={() => removeSubnet(index)}
            class="p-1 text-gray-400 hover:text-red-400 hover:bg-red-900/20 rounded"
            title="Remove subnet"
          >
            <Trash2 size={16} />
          </button>
        </div>
      {/each}
    </div>
  {:else}
    <div class="text-gray-500 text-sm mb-3 p-3 bg-gray-700/30 rounded-lg text-center">
      No subnets added yet
    </div>
  {/if}
  
  <p class="text-xs text-gray-400">
    CIDR blocks this node belongs to (e.g., 192.168.1.0/24)
  </p>
</div>