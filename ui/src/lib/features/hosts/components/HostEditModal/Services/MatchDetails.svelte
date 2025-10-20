<script lang="ts">
	import { matchConfidenceColor, type MatchReason, type MatchDetails } from '$lib/shared/types';
	import { ChevronDown, ChevronRight } from 'lucide-svelte';
	import Tag from '$lib/shared/components/data/Tag.svelte';

	let { details }: { details: MatchDetails } = $props();

	let isExpanded = $state(false);
	let confidenceColor = $derived(matchConfidenceColor(details.confidence));
</script>

{#snippet matchReasonNode(reason: MatchReason)}
	{#if reason.type === 'reason'}
		<div class="flex items-start gap-2 py-1 text-sm text-gray-300">
			<span class="select-none text-gray-500">•</span>
			<span>{reason.data}</span>
		</div>
	{:else if reason.type === 'container'}
		{@const label = reason.data[0]}
		{@const children = reason.data[1]}

		<div class="py-1">
			<div class="flex items-start gap-2 px-2 py-1">
				<span class="w-4 flex-shrink-0"></span>
				<span class="text-sm font-medium text-gray-200">{label}</span>
			</div>

			{#if children.length > 0}
				<div class="ml-6 mt-1 border-l-2 border-gray-700 pl-4">
					{#each children as child, index (index)}
						{@render matchReasonNode(child)}
					{/each}
				</div>
			{/if}
		</div>
	{/if}
{/snippet}

<div class="space-y-4">
	<button
		type="button"
		onclick={() => (isExpanded = !isExpanded)}
		class="flex w-full items-center justify-between text-left transition-colors hover:text-white"
	>
		<div class="flex items-center gap-2">
			{#if isExpanded}
				<ChevronDown class="h-4 w-4 text-gray-400" />
			{:else}
				<ChevronRight class="h-4 w-4 text-gray-400" />
			{/if}
			<h3 class="text-sm font-semibold text-gray-300">Match Details</h3>
		</div>
		<Tag label={details.confidence + ' Confidence'} color={confidenceColor} />
	</button>

	{#if isExpanded}
		<div class="pl-1">
			{@render matchReasonNode(details.reason)}
		</div>
	{/if}
</div>
