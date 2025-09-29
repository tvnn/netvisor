<script lang="ts">
	export let message: string = 'Loading...';
</script>

<div class="py-8 text-center">
	<div class="mx-auto h-8 w-8 animate-spin rounded-full border-b-2 border-blue-500"></div>
	<p class="mt-2 text-gray-400">{message}</p>
</div>
