<script lang="ts">
	import type { IconComponent } from '$lib/shared/utils/types';
	import { type Component } from 'svelte';

	export let title: string;
	export let subtitle: string;
	export let buttons: {
		cta?: string;
		onClick?: () => void;
		disabled?: boolean;
		IconComponent?: IconComponent;
		ButtonComponent?: Component;
	}[] = [];
</script>

<div class="flex items-center justify-between">
	<div>
		<h2 class="text-primary text-2xl font-bold">{title}</h2>
		<p class="text-secondary mt-1">{subtitle}</p>
	</div>
	<div class="flex gap-4">
		{#each buttons as button (button)}
			{#if button.ButtonComponent}
				<svelte:component this={button.ButtonComponent} />
			{:else}
				<button on:click={button.onClick} disabled={button.disabled} class="btn-primary">
					<svelte:component this={button.IconComponent} class="h-4 w-4" />
					{button.cta}
				</button>
			{/if}
		{/each}
	</div>
</div>
