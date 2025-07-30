<script lang="ts">
  import { ChevronDown, ChevronUp, Zap } from 'lucide-svelte';
  import { CHECK_TYPES, CATEGORY_ICONS } from '$lib/stores/checks';
  import { createEventDispatcher, tick } from 'svelte';

  export let value: string = '';
  export let placeholder: string = 'Select check type...';

  const dispatch = createEventDispatcher<{
    change: { value: string };
  }>();
  
  let isOpen: boolean = false;
  let triggerElement: HTMLButtonElement | undefined;
  let dropdownElement: HTMLDivElement | undefined;

  $: selectedType = CHECK_TYPES[value];

  async function toggleDropdown(): Promise<void> {
    isOpen = !isOpen;
    if (isOpen) {
      await tick();
      // Use requestAnimationFrame to ensure DOM is ready
      requestAnimationFrame(() => {
        positionDropdown();
      });
    }
  }

  function positionDropdown(): void {
    if (!isOpen || !triggerElement || !dropdownElement) return;
    
    const rect = triggerElement.getBoundingClientRect();
    const viewportHeight = window.innerHeight;
    const dropdownHeight = 250; // Approximate max height
    
    // Calculate space above and below
    const spaceBelow = viewportHeight - rect.bottom;
    const spaceAbove = rect.top;
    
    // Position dropdown
    if (spaceBelow >= 200 || spaceBelow > spaceAbove) {
      // Position below
      dropdownElement.style.top = (rect.bottom + window.scrollY + 4) + 'px';
      dropdownElement.style.maxHeight = Math.min(spaceBelow - 20, dropdownHeight) + 'px';
    } else {
      // Position above
      const maxHeight = Math.min(spaceAbove - 20, dropdownHeight);
      dropdownElement.style.top = (rect.top + window.scrollY - maxHeight - 4) + 'px';
      dropdownElement.style.maxHeight = maxHeight + 'px';
    }
    
    dropdownElement.style.left = (rect.left + window.scrollX) + 'px';
    dropdownElement.style.width = rect.width + 'px';
  }

  function selectType(type: string): void {
    value = type;
    isOpen = false;
    dispatch('change', { value: type });
  }

  function handleClickOutside(event: MouseEvent): void {
    const target = event.target as Node | null;
    if (triggerElement && !triggerElement.contains(target) && 
        dropdownElement && !dropdownElement.contains(target)) {
      isOpen = false;
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === 'Escape') {
      isOpen = false;
    }
  }

  function handleScroll(): void {
    if (isOpen && triggerElement && dropdownElement) {
      // Use requestAnimationFrame to prevent infinite loops
      requestAnimationFrame(() => {
        positionDropdown();
      });
    }
  }

  // Action to position dropdown when it mounts
  function positionOnMount(node: HTMLElement) {
    requestAnimationFrame(() => {
      positionDropdown();
    });
    
    return {
      destroy() {
        // Cleanup if needed
      }
    };
  }

  // Group checks by category
  $: groupedChecks = Object.entries(CHECK_TYPES).reduce((groups: Record<string, Array<[string, any]>>, [type, config]) => {
    const category = config.category;
    if (!groups[category]) groups[category] = [];
    groups[category].push([type, config]);
    return groups;
  }, {});
</script>

<svelte:window 
  on:click={handleClickOutside} 
  on:keydown={handleKeydown}
  on:scroll={handleScroll}
  on:resize={handleScroll}
/>

<div class="relative">
  <!-- Trigger Button -->
  <button
    type="button"
    bind:this={triggerElement}
    on:click={toggleDropdown}
    class="w-full bg-gray-700 border border-gray-600 text-white rounded px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 flex items-center justify-between hover:bg-gray-650 transition-colors"
  >
    <div class="flex items-center gap-2 flex-1 text-left">
      {#if selectedType}
        <div><div class="font-medium">{selectedType.name}</div></div>
      {:else}
        <div><div class="text-gray-400">{placeholder}</div></div>
      {/if}
    </div>
    <div class="flex items-center">
      {#if isOpen}
        <ChevronUp class="w-4 h-4 text-gray-400" />
      {:else}
        <ChevronDown class="w-4 h-4 text-gray-400" />
      {/if}
    </div>
  </button>
</div>

<!-- Fixed positioned dropdown outside modal -->
{#if isOpen}
  <div 
    bind:this={dropdownElement}
    class="fixed z-[9999] bg-gray-800 border border-gray-600 rounded-lg shadow-xl overflow-auto"
    style="top: 0px; left: 0px; width: 300px; max-height: 250px;"
    use:positionOnMount
  >
    {#each Object.entries(groupedChecks) as [category, checks]}
      <div class="p-2">
        <!-- Category Header -->
        <div class="px-2 py-1 text-xs font-semibold text-gray-400 uppercase tracking-wider border-b border-gray-700 mb-2">
          {category}
        </div>
        
        <!-- Check Options -->
        {#each checks as [type, config]}
          <button
            type="button"
            on:click={() => selectType(type)}
            class="w-full text-left px-3 py-2 rounded hover:bg-gray-700 transition-colors group"
            class:bg-blue-900={value === type}
            class:bg-opacity-30={value === type}
          >
            <div class="flex items-start gap-3">
              <div class="p-1 rounded {value === type ? 'bg-blue-600' : 'bg-gray-600 group-hover:bg-gray-500'} transition-colors">
                <svelte:component this={CATEGORY_ICONS[category]} class="w-3 h-3 text-white" />
              </div>
              <div class="flex-1 min-w-0">
                <div class="font-medium text-white text-sm mb-1">
                  {config.name}
                </div>
                <div class="text-xs text-gray-400 leading-tight">
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

<!-- Selected Type Description (shown below dropdown) -->
{#if selectedType}
  <div class="mt-2 p-3 bg-blue-900/20 border border-blue-700/30 rounded-lg">
    <div class="flex items-start gap-2">
      <div>
        <div class="text-xs text-blue-300 leading-relaxed">
          {selectedType.details}
        </div>
      </div>
    </div>
  </div>
{/if}