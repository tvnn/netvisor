<script lang="ts">
	import { clearError, errorStore } from "$lib/shared/stores/feedback";
	import { get } from "svelte/store";

    $: errors = get(errorStore)

</script>

{#if errors.length > 0}
    {#each errors as error}
        <div class="bg-red-600/20 border border-red-600/50 rounded-lg p-4">
            <p class="text-red-300">{error}</p>
            <button
            on:click={() => clearError(error)}
            class="text-red-400 hover:text-red-300 text-sm mt-2"
            >
            Dismiss
            </button>
        </div>
    {/each}
{/if}