<script lang="ts">
    // Shared component for displaying interface information
    // Used by both HostNode (inline) and InterfaceNode (standalone)
    
    interface InterfaceDisplayProps {
        interface: any; // Your Interface type
        compact?: boolean;
        showHostName?: boolean;
        hostName?: string;
    }
    
    let { 
        interface: iface, 
        compact = false, 
        showHostName = false, 
        hostName = '' 
    }: InterfaceDisplayProps = $props();
</script>

<div class="flex flex-col items-center {compact ? 'gap-0' : 'gap-1'}">
    <!-- IP Address -->
    <div class="{compact ? 'text-[0.5rem]' : 'text-xs'} font-medium">
        {iface?.name}: {iface?.ip_address || 'No IP'}
    </div>
    
    <!-- MAC Address (if not compact) -->
    {#if !compact}
        <div class="text-xs opacity-75 truncate max-w-full overflow-hidden text-ellipsis whitespace-nowrap">
            {iface?.mac_address ? iface.mac_address.slice(-8) : "No MAC"}
        </div>
    {/if}
        
    <!-- Host name reference (for standalone interface nodes) -->
    {#if showHostName && hostName}
        <div class="text-xs opacity-60 truncate max-w-full overflow-hidden text-ellipsis whitespace-nowrap">
            {hostName}
        </div>
    {/if}
</div>