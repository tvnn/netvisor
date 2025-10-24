<script lang="ts">
	import { Handle, Position } from '@xyflow/svelte';
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
	import type { NodeRenderData } from '../types/base';

	let {
		nodeData,
		width,
		height
	}: { nodeData: NodeRenderData | null; width: number; height: number } = $props();

	const hostColorHelper = entities.getColorHelper('Host');
	const virtualizationColorHelper = entities.getColorHelper('Virtualization');
</script>

{#if nodeData}
	<div
		class={`card ${nodeData.isVirtualized ? `border-color: ${virtualizationColorHelper.border}` : ''}`}
		style={`width: ${width}px; height: ${height}px; display: flex; flex-direction: column; padding: 0;`}
	>
		<!-- Header section with gradient transition to body -->
		{#if nodeData.headerText}
			<div class="relative flex-shrink-0 px-2 py-2 text-center">
				<div
					class={`truncate text-xs font-medium leading-none ${nodeData.isVirtualized ? virtualizationColorHelper.text : 'text-tertiary'}`}
				>
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
							class="text-m text-secondary flex max-w-full items-center justify-center gap-1 truncate text-center"
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

		<!-- Footer section -->
		{#if nodeData.footerText}
			<div class="relative flex flex-shrink-0 items-center justify-center px-2 py-2">
				<div class="text-tertiary truncate text-xs font-medium leading-none">
					{nodeData.footerText}
				</div>
			</div>
		{/if}

		<!-- Connection Handles -->
		<Handle type="target" id="Top" position={Position.Top} style="opacity: 0" />
		<Handle type="target" id="Right" position={Position.Right} style="opacity: 0" />
		<Handle type="target" id="Bottom" position={Position.Bottom} style="opacity: 0" />
		<Handle type="target" id="Left" position={Position.Left} style="opacity: 0" />

		<Handle type="source" id="Top" position={Position.Top} style="opacity: 0" />
		<Handle type="source" id="Right" position={Position.Right} style="opacity: 0" />
		<Handle type="source" id="Bottom" position={Position.Bottom} style="opacity: 0" />
		<Handle type="source" id="Left" position={Position.Left} style="opacity: 0" />
	</div>
{:else}
	<!-- Still render handles even if nodeData is null -->
	<Handle type="target" id="Top" position={Position.Top} style="opacity: 0" />
	<Handle type="target" id="Right" position={Position.Right} style="opacity: 0" />
	<Handle type="target" id="Bottom" position={Position.Bottom} style="opacity: 0" />
	<Handle type="target" id="Left" position={Position.Left} style="opacity: 0" />

	<Handle type="source" id="Top" position={Position.Top} style="opacity: 0" />
	<Handle type="source" id="Right" position={Position.Right} style="opacity: 0" />
	<Handle type="source" id="Bottom" position={Position.Bottom} style="opacity: 0" />
	<Handle type="source" id="Left" position={Position.Left} style="opacity: 0" />
{/if}
