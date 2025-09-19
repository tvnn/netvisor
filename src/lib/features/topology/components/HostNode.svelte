<script lang="ts">
    import { type NodeProps } from '@xyflow/svelte';
    import { getHostFromId } from '$lib/features/hosts/store';
    import BaseNode from './BaseNode.svelte';

    let { id, data, selected, width, height }: NodeProps = $props();

    let host = getHostFromId(id);
    let primaryInterface = host?.interfaces.find(iface => iface.is_primary);
    let primaryService = host?.services && host.services.length > 0 ? host.services[0] : null;

    // Prepare props for BaseNode
    let footerText = primaryInterface ? `${primaryInterface.name ? primaryInterface.name + ": " : ""}${primaryInterface.ip_address || 'No IP'}` : undefined;
</script>

<BaseNode 
    width={width as number}
    height={height as number}
    {selected}
    color="blue"
    headerIcon={primaryService ? data.IconComponent : undefined}
    bodyText={data.label as string}
    {footerText}
    footerColor="purple"
/>