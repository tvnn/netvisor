<script lang="ts">
	import { createColorHelper } from '$lib/shared/utils/styling';
	import type { Component } from 'svelte';

	export let icon: Component | null = null;

	export let color: string = 'gray';
	const colorHelper = createColorHelper(color);

	export let bgColor: string = colorHelper.bg;
	export let textColor: string = colorHelper.text;

	export let disabled = false;
	export let label: string;

	export let badge: string = '';
</script>

<div class="items-center space-x-2">
	{#if icon}
		<svelte:component this={icon} size={16} class={textColor} />
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
