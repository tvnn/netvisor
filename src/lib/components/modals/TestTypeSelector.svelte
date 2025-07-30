<script>
  import { ChevronDown, ChevronUp, Zap } from 'lucide-svelte';
  import { TEST_TYPES } from '../../stores/topologies';
  import { createEventDispatcher } from 'svelte';

  export let value = '';
  export let placeholder = 'Select test type...';

  const dispatch = createEventDispatcher();
  
  let isOpen = false;
  let dropdownElement;

  $: selectedType = TEST_TYPES[value];

  function toggleDropdown() {
    isOpen = !isOpen;
  }

  function selectType(type) {
    value = type;
    isOpen = false;
    dispatch('change', { value: type });
  }

  function handleClickOutside(event) {
    if (dropdownElement && !dropdownElement.contains(event.target)) {
      isOpen = false;
    }
  }

  function handleKeydown(event) {
    if (event.key === 'Escape') {
      isOpen = false;
    }
  }

  // Get category for grouping
  function getCategory(type) {
    if (['connectivity_test', 'dns_resolution', 'dns_over_https', 'service_health', 'response_time', 'ping_test'].includes(type)) {
      return 'Basic Tests';
    } else if (['vpn_connectivity', 'vpn_tunnel'].includes(type)) {
      return 'VPN Tests';
    } else {
      return 'Network Layer Tests';
    }
  }

  // Group tests by category
  $: groupedTests = Object.entries(TEST_TYPES).reduce((groups, [type, config]) => {
    const category = getCategory(type);
    if (!groups[category]) groups[category] = [];
    groups[category].push([type, config]);
    return groups;
  }, {});
</script>

<svelte:window on:click={handleClickOutside} on:keydown={handleKeydown} />

<div class="relative" bind:this={dropdownElement}>
  <!-- Trigger Button -->
  <button
    type="button"
    on:click={toggleDropdown}
    class="w-full bg-gray-700 border border-gray-600 text-white rounded px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 flex items-center justify-between hover:bg-gray-650 transition-colors"
  >
    <div class="flex items-center gap-2 flex-1 text-left">
      <Zap class="w-4 h-4 text-gray-400" />
      <div>
        {#if selectedType}
          <div class="font-medium">{selectedType.name}</div>
        {:else}
          <div class="text-gray-400">{placeholder}</div>
        {/if}
      </div>
    </div>
    <div class="flex items-center">
      {#if isOpen}
        <ChevronUp class="w-4 h-4 text-gray-400" />
      {:else}
        <ChevronDown class="w-4 h-4 text-gray-400" />
      {/if}
    </div>
  </button>

  <!-- Dropdown Menu -->
  {#if isOpen}
    <div class="fixed z-[9999] w-full mt-1 bg-gray-800 border border-gray-600 rounded-lg shadow-xl max-h-96 overflow-auto" style="top: {dropdownElement?.getBoundingClientRect().bottom + window.scrollY + 4}px; left: {dropdownElement?.getBoundingClientRect().left + window.scrollX}px; width: {dropdownElement?.getBoundingClientRect().width}px">
      {#each Object.entries(groupedTests) as [category, tests]}
        <div class="p-2">
          <!-- Category Header -->
          <div class="px-2 py-1 text-xs font-semibold text-gray-400 uppercase tracking-wider border-b border-gray-700 mb-2">
            {category}
          </div>
          
          <!-- Test Options -->
          {#each tests as [type, config]}
            <button
              type="button"
              on:click={() => selectType(type)}
              class="w-full text-left px-3 py-3 rounded hover:bg-gray-700 transition-colors group"
              class:bg-blue-900={value === type}
              class:bg-opacity-30={value === type}
            >
              <div class="flex items-start gap-3">
                <div class="p-1 rounded {value === type ? 'bg-blue-600' : 'bg-gray-600 group-hover:bg-gray-500'} transition-colors">
                  <Zap class="w-3 h-3 text-white" />
                </div>
                <div class="flex-1 min-w-0">
                  <div class="font-medium text-white text-sm mb-1">
                    {config.name}
                  </div>
                  <div class="text-xs text-gray-400 leading-relaxed">
                    {config.description}
                  </div>
                </div>
              </div>
            </button>
          {/each}
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Selected Type Description (shown below dropdown) -->
{#if selectedType}
  <div class="mt-2 p-3 bg-blue-900/20 border border-blue-700/30 rounded-lg">
    <div class="flex items-start gap-2">
      <div class="p-1 bg-blue-600 rounded">
        <Zap class="w-3 h-3 text-white" />
      </div>
      <div>
        <div class="font-medium text-blue-200 text-sm mb-1">
          {selectedType.name}
        </div>
        <div class="text-xs text-blue-300 leading-relaxed">
          {selectedType.description}
        </div>
      </div>
    </div>
  </div>
{/if}