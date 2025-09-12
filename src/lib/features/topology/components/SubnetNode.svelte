<script lang="ts">
    import { Handle, Position, type NodeProps } from '@xyflow/svelte';
    import { createColorHelper } from '$lib/shared/utils/styling';

    let { id, data, selected, width, height }: NodeProps = $props();

    const colorHelper = createColorHelper("orange");

    let nodeClasses = $derived(`
        ${colorHelper.bg} ${colorHelper.text} 
        border-2 ${colorHelper.border} 
        rounded-xl p-4 text-sm font-semibold text-center 
        opacity-90 min-w-[200px] min-h-[150px]
        transition-all duration-200
        ${selected ? 'ring-2 ring-orange-400 ring-opacity-75' : ''}
        flex flex-col items-center justify-center
        shadow-lg
        `.trim().replace(/\s+/g, ' '));
    let nodeStyle = $derived(`width: ${width}px; height: ${height}px;`);
</script>

<div class={nodeClasses} style={nodeStyle}>
  <!-- Icon at top if available -->
  {#if data.IconComponent}
    <div class="mb-2">
      <svelte:component 
        this={data.IconComponent as any} 
        class="w-6 h-6 text-orange-400"
      />
    </div>
  {/if}
  
  <!-- Label -->
  <div class="font-bold text-center whitespace-pre-line">
    {data.label}
  </div>
  
  <!-- Subnet info if available -->
  {#if data.cidr}
    <div class="text-xs mt-1 opacity-75">
      {data.cidr}
    </div>
  {/if}
</div>

<style>
  /* Ensure proper text wrapping and overflow handling */
  div {
    word-wrap: break-word;
    overflow-wrap: break-word;
  }
</style>