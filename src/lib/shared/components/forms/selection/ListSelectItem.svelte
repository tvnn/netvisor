<script lang="ts" generics="T">
	import Tag from '../../data/Tag.svelte';
	import type { EntityDisplayComponent } from './types';


  export let item: T;
  export let displayComponent: EntityDisplayComponent<T>;

  let icon = displayComponent.getIcon?.(item);
  let tags = displayComponent.getTags?.(item) || [];
  let description = displayComponent.getDescription?.(item) || '';
</script>

<div class="flex items-center gap-3 flex-1 min-w-0">
  <!-- Icon -->
  {#if icon}
    <div class="w-6 h-6 rounded bg-gray-600 flex items-center justify-center flex-shrink-0">
      <svelte:component 
        this={icon} 
        class="w-3 h-3 {displayComponent.getIconColor?.(item) || 'text-gray-300'}" 
      />
    </div>
  {/if}
  
  <!-- Label and description -->
  <div class="flex-1 min-w-0 text-left">
    <div class="flex gap-3 pb-1">
      <span class="block truncate">{displayComponent.getLabel(item)}</span>
      <!-- Tags -->
      {#if tags.length > 0}
        <div class="flex gap-1">
          {#each tags as tag}
            <Tag
              label={tag.label}
              color={tag.color}
              textColor={tag.textColor}
              bgColor={tag.bgColor} />
          {/each}
        </div>
      {/if}
    </div>
    {#if description}
      <span class="block text-xs text-gray-400 truncate">{description}</span>
    {/if}
  </div>
</div>