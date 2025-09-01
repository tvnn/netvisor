<script lang="ts">
	import Tag from "../data/Tag.svelte";
	import type { TagProps } from "../data/types";

    export let item: any;
    export let getIcon: (item: any) => any | null = (item) => null;
    export let getIconColor: (item: any) => string | null = (item) => null;
    export let getTags: (item: any) => TagProps[] = (item) => [];
    export let getLabel: (item: any) => string | null = (item) => null
    export let getDescription: (item: any) => string | null = (item) => null

</script>

<div class="flex items-center gap-3 flex-1 min-w-0">
        <!-- Icon -->
        {#if getIcon}
            {@const icon = getIcon(item)}
            {#if icon}
                <div class="w-6 h-6 rounded bg-gray-600 flex items-center justify-center flex-shrink-0">
                <svelte:component 
                    this={icon} 
                    class="w-3 h-3 {getIconColor ? getIconColor(item) : 'text-gray-300'}" 
                />
                </div>
            {/if}
        {/if}
        
        <!-- Label and description -->
        <div class="flex-1 min-w-0 text-left">
        <span class="block truncate">{getLabel(item)}</span>
        {#if getDescription}
            <span class="block text-xs text-gray-400 truncate">{getDescription(item)}</span>
        {/if}
        </div>
        
        <!-- Tag -->
        {#if getTags}
        {@const tags = getTags(item)}
            {#each tags as tag}
                <Tag
                    label={tag.label}
                    textColor={tag.textColor}
                    bgColor={tag.bgColor} />
            {/each}
        {/if}
</div>