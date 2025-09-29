<script lang="ts">
	export let totalStatValue: number = 0;
	export let totalStatLabel: string = '';
	export let goodStatValue: number = 0;
	export let goodStatLabel: string = '';
	export let badStatValue: number = 0;
	export let badStatLabel: string = '';
	export let infoStatValue: number = 0;
	export let infoStatLabel: string = '';
</script>

<div class="rounded-lg border border-gray-700 bg-gray-800 p-4">
	<div class="flex justify-evenly gap-4 text-center">
		{#if totalStatLabel.length > 0}
			<div>
				<div class="text-2xl font-bold text-white">{totalStatValue}</div>
				<div class="text-sm text-gray-400">{totalStatLabel}</div>
			</div>
		{/if}
		{#if goodStatLabel.length > 0}
			<div>
				<div class="text-2xl font-bold text-green-400">{goodStatValue}</div>
				<div class="text-sm text-gray-400">{goodStatLabel}</div>
			</div>
		{/if}
		{#if badStatLabel.length > 0}
			<div>
				<div class="text-2xl font-bold text-red-400">{badStatValue}</div>
				<div class="text-sm text-gray-400">{badStatLabel}</div>
			</div>
		{/if}
		{#if infoStatLabel.length > 0}
			<div>
				<div class="text-2xl font-bold text-blue-400">{infoStatValue}</div>
				<div class="text-sm text-gray-400">{infoStatLabel}</div>
			</div>
		{/if}
	</div>
</div>
