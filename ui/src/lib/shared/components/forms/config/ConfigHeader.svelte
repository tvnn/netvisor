<script lang="ts">
	export let title: string;
	export let subtitle: string | null = null;
</script>

<div class="border-b border-gray-600 pb-4">
	<h3 class="text-lg font-medium text-white">{title}</h3>
	{#if subtitle}
		<p class="text-sm text-gray-400">{subtitle}</p>
	{/if}
</div>
