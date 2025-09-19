<script lang="ts">
    import { Handle, Position, type NodeProps } from '@xyflow/svelte';
    import { createColorHelper } from '$lib/shared/utils/styling';

    let { id, data, selected, width, height }: NodeProps = $props();

    let color = data.color as string
    const colorHelper = createColorHelper(color);

    let nodeClasses = $derived(`
        ${colorHelper.bg} ${colorHelper.text} 
        border-2 ${colorHelper.border} 
        rounded-xl p-4 text-sm font-semibold text-center 
        opacity-90
        transition-all duration-200
        ${selected ? `ring-2 ring-${color}-400 ring-opacity-75` : ''}
        flex flex-col items-center justify-center
        shadow-lg
        `.trim().replace(/\s+/g, ' '));
    let nodeStyle = $derived(`width: ${width}px; height: ${height}px;`);
</script>

<!-- Wrapper with relative positioning for absolute positioning of external label -->
<div class="relative" style={nodeStyle}>
  <!-- External label in upper left corner -->
  {#if data.label}
    <div class="absolute -top-8 left-0 flex items-center gap-1 bg-gray-800/90 backdrop-blur-sm px-2 py-1 rounded-md border border-gray-600 shadow-lg z-10">
      <!-- Icon -->
      {#if data.IconComponent}
        <svelte:component 
          this={data.IconComponent as any} 
          class={`w-3 h-3 text-${color}-400`}
        />
      {/if}
      
      <!-- Label -->
      <span class="text-xs font-medium text-gray-200 whitespace-nowrap">
        {data.label}
      </span>
    </div>
  {/if}

  <!-- Main container -->
  <div class={nodeClasses} style="width: 100%; height: 100%;">
    <!-- Subnet info if available -->
    {#if data.cidr}
      <div class="text-lg font-bold">
        {data.cidr}
      </div>
    {/if}
  </div>
</div>

<style>
  /* Ensure proper text wrapping and overflow handling */
  div {
    word-wrap: break-word;
    overflow-wrap: break-word;
  }
</style>