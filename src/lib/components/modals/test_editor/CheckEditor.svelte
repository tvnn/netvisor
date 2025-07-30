<script lang="ts">
  import { Trash2, Play, ChevronDown, ChevronRight } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';
  import { getDefaultCheckConfig, CHECK_TYPES, CATEGORY_ICONS } from '../../../stores/checks';
  import type { Check, NetworkNode } from '$lib/types';

  import CheckTypeSelector from '../CheckTypeSelector.svelte';
  import NodeReferenceHelper from './NodeReferenceHelper.svelte';

  export let check: Check;
  export let index: number;
  export let nodeOptions: NetworkNode[] = [];
  export let expanded: boolean = false;

  const dispatch = createEventDispatcher<{
    checkUpdate: { index: number; check: Check };
    toggleExpanded: void;
    remove: void;
  }>();

  function updateCheckType(event: CustomEvent<{ value: string }>): void {
    const newType = event.detail.value;
    
    // Reset config when changing check type
    check = {
      ...check,
      type: newType,
      config: getDefaultCheckConfig(newType)
    };
    
    dispatchUpdate();
  }

  function updateCheckConfig(field: string, value: any): void {
    check.config = {
      ...check.config,
      [field]: value
    };
    dispatchUpdate();
  }

  function handleNodeReferenceInsert(event: CustomEvent<{ field: string; reference: string }>): void {
    const { field, reference } = event.detail;
    const currentValue = (check.config as any)[field] || '';
    updateCheckConfig(field, currentValue + reference);
  }

  function dispatchUpdate(): void {
    dispatch('checkUpdate', { index, check });
  }
</script>

<div class="border border-gray-600 rounded bg-gray-900/50">
  <!-- Check Header -->
  <div class="flex items-center justify-between p-2">
    <button
      type="button"
      on:click={() => dispatch('toggleExpanded')}
      class="flex items-center gap-2 text-sm text-gray-300 hover:text-white transition-colors flex-1 text-left"
    >
      {#if expanded}
        <ChevronDown class="w-3 h-3" />
      {:else}
        <ChevronRight class="w-3 h-3" />
      {/if}
      
      {#if check.type && CHECK_TYPES[check.type]}
        <div class="flex items-center gap-2">
          <svelte:component this={CATEGORY_ICONS[CHECK_TYPES[check.type].category]} class="w-3 h-3" />
          <span>{CHECK_TYPES[check.type].name}</span>
        </div>
      {:else}
        <span class="text-gray-500">Unconfigured Check</span>
      {/if}
    </button>

    <div class="flex items-center gap-1">
      <button
        type="button"
        on:click={() => dispatch('toggleExpanded')}
        class="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white transition-colors"
        title="Test check"
      >
        <Play class="w-3 h-3" />
      </button>
      <button
        type="button"
        on:click={() => dispatch('remove')}
        class="p-1 hover:bg-red-700 rounded text-gray-400 hover:text-red-400 transition-colors"
        title="Remove check"
      >
        <Trash2 class="w-3 h-3" />
      </button>
    </div>
  </div>

  <!-- Check Configuration -->
  {#if expanded}
    <div class="p-3 space-y-3">
      <!-- Check Type Selection -->
      <div>
        <label for="check-type-{index}" class="block text-sm font-medium text-gray-300 mb-2">
          Check Type
        </label>
        <div id="check-type-{index}">
          <CheckTypeSelector 
            bind:value={check.type}
            on:change={updateCheckType}
          />
        </div>
      </div>

      <!-- Dynamic Check Configuration -->
      {#if check.type && CHECK_TYPES[check.type]}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
          {#each CHECK_TYPES[check.type].fields as field}
            <div>
              <label for="check-{index}-{field}" class="block text-xs font-medium text-gray-300 mb-1 capitalize">
                {field.replace('_', ' ')}
              </label>
              
              {#if field === 'protocol'}
                <select
                  id="check-{index}-{field}"
                  bind:value={(check.config as any)[field]}
                  on:change={() => dispatchUpdate()}
                  class="w-full bg-gray-700 border border-gray-600 text-white rounded px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500"
                >
                  <option value="http">HTTP</option>
                  <option value="https">HTTPS</option>
                </select>
              {:else if field === 'service_type'}
                <select
                  id="check-{index}-{field}"
                  bind:value={(check.config as any)[field]}
                  on:change={() => dispatchUpdate()}
                  class="w-full bg-gray-700 border border-gray-600 text-white rounded px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500"
                >
                  <option value="auto">Auto-detect</option>
                  <option value="google">Google</option>
                  <option value="cloudflare">Cloudflare</option>
                  <option value="pihole">Pi-hole</option>
                </select>
              {:else if field.includes('port') || field.includes('timeout') || field.includes('status') || field.includes('time') || field === 'attempts'}
                <input
                  id="check-{index}-{field}"
                  type="number"
                  bind:value={(check.config as any)[field]}
                  on:input={() => dispatchUpdate()}
                  class="w-full bg-gray-700 border border-gray-600 text-white rounded px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500"
                  placeholder={field === 'port' ? '443' : field === 'timeout' ? '5000' : ''}
                />
              {:else}
                <div class="relative">
                  <input
                    id="check-{index}-{field}"
                    type="text"
                    bind:value={(check.config as any)[field]}
                    on:input={() => dispatchUpdate()}
                    class="w-full bg-gray-700 border border-gray-600 text-white rounded px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500"
                    placeholder={field === 'target' ? 'example.com or {{node-id}}' : field === 'domain' ? 'google.com' : ''}
                  />
                  {#if (field === 'target' || field === 'domain') && nodeOptions.length > 0}
                    <NodeReferenceHelper
                      {nodeOptions}
                      {field}
                      on:insertReference={handleNodeReferenceInsert}
                    />
                  {/if}
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>