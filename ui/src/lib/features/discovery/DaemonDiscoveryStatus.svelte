<script lang="ts">
	import { X, Loader2 } from 'lucide-svelte';
	import { cancelDiscovery, cancelling, sessions } from '$lib/features/discovery/store';
	import type { DiscoveryUpdatePayload } from '$lib/features/discovery/types/api';
	import type { Daemon } from '../daemons/types/base';
	import { getDaemonIsRunningDiscovery } from '../daemons/store';

	export let daemon: Daemon;
	export let discoveryData: DiscoveryUpdatePayload | null = null;

	$: isActive = getDaemonIsRunningDiscovery(daemon.id, $sessions);
	$: isCancelling = discoveryData?.session_id
		? $cancelling.get(discoveryData.session_id) === true
		: false;

	// Calculate progress across multiple subnets
	$: progressPercent = (() => {
		if (!isActive || !discoveryData) {
			return 0;
		}

		const progress =
			discoveryData.completed && discoveryData.total && discoveryData.total > 0
				? discoveryData.completed / discoveryData.total
				: 0;

		return Math.min(100, progress * 100);
	})();

	async function handleCancelDiscovery() {
		if (isActive && discoveryData?.session_id) {
			await cancelDiscovery(discoveryData.session_id);
		}
	}
</script>

{#if isActive && discoveryData}
	<!-- Active Discovery Status -->
	<div class="flex items-center justify-between gap-3">
		<div class="flex-1 space-y-2">
			<div class="flex items-center gap-3">
				<span class="text-sm font-medium text-blue-400"
					>{isCancelling ? 'Cancelling' : discoveryData.phase}</span
				>
				<span class="text-sm font-medium text-green-400"
					>{discoveryData.discovered_count} hosts found</span
				>
			</div>

			{#if discoveryData.total && discoveryData.total > 0}
				<div class="flex items-center gap-2">
					<div class="h-2 flex-1 overflow-hidden rounded-full bg-gray-700">
						<div
							class="h-full bg-blue-500 transition-all duration-300 ease-out"
							style="width: {progressPercent}%"
						></div>
					</div>
					<span class="text-xs text-gray-400">{Math.round(progressPercent)}%</span>
				</div>
			{/if}
		</div>

		<button
			class="rounded p-1 text-red-400 transition-colors hover:bg-gray-700 hover:text-red-300 disabled:cursor-not-allowed disabled:opacity-50"
			on:click={handleCancelDiscovery}
			title="Cancel Discovery"
		>
			{#if isCancelling}
				<Loader2 class="h-4 w-4 animate-spin" />
			{:else}
				<X class="h-4 w-4" />
			{/if}
		</button>
	</div>
{/if}
