<script lang="ts">
	import { type NodeProps } from '@xyflow/svelte';
	import { getHostFromId } from '$lib/features/hosts/store';
	import { serviceDefinitions } from '$lib/shared/stores/metadata';
	import { getServicesForHost } from '$lib/features/services/store';
	import { isContainerSubnet } from '$lib/features/subnets/store';
	import { topologyOptions } from '../store';
	import BaseChildNode from './BaseChildNode.svelte';
	import type { NodeRenderData } from '../types/base';
	import { get } from 'svelte/store';

	let { data, width, height }: NodeProps = $props();

	height = height ? height : 0;
	width = width ? width : 0;

	let hostStore = $derived(data.host_id ? getHostFromId(data.host_id as string) : null);
	let host = $derived(hostStore ? $hostStore : null);

	let servicesForHostStore = $derived(
		data.host_id ? getServicesForHost(data.host_id as string) : null
	);
	let servicesForHost = $derived(servicesForHostStore ? $servicesForHostStore : []);

	// Compute nodeData reactively
	let nodeData: NodeRenderData | null = $derived(
		host && data.host_id
			? (() => {
					const iface = host.interfaces.find((i) => i.id === data.interface_id);

					const servicesOnInterface = servicesForHost
						? servicesForHost.filter(
								(s) =>
									s.bindings.some(
										(b) => b.interface_id == null || (iface && b.interface_id == iface.id)
									) &&
									!$topologyOptions.request_options.hide_service_categories.includes(
										serviceDefinitions.getCategory(s.service_definition)
									)
							)
						: [];

					let bodyText: string | null = null;
					let footerText: string | null = null;
					let headerText: string | null = data.header ? (data.header as string) : null;
					let showServices = servicesOnInterface.length != 0;

					if (iface && !get(isContainerSubnet(iface?.subnet_id))) {
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
					} as NodeRenderData;
				})()
			: null
	);
</script>

<BaseChildNode {nodeData} {width} {height} />
