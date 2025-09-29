<script lang="ts">
	import { Plus, Search } from 'lucide-svelte';

	export let IconComponent = Search;
	export let title: string = '';
	export let subtitle: string = '';
	export let onClick: () => void = () => {};
	export let cta = '';
</script>

<div class="py-12 text-center">
	<svelte:component this={IconComponent} size={16} class="mx-auto mb-4 h-16 w-16 text-gray-600" />
	<p class="mb-4 text-lg text-gray-400">{title}</p>
	<p class="mb-6 text-gray-500">{subtitle}</p>
	{#if cta.length > 0}
		<button
			on:click={onClick}
			class="inline-flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700"
		>
			<Plus class="h-4 w-4" />
			{cta}
		</button>
	{/if}
</div>
