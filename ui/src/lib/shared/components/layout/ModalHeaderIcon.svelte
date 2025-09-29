<script lang="ts">
	import { createColorHelper } from '$lib/shared/utils/styling';
	import type { IconComponent } from '$lib/shared/utils/types';

	export let Icon: IconComponent; // Expects Svelte component, not string
	export let color: string;

	let colorHelper = createColorHelper(color);
</script>

<div class="p-2 {colorHelper.bg} rounded-lg">
	<Icon size={24} class={colorHelper.icon} />
</div>
