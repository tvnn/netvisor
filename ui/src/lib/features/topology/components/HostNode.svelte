<script lang="ts">
	import { Handle, Position, type NodeProps } from '@xyflow/svelte';
	import { getHostFromId, getHostIsVirtualized } from '$lib/features/hosts/store';
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
	import { getServicesForHost } from '$lib/features/services/store';
	import { isContainerSubnet } from '$lib/features/subnets/store';
	import { twColorToRgba } from '$lib/shared/utils/styling';

	let { data, selected, width, height }: NodeProps = $props();

	height = height ? height : 0;
	width = width ? width : 0;

	let nodeData = $derived(
		data.host_id
			? (() => {
					const host = getHostFromId(data.host_id as string);
					if (!host) return null;

					const iface = host.interfaces.find((i) => i.id === data.interface_id);

					const servicesForHost = getServicesForHost(data.host_id as string);
					const servicesOnInterface = servicesForHost.filter((s) =>
						s.bindings.some((b) => b.interface_id == null || (iface && b.interface_id == iface.id))
					);

					let bodyText: string | null = null;
					let footerText: string | null = null;
					let showServices = servicesOnInterface.length != 0;

					if (iface && !isContainerSubnet(iface?.subnet_id)) {
						footerText = (iface.name ? iface.name + ': ' : '') + iface.ip_address;
					}

					return {
						footerText,
						services: servicesOnInterface,
						headerText: data.header,
						bodyText,
						showServices,
						isVirtualized: getHostIsVirtualized(host.id)
					};
				})()
			: null
	);

	// const hostColorHelper = entities.getColorHelper('Host');
	// const virtualizationColorHelper = entities.getColorHelper('Virtualization');

	// const headerColorHelper = $derived(
	// 	!nodeData?.isVirtualized ? hostColorHelper : virtualizationColorHelper
	// );

	// let nodeClasses = $derived(`
	//     border-2 ${hostColorHelper.border} ${hostColorHelper.text}
	//     rounded-lg text-s font-medium transition-all duration-200
	//     shadow-md overflow-hidden
	//     ${selected ? `ring-2 ${hostColorHelper.ring} ring-opacity-75` : ''}
	// `);
	// let nodeStyle = $derived(
	// 	`width: ${width}px; height: ${height}px; display: flex; flex-direction: column;
	// 	background: linear-gradient(to bottom,
	// 		${twColorToRgba(headerColorHelper.bg)} 0%,
	// 		color-mix(in srgb, ${twColorToRgba(hostColorHelper.bg)} 15%, ${twColorToRgba(headerColorHelper.bg)} 85%) 30%,
	// 		${twColorToRgba(hostColorHelper.bg)} 70%,
	// 		color-mix(in srgb, ${twColorToRgba(hostColorHelper.bg)} 75%, black) ${height}px);`
	// );

	const hostColorHelper = entities.getColorHelper('Host');
	const virtualizationColorHelper = entities.getColorHelper('Virtualization');

	const headerColorHelper = $derived(
		!nodeData?.isVirtualized ? hostColorHelper : virtualizationColorHelper
	);

	let nodeClasses = $derived(`
		border-2 ${hostColorHelper.border} ${hostColorHelper.text}
		rounded-lg text-s font-medium transition-all duration-200
		shadow-md overflow-hidden
		${selected ? `ring-2 ${hostColorHelper.ring} ring-opacity-75` : ''}
	`);

	let nodeStyle = $state('');

	$effect(() => {
		const isVirtualized = !!nodeData?.isVirtualized;

		const topBlendHeight = 50;
		const bottomBlendHeight = 50;

		const hostBg = twColorToRgba(hostColorHelper.bg);
		const headerBg = twColorToRgba(headerColorHelper.bg);

		nodeStyle = `
			width: ${width}px;
			height: ${height}px;
			display: flex;
			flex-direction: column;
			background: linear-gradient(
				to bottom,
				${isVirtualized ? `${headerBg} ` : `color-mix(in srgb, ${hostBg} 97%, white) `} 0px,
				${isVirtualized ? `${headerBg} ` : `color-mix(in srgb, ${hostBg} 97%, white) `} ${topBlendHeight / 2}px,
				${hostBg} ${topBlendHeight}px,
				${hostBg} ${height - bottomBlendHeight}px,
				color-mix(in srgb, ${hostBg} 80%, black) ${height}px
			);
		`;
	});
</script>

{#if nodeData}
	<div class={nodeClasses} style={`${nodeStyle} padding: 0;`}>
		<!-- Header section with gradient transition to body -->
		{#if nodeData.headerText}
			<div class="relative flex-shrink-0 px-2 py-2 text-center">
				<div class={`truncate text-xs font-medium leading-none ${hostColorHelper.text}`}>
					{nodeData.headerText}
				</div>
			</div>
		{/if}

		<!-- Body section -->
		<div
			class="flex flex-col items-center justify-around px-3 py-2"
			style="flex: 1 1 0; min-height: 0;"
		>
			{#if nodeData.showServices}
				<!-- Show services list -->
				<div class="flex w-full flex-1 flex-col items-center justify-evenly">
					{#each nodeData.services as service (service.id)}
						{@const ServiceIcon = serviceDefinitions.getIconComponent(service.service_definition)}
						<div
							class="text-m flex max-w-full items-center justify-center gap-1 truncate text-center"
							style="line-height: 1.3;"
							title={service.name}
						>
							<ServiceIcon class="mr-1 h-5 w-5 flex-shrink-0 {hostColorHelper.icon}" />
							<span class="truncate">{service.name}</span>
						</div>
					{/each}
				</div>
			{:else}
				<!-- Show host name as body text -->
				<div class="truncate text-center text-xs leading-none" title={nodeData.bodyText}>
					{nodeData.bodyText}
				</div>
			{/if}
		</div>

		<!-- Footer section with gradient transition from body -->
		{#if nodeData.footerText}
			<div class="relative flex flex-shrink-0 items-center justify-center px-2 py-2">
				<div class={`truncate text-xs font-medium leading-none ${hostColorHelper.text}`}>
					{nodeData.footerText}
				</div>
			</div>
		{/if}

		<!-- Connection handles remain the same -->
		<Handle type="target" id="Top" position={Position.Top} style="opacity: 0" />
		<Handle type="target" id="Right" position={Position.Right} style="opacity: 0" />
		<Handle type="target" id="Bottom" position={Position.Bottom} style="opacity: 0" />
		<Handle type="target" id="Left" position={Position.Left} style="opacity: 0" />

		<Handle type="source" id="Top" position={Position.Top} style="opacity: 0" />
		<Handle type="source" id="Right" position={Position.Right} style="opacity: 0" />
		<Handle type="source" id="Bottom" position={Position.Bottom} style="opacity: 0" />
		<Handle type="source" id="Left" position={Position.Left} style="opacity: 0" />
	</div>
{/if}
