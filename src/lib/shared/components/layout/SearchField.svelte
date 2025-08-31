<script lang="ts">
	import { Search } from "lucide-svelte";

    export let searchTerm: string;
    export let placeholder: string;

</script>

<div class="relative">
    <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
    <input
        bind:value={searchTerm}
        type="text"
        placeholder={placeholder}
        class="w-full pl-10 pr-4 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
</div>