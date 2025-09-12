<script lang="ts">
    import { Handle, Position, type NodeProps } from '@xyflow/svelte';
    import { createColorHelper } from '$lib/shared/utils/styling';

    let { id, data, selected, width, height }: NodeProps = $props();

    const colorHelper = createColorHelper("blue");
    let nodeClasses = $derived(`
        ${colorHelper.bg} ${colorHelper.text} border-2 ${colorHelper.border} 
        rounded-lg p-3 text-xs font-medium text-center transition-all duration-200
        flex flex-col items-center justify-center shadow-md
        ${selected ? 'ring-2 ring-blue-400 ring-opacity-75' : ''}
        `);
    let nodeStyle = $derived(`width: ${width}px; height: ${height}px;`);
</script>

<div class={nodeClasses} style={nodeStyle}>
  <!-- Icon at top if available -->
  {#if data.IconComponent}
    <div class="mb-1">
      <svelte:component 
        this={data.IconComponent as any} 
        class="w-4 h-4 text-blue-400"
      />
    </div>
  {/if}
  
  <!-- Label -->
  <div class="font-semibold text-center whitespace-pre-line">
    {data.label}
  </div>
  
  <!-- Connection handles - hosts can connect from multiple sides -->
  <Handle type="target" position={Position.Left} class="w-2 h-2 !bg-blue-500 !border-blue-600" />
  <Handle type="target" position={Position.Top} class="w-2 h-2 !bg-blue-500 !border-blue-600" />
  <Handle type="source" position={Position.Right} class="w-2 h-2 !bg-blue-500 !border-blue-600" />
  <Handle type="source" position={Position.Bottom} class="w-2 h-2 !bg-blue-500 !border-blue-600" />
</div>

<style>
  /* Ensure proper text wrapping and overflow handling */
  div {
    word-wrap: break-word;
    overflow-wrap: break-word;
  }
</style>