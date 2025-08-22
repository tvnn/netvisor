<script lang="ts">
	import { Plus } from "lucide-svelte";

    export let title: string;
    export let subtitle: string;
    export let onClick: () => void;
    export let IconComponent = Plus;
    export let cta = '';
    export let ctaDisabled: boolean = false;
</script>

<div class="flex items-center justify-between">
    <div>
        <h2 class="text-2xl font-bold text-white">{title}</h2>
        <p class="text-gray-400 mt-1">{subtitle}</p>
    </div>
    <button
        on:click={onClick}
        disabled={ctaDisabled}
        class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
    >
        <svelte:component this={IconComponent} class="w-4 h-4" />
        {cta}
    </button>
</div>