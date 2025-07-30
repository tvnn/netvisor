<script lang="ts">
  import { Save, AlertCircle } from 'lucide-svelte';
  import { testActions, validateTest, createBlankTest } from '../../../stores/tests';
  import { nodes } from '../../../stores/nodes';
  import { modalActions, notificationActions } from '../../../stores/ui';
  import type { Test, Layer, Check } from '$lib/types';

  import LayerManager from './LayerManager.svelte';
  import ValidationDisplay from './ValidationDisplay.svelte';

  export let mode: 'create' | 'edit' = 'create';
  export let test: Test | null = null;

  // Initialize form data
  let formData: Test = {
    name: '',
    description: '',
    version: '1.0',
    layers: [],
    ...test
  };

  // Ensure layers have IDs and proper structure
  if (formData.layers) {
    formData.layers = formData.layers.map((layer): Layer => ({
      ...layer,
      id: layer.id || crypto.randomUUID(),
      description: layer.description || '',
      checks: layer.checks || [],
      failureActions: layer.failureActions || []
    }));
  }

  // If creating and no layers, add a default layer
  if (mode === 'create' && formData.layers.length === 0) {
    const blank = createBlankTest();
    formData.layers = blank.layers.map((layer): Layer => ({
      id: crypto.randomUUID(),
      name: layer.name || 'Layer 1',
      description: layer.description || '',
      checks: layer.checks.map((check): Check => ({
        type: check.type,
        config: check.config
      })),
      failureActions: layer.failureActions || []
    }));
  }

  let errors: string[] = [];
  let saving: boolean = false;

  function validateForm(): boolean {
    errors = validateTest(formData, $nodes);
    return errors.length === 0;
  }

  async function handleSave(): Promise<void> {
    if (!validateForm()) {
      return;
    }

    saving = true;
    try {
      // Clean up form data
      const cleanData: Test = {
        ...formData,
        layers: formData.layers.map((layer): Layer => ({
          ...layer,
          checks: layer.checks.filter(check => check.type), // Remove incomplete tests
        }))
      };

      if (mode === 'create') {
        await testActions.add(cleanData);
        notificationActions.success(`Created test: ${cleanData.name}`);
      } else {
        if (!formData.id) {
          throw new Error('Test ID is required for updates');
        }
        await testActions.update(formData.id, cleanData);
        notificationActions.success(`Updated test: ${cleanData.name}`);
      }

      modalActions.close();
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      notificationActions.error(`Failed to save test: ${errorMessage}`);
    } finally {
      saving = false;
    }
  }

  function handleCancel(): void {
    modalActions.close();
  }

  function handleLayersChange(event: CustomEvent<Layer[]>): void {
    formData.layers = event.detail;
    formData = formData; // Trigger reactivity
  }
</script>

<div class="p-6 max-h-[80vh] overflow-y-auto">
  <form on:submit|preventDefault={handleSave} class="space-y-6">
    <!-- Basic Info -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div>
        <label for="test-name" class="block text-sm font-medium text-gray-300 mb-2">
          Test Name <span class="text-red-400">*</span>
        </label>
        <input
          id="test-name"
          type="text"
          bind:value={formData.name}
          required
          class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500"
          placeholder="e.g., Weekly Connectivity Check"
        />
      </div>

      <div>
        <label for="test-version" class="block text-sm font-medium text-gray-300 mb-2">
          Version
        </label>
        <input
          id="test-version"
          type="text"
          bind:value={formData.version}
          class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500"
          placeholder="1.0"
        />
      </div>
    </div>

    <div>
      <label for="test-description" class="block text-sm font-medium text-gray-300 mb-2">
        Description
      </label>
      <textarea
        id="test-description"
        bind:value={formData.description}
        rows="2"
        class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500"
        placeholder="Describe what this test accomplishes..."
      ></textarea>
    </div>

    <!-- Test Layers -->
    <LayerManager
      bind:layers={formData.layers}
      nodeOptions={$nodes}
      on:layersChange={handleLayersChange}
    />

    <!-- Validation Errors -->
    {#if errors.length > 0}
      <ValidationDisplay {errors} />
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
        {saving ? 'Saving...' : mode === 'create' ? 'Create Test' : 'Update Test'}
      </button>
    </div>
  </form>
</div>