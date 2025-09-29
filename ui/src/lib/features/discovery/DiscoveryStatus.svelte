<script lang="ts">
	import { daemons } from '../daemons/store';
	import { sessions } from './store';
	import { getDaemonDiscoveryState } from '../daemons/store';
	import RichSelect from '$lib/shared/components/forms/selection/RichSelect.svelte';
	import DaemonDiscoveryStatus from './DaemonDiscoveryStatus.svelte';
	import { get } from 'svelte/store';
	import { DaemonDisplay } from '$lib/shared/components/forms/selection/display/DaemonDisplay.svelte';

	let selectedDaemonId: string | null = null;
	$: discoveryData = getDaemonDiscoveryState(selectedDaemonId, get(sessions));
	$: selectedDaemon = $daemons.find((daemon) => daemon.id == selectedDaemonId);

	// Auto-select daemon logic: prioritize daemon with active session, fallback to first daemon
	$: if (!selectedDaemonId && $daemons.length > 0) {
		// First try to find a daemon that has an active session
		const daemonWithActiveSession = $daemons.find((daemon) => $sessions.get(daemon.id));

		if (daemonWithActiveSession) {
			selectedDaemonId = daemonWithActiveSession.id;
		} else {
			// Fallback to first daemon if no active sessions
			selectedDaemonId = $daemons[0].id;
		}
	}

	function handleDaemonSelect(daemonId: string) {
		selectedDaemonId = daemonId;
	}
</script>

<div class="flex items-center justify-end">
	{#if $daemons.length > 0 && selectedDaemonId == null}
		<div class="flex">
			<RichSelect
				selectedValue={selectedDaemonId}
				options={$daemons}
				placeholder="Select a daemon..."
				displayComponent={DaemonDisplay}
				onSelect={handleDaemonSelect}
			/>
		</div>
	{/if}

	<div class="flex">
		{#if selectedDaemon}
			<DaemonDiscoveryStatus daemon={selectedDaemon} {discoveryData} />
		{/if}
	</div>
	{#if $daemons.length == 0}
		<div class="flex items-center justify-center rounded-md border border-gray-600 bg-gray-800 p-3">
			<p class="text-gray-400">No daemons available for discovery.</p>
		</div>
	{/if}
</div>
