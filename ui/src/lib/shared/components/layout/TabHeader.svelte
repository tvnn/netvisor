<script lang="ts">
	import type { Component } from 'svelte';

	export let title: string;
	export let subtitle: string;
	export let buttons: {
		cta: string;
		onClick: () => void;
		disabled?: boolean;
		IconComponent?: Component;
	}[] = [];
</script>

<div class="flex items-center justify-between">
	<div>
		<h2 class="text-2xl font-bold text-white">{title}</h2>
		<p class="mt-1 text-gray-400">{subtitle}</p>
	</div>
	<div class="flex gap-4">
		{#each buttons as button (button)}
			<button
				on:click={button.onClick}
				disabled={button.disabled}
				class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700"
			>
				<svelte:component this={button.IconComponent} class="h-4 w-4" />
				{button.cta}
			</button>
		{/each}
	</div>
</div>
