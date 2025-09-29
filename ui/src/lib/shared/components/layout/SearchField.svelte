<script lang="ts">
	import { Search } from 'lucide-svelte';

	export let searchTerm: string;
	export let placeholder: string;
</script>

<div class="relative">
	<Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 transform text-gray-400" />
	<input
		bind:value={searchTerm}
		type="text"
		{placeholder}
		class="w-full rounded-lg border border-gray-700 bg-gray-800 py-2 pl-10 pr-4 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
	/>
</div>
