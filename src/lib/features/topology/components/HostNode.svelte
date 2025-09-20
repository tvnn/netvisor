<script lang="ts">
    import { Handle, Position, type NodeProps } from '@xyflow/svelte';
    import { createColorHelper } from '$lib/shared/utils/styling';
    import { getHostFromId, getHostTargetString } from '$lib/features/hosts/store';
    import { entities } from '$lib/shared/stores/registry';
    import { getServicesForHost } from '$lib/features/services/store';

    let { id, data, selected, width, height }: NodeProps = $props();

    let nodeData = $derived(
        data.host_id ? (() => {
            const host = getHostFromId(data.host_id as string);
            if (!host) return null;

            const iface = host.interfaces.find(i => i.id === data.interface_id);

            const servicesForHost = getServicesForHost(data.host_id as string);
            const servicesOnInterface = servicesForHost.filter(s => 
                s.interface_bindings.some(b => 
                    host.interfaces.map(i => i.id).includes(b)
                )
            );

            let bodyText: string | null = null;
            let headerText: string | null = null;
            let showServices = false;

            if (servicesOnInterface.length == 0 || (servicesOnInterface.length == 1 && servicesOnInterface[0].name == host.name)) {
                bodyText = host.name;
                showServices = false;
            } else {
                headerText = host.name;
                showServices = true;
            }

            const footerText = iface ? `${iface.name ? iface.name+': ' : ''}${iface.ip_address}` : ""

            return {
                footerText,
                services: servicesOnInterface,
                headerText,
                bodyText,
                showServices
            };
        })() : null
    );

    const colorHelper = entities.getColorHelper("Host");
    
    let nodeClasses = $derived(`
        ${colorHelper.bg} ${colorHelper.text} border-2 ${colorHelper.border} 
        rounded-lg text-xs font-medium transition-all duration-200
        shadow-md overflow-hidden
        ${selected ? `ring-2 ${colorHelper.ring} ring-opacity-75` : ''}
    `);
    let nodeStyle = $derived(`width: ${width}px; height: ${height}px; display: flex; flex-direction: column;`);
</script>

{#if nodeData}
    <div class={nodeClasses} style={`${nodeStyle} padding: 0;`}>
        <!-- Header section - only show if headerText exists -->
        {#if nodeData.headerText}
            <div class={`px-2 py-2 text-center ${colorHelper.text} flex-shrink-0 border-b ${colorHelper.border}`}>
                <div class={`text-xs font-medium truncate leading-none ${colorHelper.text}`}>
                    {nodeData.headerText}
                </div>
            </div>
        {/if}
        
        <!-- Body section - main content -->
        <div class="flex-1 flex flex-col justify-center items-center px-3">
            {#if nodeData.showServices}
                <!-- Show services list -->
                <div class="w-full space-y-1">
                    {#each nodeData.services as service}
                        <div 
                            class={`font-semibold text-left text-xs max-w-full truncate`}
                            style="line-height: 1.3;"
                            title={service.name}
                        >
                            • {service.name}
                        </div>
                    {/each}
                </div>
            {:else}
                <!-- Show host name as body text -->
                <div 
                    class={`font-semibold text-center text-xs max-w-full truncate`}
                    style="line-height: 1.3;"
                    title={nodeData.bodyText}
                >
                    {nodeData.bodyText}
                </div>
            {/if}
        </div>
        
        <!-- Footer section -->
        {#if nodeData.footerText}
            <div class={`px-2 py-2 ${colorHelper.bg} border-t ${colorHelper.border} flex items-center justify-center flex-shrink-0`}>
                <div class={`text-xs font-medium leading-none truncate ${colorHelper.text}`}>
                    {nodeData.footerText}
                </div>
            </div>
        {/if}
        
        <!-- Connection handles -->
        <Handle type="target" position={Position.Left} class={`hidden !bg-${colorHelper.string}-500 !border-${colorHelper.string}-600`} />    
        <Handle type="source" position={Position.Right} class={`hidden !bg-${colorHelper.string}-500 !border-${colorHelper.string}-600`} />    
        <Handle type="target" position={Position.Top} class={`hidden !bg-${colorHelper.string}-500 !border-${colorHelper.string}-600`} />    
        <Handle type="source" position={Position.Bottom} class={`hidden !bg-${colorHelper.string}-500 !border-${colorHelper.string}-600`} />    
    </div>
{/if}