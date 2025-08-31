<script lang="ts">
    import type { CardAction, CardSection, CardList } from "./types";
	import Tag from './Tag.svelte';

  export let title: string;
  export let subtitle: string = '';
  export let status: string = '';
  export let statusColor: string = 'text-gray-400';
  export let icon: any = null;  // Expects Svelte component, not string
  export let iconColor: string = 'text-blue-400';
  export let actions: CardAction[] = [];
  export let sections: CardSection[] = [];
  export let lists: CardList[] = [];
</script>

<div class="bg-gray-800 rounded-lg p-6 border border-gray-700 hover:border-gray-600 transition-colors flex flex-col h-full">
  <!-- Header -->
  <div class="flex justify-between items-start mb-4">
    <div class="flex items-center space-x-3">
      {#if icon}
        <svelte:component this={icon} size={24} class={iconColor} />
      {/if}
      <div>
        <h3 class="text-lg font-semibold text-white">{title}</h3>
        {#if subtitle}
          <p class="text-sm text-gray-400">{subtitle}</p>
        {/if}
      </div>
    </div>
    {#if status}
      <span class="text-sm font-medium {statusColor}">{status}</span>
    {/if}
  </div>
  
  <!-- Content - grows to fill available space -->
  <div class="flex-grow space-y-3">
    <!-- Basic info sections -->
    {#each sections as section}
      <div class="text-sm text-gray-300">
        <span class="text-gray-400">{section.label}:</span>
        <span class="ml-2">{section.value}</span>
      </div>
    {/each}
    
    <!-- List sections -->
    {#each lists as list}
      <div class="text-sm">
        <div class="flex items-center flex-wrap gap-2">
          <span class="text-gray-400">{list.label}:</span>
          {#if list.items.length > 0}
            {#each list.items as item}
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2">
                  <Tag
                    icon={item.icon}
                    enableIcon={!!item.icon}
                    disabled={item.disabled}
                    bgColor={item.bgColor}
                    textColor={item.color}
                    badge={item.badge}
                    label={item.label} />
                </div>
                
                <!-- Item actions -->
                {#if list.itemActions}
                  <div class="flex items-center space-x-1">
                    {#each list.itemActions(item) as action}
                      <button
                        on:click={action.onClick}
                        disabled={action.disabled}
                        class="p-1 {action.color || 'text-gray-400'} hover:{action.hoverColor || 'text-white'} {action.bgHover || 'hover:bg-gray-700'} rounded transition-colors disabled:opacity-50"
                        title={action.label}
                      >
                        <svelte:component this={action.icon} size={16} animation={action.animation} />
                      </button>
                    {/each}
                  </div>
                {/if}
              </div>
            {/each}
          {:else}
            <span class="text-gray-500">{list.emptyText || `No ${list.label.toLowerCase()}`}</span>
          {/if}
        </div>
      </div>
    {/each}
  </div>
  
  <!-- Action Buttons -->
  {#if actions.length > 0}
    <div class="flex justify-between items-center pt-4 mt-4 border-t border-gray-700">
      {#each actions as action}
        <button
          on:click={action.onClick}
          disabled={action.disabled}
          class="p-2 {action.color || 'text-gray-400'} hover:{action.hoverColor || 'text-white'} {action.bgHover || 'hover:bg-gray-700'} rounded transition-colors disabled:opacity-50"
          title={action.label}
        >
          <svelte:component this={action.icon} size={16} />
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  button:disabled:hover {
    background-color: transparent;
    color: inherit;
  }
</style>