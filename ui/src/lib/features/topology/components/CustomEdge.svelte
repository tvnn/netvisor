<script lang="ts">
	import { type EdgeProps, getSmoothStepPath, BaseEdge, EdgeLabel } from '@xyflow/svelte';

	let {
		id,
		sourceX,
		sourceY,
		sourcePosition,
		targetX,
		targetY,
		targetPosition,
		markerEnd,
		markerStart,
		style,
		label
	}: EdgeProps = $props();

	let [edgePath, labelX, labelY] = $derived(
		getSmoothStepPath({
			sourceX,
			sourceY,
			sourcePosition,
			targetX,
			targetY,
			targetPosition,
			borderRadius: 10,
			offset: 20
		})
	);

	let labelOffsetX = $state(0);
	let labelOffsetY = $state(0);
	let isDragging = $state(false);
	let dragStartX = 0;
	let dragStartY = 0;

	function onDragStart(event: DragEvent) {
		isDragging = true;
		dragStartX = event.clientX - labelOffsetX;
		dragStartY = event.clientY - labelOffsetY;
	}

	function onDrag(event: DragEvent) {
		if (event.clientX === 0 && event.clientY === 0) return; // Ignore end drag event
		labelOffsetX = event.clientX - dragStartX;
		labelOffsetY = event.clientY - dragStartY;
	}

	function onDragEnd() {
		isDragging = false;
	}
</script>

<BaseEdge path={edgePath} {markerEnd} {markerStart} {style} {id} />

{#if label}
	<EdgeLabel x={labelX + labelOffsetX} y={labelY + labelOffsetY} style="background: none;">
		<div
			class="card text-secondary nopan"
			style="font-size: 12px; font-weight: 500; padding: 0.5rem 0.75rem; border-color: rgb(55 65 81); cursor: {isDragging
				? 'grabbing'
				: 'grab'};"
			draggable="true"
			role="button"
			tabindex="0"
			ondragstart={onDragStart}
			ondrag={onDrag}
			ondragend={onDragEnd}
		>
			{label}
		</div>
	</EdgeLabel>
{/if}
