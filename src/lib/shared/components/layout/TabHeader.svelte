<script lang="ts">
    export let title: string;
    export let subtitle: string;
    export let buttons: any[];
</script>

<div class="flex items-center justify-between">
    <div>
        <h2 class="text-2xl font-bold text-white">{title}</h2>
        <p class="text-gray-400 mt-1">{subtitle}</p>
    </div>
    <div class="flex gap-4">
        {#each buttons as button}
            <button
                on:click={button.onClick}
                disabled={button.disabled}
                class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
            >
                <svelte:component this={button.IconComponent} class="w-4 h-4" />
                {button.cta}
            </button>
        {/each}
    </div>
</div>