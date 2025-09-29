<script lang="ts">
	import { Handle, Position, type NodeProps } from '@xyflow/svelte';
	import { createColorHelper } from '$lib/shared/utils/styling';
	import { getHostFromId, getHostTargetString } from '$lib/features/hosts/store';
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
	import { getServicesForHost } from '$lib/features/services/store';
	import { isContainerSubnet } from '$lib/features/subnets/store';

	let { id, data, selected, width, height }: NodeProps = $props();

	let nodeData = $derived(
		data.host_id
			? (() => {
					const host = getHostFromId(data.host_id as string);
					if (!host) return null;

					const iface = host.interfaces.find((i) => i.id === data.interface_id);

					const servicesForHost = getServicesForHost(data.host_id as string);
					const servicesOnInterface = servicesForHost.filter((s) =>
						s.interface_bindings.some((b) => iface && b == iface.id)
					);

					let bodyText: string | null = null;
					let headerText: string | null = null;
					let footerText: string | null = null;
					let showServices = servicesOnInterface.length != 0;

					if (
						(servicesOnInterface.length > 0 && servicesOnInterface[0].name == host.name) ||
						host.name.includes('Unknown Device')
					) {
					} else {
						headerText = host.name;
					}

					if (iface && !isContainerSubnet(iface?.subnet_id)) {
						footerText = (iface.name ? iface.name + ': ' : '') + iface.ip_address;
					}

					return {
						footerText,
						services: servicesOnInterface,
						headerText,
						bodyText,
						showServices
					};
				})()
			: null
	);

	const colorHelper = entities.getColorHelper('Host');

	let nodeClasses = $derived(`
        ${colorHelper.bg} ${colorHelper.text} border-2 ${colorHelper.border} 
        rounded-lg text-xs font-medium transition-all duration-200
        shadow-md overflow-hidden
        ${selected ? `ring-2 ${colorHelper.ring} ring-opacity-75` : ''}
    `);
	let nodeStyle = $derived(
		`width: ${width}px; height: ${height}px; display: flex; flex-direction: column;`
	);
</script>

{#if nodeData}
	<div class={nodeClasses} style={`${nodeStyle} padding: 0;`}>
		<!-- Header section - only show if headerText exists -->
		{#if nodeData.headerText}
			<div
				class={`px-2 py-2 text-center ${colorHelper.text} flex-shrink-0 border-b ${colorHelper.border}`}
			>
				<div class={`truncate text-xs font-medium leading-none ${colorHelper.text}`}>
					{nodeData.headerText}
				</div>
			</div>
		{/if}

		<!-- Body section - main content -->
		<div class="flex flex-1 flex-col items-center justify-center px-3">
			{#if nodeData.showServices}
				<!-- Show services list -->
				<div class="flex w-full flex-col items-center space-y-1">
					{#each nodeData.services as service}
						{@const ServiceIcon = serviceDefinitions.getIconComponent(service.service_definition)}
						<div
							class={`flex max-w-full items-center justify-center gap-1 truncate text-center text-xs`}
							style="line-height: 1.3;"
							title={service.name}
						>
							<ServiceIcon class="h-3 w-3 flex-shrink-0 {colorHelper.icon}" />
							<span class="truncate">{service.name}</span>
						</div>
					{/each}
				</div>
			{:else}
				<!-- Show host name as body text -->
				<div class={`truncate text-center text-xs leading-none`} title={nodeData.bodyText}>
					{nodeData.bodyText}
				</div>
			{/if}
		</div>

		<!-- Footer section -->
		{#if nodeData.footerText}
			<div
				class={`border-t px-2 py-2 ${colorHelper.border} flex flex-shrink-0 items-center justify-center`}
			>
				<div class={`truncate text-xs font-medium leading-none ${colorHelper.text}`}>
					{nodeData.footerText}
				</div>
			</div>
		{/if}

		<!-- Connection handles remain the same -->
		<Handle type="target" id="Top" position={Position.Top} style={`opacity: 0`} />
		<Handle type="target" id="Right" position={Position.Right} style={`opacity: 0`} />
		<Handle type="target" id="Bottom" position={Position.Bottom} style={`opacity: 0`} />
		<Handle type="target" id="Left" position={Position.Left} style={`opacity: 0`} />

		<Handle type="source" id="Top" position={Position.Top} style={`opacity: 0`} />
		<Handle type="source" id="Right" position={Position.Right} style={`opacity: 0`} />
		<Handle type="source" id="Bottom" position={Position.Bottom} style={`opacity: 0`} />
		<Handle type="source" id="Left" position={Position.Left} style={`opacity: 0`} />
	</div>
{/if}
