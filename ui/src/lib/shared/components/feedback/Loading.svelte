<script lang="ts">
  export let message: string = "Loading...";
</script>

<div class="text-center py-8">
    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"></div>
    <p class="text-gray-400 mt-2">{message}</p>
</div>