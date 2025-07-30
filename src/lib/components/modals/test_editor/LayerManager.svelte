<script lang="ts">
  import { Plus } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';
  import type { Layer, NetworkNode } from '$lib/types';

  import LayerEditor from './LayerEditor.svelte';

  export let layers: Layer[] = [];
  export let nodeOptions: NetworkNode[] = [];

  const dispatch = createEventDispatcher<{
    layersChange: Layer[];
  }>();

  let expandedLayers: Set<string> = new Set(layers.map(l => l.id));

  function addLayer(): void {
    const newLayer: Layer = {
      id: crypto.randomUUID(),
      name: `Layer ${layers.length + 1}`,
      description: '',
      checks: [],
      failureActions: []
    };

    layers = [...layers, newLayer];
    expandedLayers.add(newLayer.id);
    expandedLayers = expandedLayers;
    dispatch('layersChange', layers);
  }

  function removeLayer(layerId: string): void {
    layers = layers.filter(l => l.id !== layerId);
    expandedLayers.delete(layerId);
    expandedLayers = expandedLayers;
    dispatch('layersChange', layers);
  }

  function moveLayer(index: number, direction: 'up' | 'down'): void {
    const newIndex = direction === 'up' ? index - 1 : index + 1;
    if (newIndex < 0 || newIndex >= layers.length) return;

    const newLayers = [...layers];
    [newLayers[index], newLayers[newIndex]] = [newLayers[newIndex], newLayers[index]];
    layers = newLayers;
    dispatch('layersChange', layers);
  }

  function toggleLayer(layerId: string): void {
    if (expandedLayers.has(layerId)) {
      expandedLayers.delete(layerId);
    } else {
      expandedLayers.add(layerId);
    }
    expandedLayers = expandedLayers;
  }

  function handleLayerUpdate(event: CustomEvent<Layer>): void {
    const updatedLayer = event.detail;
    const index = layers.findIndex(l => l.id === updatedLayer.id);
    if (index !== -1) {
      layers[index] = updatedLayer;
      layers = layers; // Trigger reactivity
      dispatch('layersChange', layers);
    }
  }
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <h3 class="text-lg font-medium text-white">Test Layers</h3>
    <button
      type="button"
      on:click={addLayer}
      class="flex items-center gap-2 px-3 py-1 bg-green-600 hover:bg-green-700 text-white rounded-lg text-sm transition-colors"
    >
      <Plus class="w-4 h-4" />
      Add Layer
    </button>
  </div>

  {#each layers as layer, index}
    <LayerEditor
      {layer}
      {nodeOptions}
      expanded={expandedLayers.has(layer.id)}
      canMoveUp={index > 0}
      canMoveDown={index < layers.length - 1}
      on:layerUpdate={handleLayerUpdate}
      on:toggleExpanded={() => toggleLayer(layer.id)}
      on:moveUp={() => moveLayer(index, 'up')}
      on:moveDown={() => moveLayer(index, 'down')}
      on:remove={() => removeLayer(layer.id)}
    />
  {/each}
</div>