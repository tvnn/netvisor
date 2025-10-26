<script lang="ts">
	import { serviceDefinitions } from '$lib/shared/stores/metadata';
	import { onMount } from 'svelte';

	export let topologyOption: string[];
	export let onChange: (event: Event) => void;
	export let title: string;
	export let description: string;

	// Get unique service categories
	let serviceCategories: string[] = [];

	onMount(() => {
		const items = serviceDefinitions.getItems() || [];
		const categoriesSet = new Set(items.map((i) => serviceDefinitions.getCategory(i.id)));
		serviceCategories = Array.from(categoriesSet)
			.filter((c) => c)
			.sort();
	});
</script>

<div class="space-y-1.5">
	<label for="infra-categories" class="text-primary block text-sm font-medium">
		{title}
	</label>
	<p class="text-tertiary text-xs leading-tight">
		{description}
	</p>
	<select
		id="infra-categories"
		multiple
		on:change={onChange}
		class="text-primary w-full rounded-md border border-gray-600 bg-gray-700 px-2 py-1.5 text-xs focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
	>
		{#each serviceCategories as category (category)}
			<option value={category} selected={topologyOption.includes(category)}>
				{category}
			</option>
		{/each}
	</select>
</div>

<style>
	/* Style multi-select options */
	select[multiple] option {
		padding: 0.25rem 0.5rem;
		cursor: pointer;
	}

	select[multiple] option:checked {
		background-color: rgb(37, 99, 235);
		color: white;
	}

	/* Remove default select styling for multi-select */
	select[multiple] {
		scrollbar-width: thin;
		scrollbar-color: #4b5563 #1f2937;
	}

	select[multiple]::-webkit-scrollbar {
		width: 8px;
	}

	select[multiple]::-webkit-scrollbar-track {
		background: #1f2937;
	}

	select[multiple]::-webkit-scrollbar-thumb {
		background-color: #4b5563;
		border-radius: 4px;
	}
</style>
