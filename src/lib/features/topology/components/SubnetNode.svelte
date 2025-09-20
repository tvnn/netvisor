<script lang="ts">
    import { Handle, Position, type NodeProps } from '@xyflow/svelte';
    import { createColorHelper } from '$lib/shared/utils/styling';
	import { Network } from 'lucide-svelte';
	import { entities } from '$lib/shared/stores/registry';
	import { getSubnetFromId, getSubnets } from '$lib/features/subnets/store';

    let { id, data, selected, width, height }: NodeProps = $props();

    const subnetColorHelper = entities.getColorHelper("Subnet");
    const grayColorHelper = createColorHelper("gray");
    let IconComponent = entities.getIconComponent("Subnet")
    let subnet = getSubnetFromId(id);
    let cidr = subnet?.cidr
    let infra_width = (data.infra_width as number) || 0;

    let nodeClasses = $derived(`
        ${grayColorHelper.bg} ${grayColorHelper.text} 
        border-2 ${grayColorHelper.border} 
        rounded-xl text-sm font-semibold text-center 
        opacity-90
        transition-all duration-200
        ${selected ? `ring-2 ${subnetColorHelper.ring} ring-opacity-75` : ''}
        shadow-lg
        `.trim().replace(/\s+/g, ' '));
    
    let infraClasses = $derived(`
        ${grayColorHelper.bg}
        opacity-80
        `.trim().replace(/\s+/g, ' '));
        
    let nodeStyle = $derived(`width: ${width}px; height: ${height}px;`);
    let infraStyle = $derived(`width: ${infra_width}px; height: 100%;`);
    let hasInfra = $derived(infra_width > 0);
</script>

<!-- Wrapper with relative positioning for absolute positioning of external label -->
<div class="relative" style={nodeStyle}>
  <!-- External label in upper left corner -->
  {#if cidr}
    <div class="absolute -top-8 left-0 flex items-center gap-1 bg-gray-800/90 backdrop-blur-sm px-2 py-1 rounded-md border border-gray-600 shadow-lg z-10">
      <!-- Icon -->
      {#if IconComponent}
        <svelte:component this={IconComponent} class={`w-3 h-3 ${subnetColorHelper.icon}`} />
      {/if}
      
      <!-- Label -->
      <span class="text-xs font-medium text-gray-200 whitespace-nowrap">
        {cidr}
      </span>
    </div>
  {/if}

  <!-- Main container -->
  <div class={nodeClasses} style="width: 100%; height: 100%; position: relative; overflow: hidden;">
    <!-- Infrastructure background area -->
    {#if hasInfra}
      <div 
        class={infraClasses} 
        style={`${infraStyle} position: absolute; top: 0; left: 0; border-radius: 0.75rem 0 0 0.75rem;`}
      >
        <!-- Infrastructure title -->
        <div class="absolute top-0.5 left-1/2 transform -translate-x-1/2 text-[0.5rem] font-semibold {grayColorHelper.text}">
          Infrastructure
        </div>
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