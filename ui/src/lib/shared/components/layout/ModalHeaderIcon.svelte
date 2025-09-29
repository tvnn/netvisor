<script lang="ts">
	import { createColorHelper } from "$lib/shared/utils/styling";

    export let icon: any;  // Expects Svelte component, not string
    export let color: string;

    let colorHelper = createColorHelper(color);
</script>

<div class="p-2 {colorHelper.bg} rounded-lg">
    <svelte:component this={icon} size={24} class={colorHelper.icon} />
</div>
