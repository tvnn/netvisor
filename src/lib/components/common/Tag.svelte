<script lang="ts">
	import { Circle } from "lucide-svelte";
	import type { Component } from "svelte";
	import { getBgColor, getTextColor } from "./colors";

    export let icon = Circle;
    export let enableIcon = false

    export let color: string = 'gray';
    export let bgColor: string = 'bg-gray-700/30';
    export let textColor: string = 'text-gray-500';

    export let disabled = false;
    export let label: string;

    export let badge: string = ''

    if (color !== 'gray') {
        bgColor = getBgColor(color);
        textColor = getTextColor(color);
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