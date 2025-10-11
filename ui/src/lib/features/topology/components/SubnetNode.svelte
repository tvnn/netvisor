<script lang="ts">
	import { NodeResizeControl, type NodeProps } from '@xyflow/svelte';
	import { createColorHelper } from '$lib/shared/utils/styling';
	import { subnetTypes } from '$lib/shared/stores/metadata';
	import {
		createEmptySubnetFormData,
		getSubnetFromId,
		isContainerSubnet
	} from '$lib/features/subnets/store';

	let { id, data, selected, width, height }: NodeProps = $props();

	let subnet = getSubnetFromId(id) || createEmptySubnetFormData();

	const subnetColorHelper = subnetTypes.getColorHelper(subnet.subnet_type);
	const grayColorHelper = createColorHelper('gray');
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	let IconComponent = subnetTypes.getIconComponent(subnet.subnet_type) as any;
	let cidr = subnet.cidr;

	let label_override = data.label_override as string | null;

	let label = label_override
		? label_override
		: (subnet.name != subnet.cidr ? subnet.name : subnetTypes.getName(subnet.subnet_type)) +
			(isContainerSubnet(subnet.id) ? '' : ': ' + subnet.cidr);
	let infra_width = (data.infra_width as number) || 0;

	let nodeClasses = $derived(
		`
        ${grayColorHelper.bg} ${grayColorHelper.text} 
        border-2 ${selected ? subnetColorHelper.border : grayColorHelper.border} 
        rounded-xl text-sm font-semibold text-center 
        transition-all duration-200
        shadow-lg
        `
			.trim()
			.replace(/\s+/g, ' ')
	);

	let infraClasses = $derived(
		`
        ${grayColorHelper.bg}
        opacity-50
        `
			.trim()
			.replace(/\s+/g, ' ')
	);

	let nodeStyle = $derived(`width: ${width}px; height: ${height}px;`);
	let infraStyle = $derived(`width: ${infra_width}px; height: 100%;`);
	let hasInfra = $derived(infra_width > 0);
</script>

<NodeResizeControl
	position="bottom-right"
	style="z-index: 100; border: none; width: 20px; height: 20px;"
>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		width="20"
		height="20"
		viewBox="0 0 20 20"
		style="position: absolute; right: 10px; bottom: 10px;"
	>
		<path
			d="M20 7.5 L20 20 L7.5 20 Z"
			fill={selected ? subnetColorHelper.rgb : grayColorHelper.rgb}
			style="transition: fill 200ms ease-in-out;"
		/>
		<line x1="11.667" y1="20" x2="20" y2="11.667" stroke="#374151" stroke-width="1" />
		<line x1="16.333" y1="20" x2="20" y2="16.333" stroke="#374151" stroke-width="1" />
	</svg>
</NodeResizeControl>

<NodeResizeControl
	position="top-left"
	style="z-index: 100; border: none; width: 20px; height: 20px;"
>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		width="20"
		height="20"
		viewBox="0 0 20 20"
		style="position: absolute; left: 10px; top: 10px;"
	>
		<path
			d="M0 12.5 L0 0 L12.5 0 Z"
			fill={selected ? subnetColorHelper.rgb : grayColorHelper.rgb}
			style="transition: fill 200ms ease-in-out;"
		/>
		<line x1="8.333" y1="0" x2="0" y2="8.333" stroke="#374151" stroke-width="1" />
		<line x1="3.667" y1="0" x2="0" y2="3.667" stroke="#374151" stroke-width="1" />
	</svg>
</NodeResizeControl>

<NodeResizeControl
	position="top-right"
	style="z-index: 100; border: none; width: 20px; height: 20px;"
>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		width="20"
		height="20"
		viewBox="0 0 20 20"
		style="position: absolute; right: 10px; top: 10px;"
	>
		<path
			d="M7.5 0 L20 0 L20 12.5 Z"
			fill={selected ? subnetColorHelper.rgb : grayColorHelper.rgb}
			style="transition: fill 200ms ease-in-out;"
		/>
		<line x1="11.667" y1="0" x2="20" y2="8.333" stroke="#374151" stroke-width="1" />
		<line x1="16.333" y1="0" x2="20" y2="3.667" stroke="#374151" stroke-width="1" />
	</svg>
</NodeResizeControl>

<NodeResizeControl
	position="bottom-left"
	style="z-index: 100; border: none; width: 20px; height: 20px;"
>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		width="20"
		height="20"
		viewBox="0 0 20 20"
		style="position: absolute; left: 10px; bottom: 10px;"
	>
		<path
			d="M0 7.5 L12.5 20 L0 20 Z"
			fill={selected ? subnetColorHelper.rgb : grayColorHelper.rgb}
			style="transition: fill 200ms ease-in-out;"
		/>
		<line x1="0" y1="11.667" x2="8.333" y2="20" stroke="#374151" stroke-width="1" />
		<line x1="0" y1="16.333" x2="3.667" y2="20" stroke="#374151" stroke-width="1" />
	</svg>
</NodeResizeControl>

<div class="relative" style={nodeStyle}>
	<!-- External label in upper left corner -->
	{#if cidr || label}
		<div
			class="absolute -top-8 left-0 z-10 flex items-center gap-1 rounded-md border border-gray-600 bg-gray-800/90 px-2 py-1 shadow-lg backdrop-blur-sm"
		>
			<!-- Icon -->
			{#if IconComponent}
				<!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
				<IconComponent class={`h-3 w-3 ${subnetColorHelper.icon}`} />
			{/if}

			<!-- Label -->
			<span class="whitespace-nowrap text-xs font-medium text-gray-200">
				{label || cidr}
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
				<div
					class="absolute left-1/2 top-0.5 -translate-x-1/2 transform text-[0.5rem] font-semibold {grayColorHelper.text}"
				>
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

	:global(.svelte-flow__resize-control) {
		background-color: transparent !important;
	}
</style>
