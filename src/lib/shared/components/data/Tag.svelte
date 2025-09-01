<script lang="ts">
	import { createColorHelper } from "$lib/shared/utils/styling";
	import { Circle } from "lucide-svelte";

    export let icon = Circle;
    export let enableIcon = false

    export let color: string = 'gray';
    export let bgColor: string = 'bg-gray-700/30';
    export let textColor: string = 'text-gray-500';

    export let disabled = false;
    export let label: string;

    export let badge: string = ''

    const colorHelper = createColorHelper(color)

    if (color !== 'gray') {
        bgColor = colorHelper.bg;
        textColor = colorHelper.text;
    }

</script>

<div class="items-center space-x-2">

{#if enableIcon}
    <svelte:component this={icon} size={16} class={textColor} />
{/if}

<!-- Main content -->
<span class="inline-block {!disabled ? bgColor : 'bg-gray-700/30'} {!disabled ? textColor : 'text-gray-500'} px-2 py-1 rounded text-xs">
    {label}
    {#if badge.length > 0}
        <span class="text-xs {textColor}">{badge}</span>
    {/if}
</span>

</div>