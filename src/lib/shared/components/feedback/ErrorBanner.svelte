<script lang="ts">
    export let error: string | null = "";
    export let onClear: () => void;
</script>

{#if error}
    <div class="bg-red-600/20 border border-red-600/50 rounded-lg p-4">
        <p class="text-red-300">{error}</p>
        <button
        on:click={onClear}
        class="text-red-400 hover:text-red-300 text-sm mt-2"
        >
        Dismiss
        </button>
    </div>
{/if}