<script lang="ts">
	export let title: string;
	export let subtitle: string | null = null;
</script>

<div class="border-b border-gray-600 pb-4">
	<h3 class="text-primary text-sm font-medium">{title}</h3>
	{#if subtitle}
		<p class="text-secondary text-sm">{subtitle}</p>
	{/if}
</div>
