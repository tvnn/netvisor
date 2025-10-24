<script lang="ts">
	import { type NodeProps } from '@xyflow/svelte';
	import { getHostFromId } from '$lib/features/hosts/store';
	import { serviceDefinitions } from '$lib/shared/stores/metadata';
	import { getServicesForHost } from '$lib/features/services/store';
	import { isContainerSubnet } from '$lib/features/subnets/store';
	import { get } from 'svelte/store';
	import { topologyOptions } from '../store';
	import BaseChildNode from './BaseChildNode.svelte';
	import type { NodeRenderData } from '../types/base';

	let { data, width, height }: NodeProps = $props();

	height = height ? height : 0;
	width = width ? width : 0;

	let nodeData: NodeRenderData | null = $derived(
		data.host_id
			? (() => {
					const host = getHostFromId(data.host_id as string);
					if (!host) return null;

					const iface = host.interfaces.find((i) => i.id === data.interface_id);

					const servicesForHost = getServicesForHost(data.host_id as string);
					const servicesOnInterface = servicesForHost.filter(
						(s) =>
							s.bindings.some(
								(b) => b.interface_id == null || (iface && b.interface_id == iface.id)
							) &&
							!get(topologyOptions).hide_service_categories.includes(
								serviceDefinitions.getCategory(s.service_definition)
							)
					);

					let bodyText: string | null = null;
					let footerText: string | null = null;
					let headerText: string | null = data.header ? (data.header as string) : null;
					let showServices = servicesOnInterface.length != 0;

					if (iface && !isContainerSubnet(iface?.subnet_id)) {
						footerText = (iface.name ? iface.name + ': ' : '') + iface.ip_address;
					}

					if (servicesOnInterface.length == 0) {
						bodyText = host.name;
					}

					return {
						footerText,
						services: servicesOnInterface,
						headerText,
						bodyText,
						showServices,
						isVirtualized: host.virtualization !== null
					};
				})()
			: null
	);
</script>

<BaseChildNode {nodeData} {width} {height} />
