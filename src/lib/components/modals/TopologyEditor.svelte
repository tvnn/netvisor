<script>
  import { Save, AlertCircle, Plus, Trash2, Move, ChevronDown, ChevronRight, Play, Server, Zap } from 'lucide-svelte';
  import { topologyActions, validateTopology, TEST_TYPES, createBlankTopology } from '../../stores/topologies';
  import { nodes } from '../../stores/nodes';
  import { modalActions, notificationActions } from '../../stores/ui';
  import { getDefaultTestConfig } from '../../stores/topologies';

  import TestTypeSelector from './TestTypeSelector.svelte';

  export let mode = 'create'; // 'create' or 'edit'
  export let topology = null;

  // Initialize form data
  let formData = {
    name: '',
    description: '',
    version: '1.0',
    layers: [],
    ...topology
  };

  // Ensure layers have IDs and proper structure
  if (formData.layers) {
    formData.layers = formData.layers.map(layer => ({
      ...layer,
      id: layer.id || crypto.randomUUID(),
      tests: layer.tests || [],
      failureActions: layer.failureActions || []
    }));
  }

  // If creating and no layers, add a default layer
  if (mode === 'create' && formData.layers.length === 0) {
    const blank = createBlankTopology();
    formData.layers = blank.layers;
  }

  let errors = [];
  let saving = false;
  let expandedLayers = new Set(formData.layers.map(l => l.id));
  let expandedTests = new Set();

  // Available node references for interpolation
  $: nodeOptions = $nodes.map(node => ({
    id: node.id,
    name: node.name,
    domain: node.domain,
    ip: node.ip,
    defaultPort: node.defaultPort
  }));

  function validateForm() {
    errors = validateTopology(formData, $nodes);
    return errors.length === 0;
  }

  async function handleSave() {
    if (!validateForm()) {
      return;
    }

    saving = true;
    try {
      // Clean up form data
      const cleanData = {
        ...formData,
        layers: formData.layers.map(layer => ({
          ...layer,
          tests: layer.tests.filter(test => test.type), // Remove incomplete tests
        }))
      };

      if (mode === 'create') {
        await topologyActions.add(cleanData);
        notificationActions.success(`Created topology: ${cleanData.name}`);
      } else {
        await topologyActions.update(formData.id, cleanData);
        notificationActions.success(`Updated topology: ${cleanData.name}`);
      }

      modalActions.close();
    } catch (error) {
      notificationActions.error(`Failed to save topology: ${error.message}`);
    } finally {
      saving = false;
    }
  }

  function handleCancel() {
    modalActions.close();
  }

  // Layer management
  function addLayer() {
    const newLayer = {
      id: crypto.randomUUID(),
      name: `Layer ${formData.layers.length + 1}`,
      description: '',
      tests: [],
    };
    formData.layers = [...formData.layers, newLayer];
    expandedLayers.add(newLayer.id);
  }

  function removeLayer(layerId) {
    formData.layers = formData.layers.filter(l => l.id !== layerId);
    expandedLayers.delete(layerId);
  }

  function moveLayer(layerId, direction) {
    const index = formData.layers.findIndex(l => l.id === layerId);
    if (index === -1) return;
    
    const newIndex = direction === 'up' ? index - 1 : index + 1;
    if (newIndex < 0 || newIndex >= formData.layers.length) return;

    const layers = [...formData.layers];
    [layers[index], layers[newIndex]] = [layers[newIndex], layers[index]];
    formData.layers = layers;
  }

  function toggleLayer(layerId) {
    if (expandedLayers.has(layerId)) {
      expandedLayers.delete(layerId);
    } else {
      expandedLayers.add(layerId);
    }
    expandedLayers = expandedLayers;
  }

  // Test management
  function addTest(layerId) {
    const layer = formData.layers.find(l => l.id === layerId);
    if (!layer) return;

    const newTest = {
      id: crypto.randomUUID(),
      type: 'connectivity_test',
      config: {}
    };

    layer.tests = [...layer.tests, newTest];
    expandedTests.add(newTest.id);
    formData = formData; // Trigger reactivity
  }

  function removeTest(layerId, testIndex) {
    const layer = formData.layers.find(l => l.id === layerId);
    if (!layer) return;

    const testId = layer.tests[testIndex]?.id;
    if (testId) {
      expandedTests.delete(testId);
    }

    layer.tests = layer.tests.filter((_, i) => i !== testIndex);
    formData = formData;
  }

  function toggleTest(testId) {
    if (expandedTests.has(testId)) {
      expandedTests.delete(testId);
    } else {
      expandedTests.add(testId);
    }
    expandedTests = expandedTests;
  }

  function updateTestType(layerId, testIndex, newType) {
    const layer = formData.layers.find(l => l.id === layerId);
    if (!layer || !layer.tests[testIndex]) return;

    // Reset config when changing test type
    layer.tests[testIndex] = {
      ...layer.tests[testIndex],
      type: newType,
      config: getDefaultTestConfig(newType)
    };
    formData = formData;
  }

  // Node reference helpers
  function getNodeReferenceOptions() {
    return nodeOptions.map(node => [
      { value: `{{${node.id}}}`, label: `${node.name} (auto)` },
      { value: `{{${node.id}.domain}}`, label: `${node.name} (domain)` },
      { value: `{{${node.id}.ip}}`, label: `${node.name} (IP)` },
      { value: `{{${node.id}.defaultPort}}`, label: `${node.name} (port)` }
    ]).flat();
  }

  function insertNodeReference(event, layerId, testIndex, field) {
    const target = event.target;
    const value = target.value;
    
    if (value.startsWith('{{')) {
      const layer = formData.layers.find(l => l.id === layerId);
      if (layer?.tests[testIndex]) {
        layer.tests[testIndex].config[field] = value;
        formData = formData;
      }
    }
  }
</script>

<div class="p-6 max-h-[80vh] overflow-y-auto">
  <form on:submit|preventDefault={handleSave} class="space-y-6">
    <!-- Basic Information -->
    <div class="space-y-4">
      <h3 class="text-lg font-semibold text-white border-b border-gray-700 pb-2">
        Basic Information
      </h3>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-300 mb-2">
            Name <span class="text-red-400">*</span>
          </label>
          <input
            type="text"
            bind:value={formData.name}
            class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            placeholder="e.g., WireGuard + Pi-hole Setup"
            required
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-300 mb-2">
            Version
          </label>
          <input
            type="text"
            bind:value={formData.version}
            class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            placeholder="e.g., 1.0"
          />
        </div>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-300 mb-2">
          Description
        </label>
        <textarea
          bind:value={formData.description}
          rows="2"
          class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          placeholder="Describe what this topology tests"
        ></textarea>
      </div>
    </div>

    <!-- Layers -->
    <div class="space-y-4">
      <div class="flex items-center justify-between border-b border-gray-700 pb-2">
        <h3 class="text-lg font-semibold text-white">
          Test Layers ({formData.layers.length})
        </h3>
        <button
          type="button"
          on:click={addLayer}
          class="flex items-center gap-2 px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
        >
          <Plus class="w-4 h-4" />
          Add Layer
        </button>
      </div>

      {#if formData.layers.length === 0}
        <div class="text-center py-8 text-gray-400">
          <Server class="w-12 h-12 mx-auto mb-3 opacity-50" />
          <p>No layers configured</p>
          <p class="text-sm">Add your first test layer to get started</p>
        </div>
      {/if}

      {#each formData.layers as layer, layerIndex (layer.id)}
        <div class="bg-gray-800 border border-gray-700 rounded-lg overflow-hidden">
          <!-- Layer Header -->
          <div class="p-4 border-b border-gray-700">
            <div class="flex items-center justify-between">
              <button
                type="button"
                on:click={() => toggleLayer(layer.id)}
                class="flex items-center gap-2 text-white hover:text-blue-400 transition-colors"
              >
                {#if expandedLayers.has(layer.id)}
                  <ChevronDown class="w-4 h-4" />
                {:else}
                  <ChevronRight class="w-4 h-4" />
                {/if}
                <span class="font-medium">{layer.name || `Layer ${layerIndex + 1}`}</span>
                <span class="text-sm text-gray-400">({layer.tests?.length || 0} tests)</span>
              </button>

              <div class="flex items-center gap-2">
                {#if layerIndex > 0}
                  <button
                    type="button"
                    on:click={() => moveLayer(layer.id, 'up')}
                    class="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white transition-colors"
                    title="Move up"
                  >
                    <Move class="w-4 h-4" />
                  </button>
                {/if}
                {#if layerIndex < formData.layers.length - 1}
                  <button
                    type="button"
                    on:click={() => moveLayer(layer.id, 'down')}
                    class="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white transition-colors"
                    title="Move down"
                  >
                    <Move class="w-4 h-4 rotate-180" />
                  </button>
                {/if}
                <button
                  type="button"
                  on:click={() => removeLayer(layer.id)}
                  class="p-1 hover:bg-red-700 rounded text-gray-400 hover:text-red-400 transition-colors"
                  title="Delete layer"
                >
                  <Trash2 class="w-4 h-4" />
                </button>
              </div>
            </div>
          </div>

          <!-- Layer Content -->
          {#if expandedLayers.has(layer.id)}
            <div class="p-4 space-y-4">
              <!-- Layer Basic Info -->
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">
                    Layer Name
                  </label>
                  <input
                    type="text"
                    bind:value={layer.name}
                    class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    placeholder="e.g., Internet Connectivity"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-300 mb-2">
                    Description
                  </label>
                  <input
                    type="text"
                    bind:value={layer.description}
                    class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    placeholder="What does this layer test?"
                  />
                </div>
              </div>

              <!-- Tests -->
              <div class="space-y-3">
                <div class="flex items-center justify-between">
                  <h4 class="font-medium text-gray-300">Tests ({layer.tests?.length || 0})</h4>
                  <button
                    type="button"
                    on:click={() => addTest(layer.id)}
                    class="flex items-center gap-1 px-2 py-1 text-sm bg-green-600 hover:bg-green-700 text-white rounded transition-colors"
                  >
                    <Plus class="w-3 h-3" />
                    Add Test
                  </button>
                </div>

                {#each layer.tests || [] as test, testIndex (test.id || testIndex)}
                  <div class="bg-gray-900 border border-gray-600 rounded-lg overflow-hidden">
                    <!-- Test Header -->
                    <div class="p-3 border-b border-gray-600">
                      <div class="flex items-center justify-between">
                        <button
                          type="button"
                          on:click={() => toggleTest(test.id)}
                          class="flex items-center gap-2 text-gray-300 hover:text-white transition-colors"
                        >
                          {#if expandedTests.has(test.id)}
                            <ChevronDown class="w-3 h-3" />
                          {:else}
                            <ChevronRight class="w-3 h-3" />
                          {/if}
                          <Zap class="w-3 h-3" />
                          <span class="text-sm">{TEST_TYPES[test.type]?.name || test.type}</span>
                        </button>
                        <button
                          type="button"
                          on:click={() => removeTest(layer.id, testIndex)}
                          class="p-1 hover:bg-red-700 rounded text-gray-400 hover:text-red-400 transition-colors"
                        >
                          <Trash2 class="w-3 h-3" />
                        </button>
                      </div>
                    </div>

                    <!-- Test Configuration -->
                    {#if expandedTests.has(test.id)}
                      <div class="p-3 space-y-3">
                        <!-- Test Type Selection -->
                        <div class="col-span-full">
                          <label class="block text-sm font-medium text-gray-300 mb-2">
                            Test Type
                          </label>
                          <TestTypeSelector 
                            bind:value={test.type}
                            on:change={(e) => updateTestType(layer.id, testIndex, e.detail.value)}
                          />
                        </div>

                        <!-- Dynamic Test Configuration -->
                        {#if test.type && TEST_TYPES[test.type]}
                          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                            {#each TEST_TYPES[test.type].fields as field}
                              <div>
                                <label class="block text-xs font-medium text-gray-300 mb-1 capitalize">
                                  {field.replace('_', ' ')}
                                </label>
                                
                                {#if field === 'protocol'}
                                  <select
                                    bind:value={test.config[field]}
                                    class="w-full bg-gray-700 border border-gray-600 text-white rounded px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500"
                                  >
                                    <option value="http">HTTP</option>
                                    <option value="https">HTTPS</option>
                                  </select>
                                {:else if field === 'service_type'}
                                  <select
                                    bind:value={test.config[field]}
                                    class="w-full bg-gray-700 border border-gray-600 text-white rounded px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500"
                                  >
                                    <option value="auto">Auto-detect</option>
                                    <option value="google">Google</option>
                                    <option value="cloudflare">Cloudflare</option>
                                    <option value="pihole">Pi-hole</option>
                                  </select>
                                {:else if field.includes('port') || field.includes('timeout') || field.includes('status') || field.includes('time') || field === 'attempts'}
                                  <input
                                    type="number"
                                    bind:value={test.config[field]}
                                    class="w-full bg-gray-700 border border-gray-600 text-white rounded px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500"
                                    placeholder={field === 'port' ? '443' : field === 'timeout' ? '5000' : ''}
                                  />
                                {:else}
                                  <div class="relative">
                                    <input
                                      type="text"
                                      bind:value={test.config[field]}
                                      class="w-full bg-gray-700 border border-gray-600 text-white rounded px-2 py-1 text-sm focus:ring-2 focus:ring-blue-500"
                                      placeholder={field === 'target' ? 'example.com or {{node-id}}' : field === 'domain' ? 'google.com' : ''}
                                    />
                                    {#if (field === 'target' || field === 'domain') && nodeOptions.length > 0}
                                      <select
                                        on:change={(e) => insertNodeReference(e, layer.id, testIndex, field)}
                                        class="absolute right-0 top-0 h-full bg-gray-600 border-l border-gray-500 text-white text-xs px-2 rounded-r opacity-75 hover:opacity-100"
                                      >
                                        <option value="">Node ref...</option>
                                        {#each getNodeReferenceOptions() as option}
                                          <option value={option.value}>{option.label}</option>
                                        {/each}
                                      </select>
                                    {/if}
                                  </div>
                                {/if}
                              </div>
                            {/each}
                          </div>
                        {/if}
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      {/each}
    </div>

    <!-- Validation Errors -->
    {#if errors.length > 0}
      <div class="bg-red-900/30 border border-red-700/50 rounded-lg p-4">
        <div class="flex items-center gap-2 mb-2">
          <AlertCircle class="w-4 h-4 text-red-400" />
          <span class="text-red-400 font-medium">Validation Errors</span>
        </div>
        <ul class="text-sm text-red-300 space-y-1">
          {#each errors as error}
            <li>• {error}</li>
          {/each}
        </ul>
      </div>
    {/if}

    <!-- Form Actions -->
    <div class="flex justify-end gap-3 pt-4 border-t border-gray-700">
      <button
        type="button"
        on:click={handleCancel}
        class="px-4 py-2 text-gray-300 hover:text-white transition-colors"
        disabled={saving}
      >
        Cancel
      </button>
      <button
        type="submit"
        class="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
        disabled={saving}
      >
        <Save class="w-4 h-4" />
        {saving ? 'Saving...' : mode === 'create' ? 'Create Topology' : 'Update Topology'}
      </button>
    </div>
  </form>
</div>