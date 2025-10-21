<script lang="ts">
	import { Plus, Search } from 'lucide-svelte';

	export let IconComponent = Search;
	export let title: string = '';
	export let subtitle: string = '';
	export let onClick: () => void = () => {};
	export let cta = '';
</script>

<div class="py-12 text-center">
	<svelte:component this={IconComponent} size={16} class="text-secondary mx-auto mb-4 h-16 w-16" />
	<p class="text-secondary mb-4 text-lg">{title}</p>
	<p class="text-tertiary mb-6">{subtitle}</p>
	{#if cta.length > 0}
		<button on:click={onClick} class="btn-primary">
			<Plus class="h-4 w-4" />
			{cta}
		</button>
	{/if}
</div>
