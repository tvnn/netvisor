<script lang="ts">
    export let title: string;
    export let subtitle: string | null = null;
    export let description: string | null = null;

</script>

<div class="border-b border-gray-600 pb-4">
    <h3 class="text-lg font-medium text-white">{title}</h3>
    {#if subtitle}
        <p class="text-sm text-gray-400">{subtitle}</p>
    {/if}
    {#if description}
        <p class="text-xs text-gray-500 mt-1">{description}</p>
    {/if}
</div>