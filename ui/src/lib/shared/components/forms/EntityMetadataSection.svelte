<script lang="ts">
	import type { Group } from '$lib/features/groups/types/base';
	import type { Subnet } from '$lib/features/subnets/types/base';
	import type { Host } from '$lib/features/hosts/types/base';
	import { formatId, formatTimestamp } from '$lib/shared/utils/formatting';
	import { Calendar, Clock, Hash, ChevronDown, ChevronRight } from 'lucide-svelte';

	export let id: string;
	export let createdAt: string;
	export let updatedAt: string;
	export let entity: Group | Host | Subnet | null = null;

	let isJsonExpanded = false;

	// Copy ID to clipboard
	async function copyId() {
		try {
			await navigator.clipboard.writeText(id);
		} catch (error) {
			console.warn('Failed to copy ID to clipboard:', error);
		}
	}

	// Copy JSON to clipboard
	async function copyJson() {
		if (!entity) return;
		try {
			await navigator.clipboard.writeText(JSON.stringify(entity, null, 2));
		} catch (error) {
			console.warn('Failed to copy JSON to clipboard:', error);
		}
	}

	function toggleJson() {
		isJsonExpanded = !isJsonExpanded;
	}
</script>

<div class="border-t border-gray-700 pt-6">
	<div class="rounded-lg bg-gray-800/50 p-4">
		<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
			<!-- ID -->
			<div class="flex items-center space-x-3">
				<div class="flex-shrink-0">
					<Hash class="h-5 w-5 text-gray-400" />
				</div>
				<div class="min-w-0 flex-1">
					<p class="text-sm font-medium text-gray-300">ID</p>
					<button
						type="button"
						class="block max-w-full cursor-pointer truncate font-mono text-sm text-gray-400 transition-colors hover:text-white"
						title={`${id} (Click to copy)`}
						on:click={copyId}
					>
						{formatId(id)}
					</button>
				</div>
			</div>

			<!-- Created -->
			<div class="flex items-center space-x-3">
				<div class="flex-shrink-0">
					<Calendar class="h-5 w-5 text-gray-400" />
				</div>
				<div class="min-w-0 flex-1">
					<p class="text-sm font-medium text-gray-300">Created</p>
					<p class="text-sm text-gray-400" title={createdAt}>
						{formatTimestamp(createdAt)}
					</p>
				</div>
			</div>

			<!-- Updated -->
			<div class="flex items-center space-x-3">
				<div class="flex-shrink-0">
					<Clock class="h-5 w-5 text-gray-400" />
				</div>
				<div class="min-w-0 flex-1">
					<p class="text-sm font-medium text-gray-300">Updated</p>
					<p class="text-sm text-gray-400" title={updatedAt}>
						{formatTimestamp(updatedAt)}
					</p>
				</div>
			</div>
		</div>

		<!-- JSON Entity Section -->
		{#if entity}
			<div class="mt-6 border-t border-gray-700 pt-4">
				<button
					type="button"
					class="flex w-full items-center space-x-2 text-left text-sm font-medium text-gray-300 transition-colors hover:text-white"
					on:click={toggleJson}
				>
					{#if isJsonExpanded}
						<ChevronDown class="h-4 w-4" />
					{:else}
						<ChevronRight class="h-4 w-4" />
					{/if}
					<span>JSON</span>
				</button>

				{#if isJsonExpanded}
					<div class="relative mt-3">
						<div class="absolute right-2 top-2 z-10">
							<button
								type="button"
								class="rounded border border-gray-600 bg-gray-900 px-2 py-1 text-xs text-gray-400 transition-colors hover:text-white"
								title="Copy JSON to clipboard"
								on:click={copyJson}
							>
								Copy
							</button>
						</div>
						<pre
							class="overflow-auto rounded-md border border-gray-600 bg-gray-900 p-4 font-mono text-sm text-gray-300"><code
								>{JSON.stringify(entity, null, 2)}</code
							></pre>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
