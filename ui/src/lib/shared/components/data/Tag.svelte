<script lang="ts">
	import { createColorHelper } from '$lib/shared/utils/styling';
	import type { Component } from 'svelte';

	let {
		icon = null,
		color = 'gray',
		disabled = false,
		label,
		badge = ''
	}: {
		icon?: Component | null;
		color?: string;
		disabled?: boolean;
		label: string;
		badge?: string;
	} = $props();

	// Make colorHelper reactive to color changes
	let colorHelper = $derived(createColorHelper(color));
	let bgColor = $derived(colorHelper.bg);
	let textColor = $derived(colorHelper.text);
</script>

<div class="items-center space-x-2">
	{#if icon}
		<icon size={16} class={textColor}></icon>
	{/if}

	<!-- Main content -->
	<span
		class="inline-block {!disabled ? bgColor : 'bg-gray-700/30'} {!disabled
			? textColor
			: 'text-gray-500'} rounded px-2 py-1 text-xs"
	>
		{label}
		{#if badge.length > 0}
			<span class="text-xs {textColor}">{badge}</span>
		{/if}
	</span>
</div>
