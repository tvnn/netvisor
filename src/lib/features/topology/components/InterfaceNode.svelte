<script lang="ts">
    import { Handle, Position, type NodeProps } from '@xyflow/svelte';
    import { createColorHelper } from '$lib/shared/utils/styling';
    import { getInterfaceFromId } from '$lib/features/hosts/store';
	import BaseNode from './BaseNode.svelte';

    let { id, data, selected, width, height }: NodeProps = $props();

    let color = "purple";
    let iface = getInterfaceFromId(id);
    let bodyText = `${iface?.name}: ${iface?.ip_address}`;
    let footerText = `${iface?.mac_address ? iface.mac_address : 'No MAC'}`;
</script>

<BaseNode 
    width={width as number}
    height={height as number}
    {selected}
    color="purple"
    {bodyText}
    {footerText}
    footerColor="purple"
/>