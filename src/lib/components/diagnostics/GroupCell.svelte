<script lang="ts">
  export let groupName: string;
  export let groupId: string;
</script>

<div class="whitespace-nowrap">
  <div class="text-sm font-medium text-white">
    {groupName}
  </div>
  {#if groupId}
    <div class="text-xs text-gray-400 font-mono">
      {groupId.slice(0, 8)}
    </div>
  {/if}
</div>