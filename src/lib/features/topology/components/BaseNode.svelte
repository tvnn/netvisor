<script lang="ts">
    import { Handle, Position } from '@xyflow/svelte';
    import { createColorHelper } from '$lib/shared/utils/styling';

    interface BaseNodeProps {
        // Node dimensions and state
        width: number;
        height: number;
        selected: boolean;
        
        // Color scheme
        color: string;
        
        // Header section (optional)
        headerText?: string;
        headerIcon?: any; // Svelte component
        headerColor?: string;
        
        // Body section
        bodyText: string;
        
        // Footer section (optional)
        footerText?: string;
        footerColor?: string;
    }

    let { 
        width, 
        height, 
        selected, 
        color,
        headerText,
        headerIcon,
        headerColor = color,
        bodyText,
        footerText,
        footerColor = color,
    }: BaseNodeProps = $props();

    const colorHelper = createColorHelper(color);
    const headerColorHelper = createColorHelper(headerColor);
    const footerColorHelper = createColorHelper(footerColor);
    
    let nodeClasses = $derived(`
        ${colorHelper.bg} ${colorHelper.text} border-2 ${colorHelper.border} 
        rounded-lg text-xs font-medium transition-all duration-200
        shadow-md overflow-hidden
        ${selected ? `ring-2 ring-${color}-400 ring-opacity-75` : ''}
    `);
    let nodeStyle = $derived(`width: ${width}px; height: ${height}px; display: flex; flex-direction: column;`);
</script>

<div class={nodeClasses} style={nodeStyle}>
    <!-- Header section - only if headerText provided -->
    {#if headerText || headerIcon}
        <div class={`px-2 py-2 absolute top-0 left-0 ${headerColorHelper.text}`}>
            {#if headerIcon}
                <svelte:component 
                    this={headerIcon} 
                    class={`w-2.5 h-2.5 ${headerColorHelper.icon} flex-shrink-0`}
                />
            {/if}
            {#if headerText}
                <div class={`text-[0.5rem] font-medium truncate leading-none color: ${headerColorHelper.text}`}>
                    {headerText}
                </div>
            {/if}
        </div>
    {/if}
    
    <!-- Body section - main content -->
    <div class="p-3 mt-2 flex-1 flex items-center justify-center min-h-0 overflow-hidden">
        <div 
            class={`font-semibold text-center text-xs max-w-full`}
            style="line-height: 1.3; word-break: break-word; overflow-wrap: break-word;"
            title={bodyText}
        >
            {bodyText}
        </div>
    </div>
    
    <!-- Footer section - only if footerText provided -->
    {#if footerText}
        <div class={`px-2 py-1 ${footerColorHelper.bg} border-t ${footerColorHelper.border} flex items-center justify-center flex-shrink-0`}>
            <div class={`text-[0.5rem] font-medium leading-none truncate color: ${footerColorHelper.text}`}>
                {footerText}
            </div>
        </div>
    {/if}
    
    <!-- Connection handles - conditional -->
    <Handle type="target" position={Position.Left} class="hidden !bg-{color}-500 !border-{color}-600" />    
    <Handle type="source" position={Position.Right} class="hidden !bg-{color}-500 !border-{color}-600" />    
    <Handle type="target" position={Position.Top} class="hidden !bg-{color}-500 !border-{color}-600" />    
    <Handle type="source" position={Position.Bottom} class="hidden !bg-{color}-500 !border-{color}-600" />    
</div>