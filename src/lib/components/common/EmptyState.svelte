<script lang="ts">
	import { Plus, Search } from "lucide-svelte";

    export let IconComponent = Search
    export let title: string = ""
    export let subtitle: string = ""
    export let onClick: () => void = () => {};
    export let cta = ""
</script>

<div class="text-center py-12">
    <svelte:component this={IconComponent} size={16} class="w-16 h-16 text-gray-600 mx-auto mb-4" />
    <p class="text-gray-400 text-lg mb-4">{title}</p>
    <p class="text-gray-500 mb-6">{subtitle}</p>
    {#if cta.length > 0}
        <button
        on:click={onClick}
        class="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
        <Plus class="w-4 h-4" />
        {cta}
        </button>
    {/if}
</div>