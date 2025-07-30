<!-- Update src/lib/components/tabs/TestsTab.svelte -->
<script lang="ts">
  import { Plus, GitBranch } from 'lucide-svelte';
  import { tests, testActions } from '../../stores/tests';
  import { modalActions, activeTab, notificationActions } from '../../stores/ui';
  import TestEditor from '../modals/test_editor/TestEditor.svelte';
  import ConfirmDialog from '../modals/ConfirmDialog.svelte';
  import Card from '../shared/Card.svelte';
  import type { Test } from '$lib/types';
  
  function createTest(): void {
    modalActions.open(TestEditor, { mode: 'create' }, 'Create Test');
  }
  
  function editTest(test: Test): void {
    modalActions.open(TestEditor, { 
      mode: 'edit', 
      test: { ...test } 
    }, `Edit ${test.name}`);
  }
  
  async function duplicateTest(testId: string): Promise<void> {
    try {
      await testActions.duplicate(testId);
      notificationActions.success('Test duplicated successfully');
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      notificationActions.error('Failed to duplicate test');
      console.error('Failed to duplicate test:', errorMessage);
    }
  }
  
  function deleteTest(test: Test): void {
    if (!test.id) {
      notificationActions.error('Cannot delete test: missing ID');
      return;
    }

    modalActions.open(ConfirmDialog, {
      title: 'Delete Test',
      message: `Are you sure you want to delete "${test.name}"? This action cannot be undone.`,
      confirmText: 'Delete',
      cancelText: 'Cancel',
      danger: true,
      onConfirm: async () => {
        try {
          if (!test.id) {
            throw new Error('Test ID is missing');
          }
          await testActions.delete(test.id);
          notificationActions.success(`Deleted test: ${test.name}`);
        } catch (error: unknown) {
          const errorMessage = error instanceof Error ? error.message : 'Unknown error';
          notificationActions.error('Failed to delete test');
          console.error('Failed to delete test:', errorMessage);
        }
      }
    }, 'Confirm Deletion');
  }
  
  function runTest(testId: string): void {
    // Navigate to diagnostics tab and run this specific test
    activeTab.set('diagnostics');
  }

  function handleDuplicateTest(test: Test): void {
    if (!test.id) {
      notificationActions.error('Cannot duplicate test: missing ID');
      return;
    }
    duplicateTest(test.id);
  }

  function handleRunTest(test: Test): void {
    if (!test.id) {
      notificationActions.error('Cannot run test: missing ID');
      return;
    }
    runTest(test.id);
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold text-white">Tests</h2>
      <p class="text-gray-400 mt-1">Configure network tests</p>
    </div>
    <button
      on:click={createTest}
      class="btn-primary flex items-center gap-2"
    >
      <Plus class="w-4 h-4" />
      Create Test
    </button>
  </div>

  <!-- Tests Grid -->
  {#if $tests.length > 0}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each $tests as test}
        {@const layerCount = test.layers?.length || 0}
        {@const totalChecks = test.layers?.reduce((sum, layer) => sum + (layer.checks?.length || 0), 0) || 0}
        
        <Card
          title={test.name}
          description={test.description || ''}
          metadata={[
            { label: 'Layers', value: layerCount.toString() },
            { label: 'Tests', value: totalChecks.toString() },
            { label: 'Version', value: test.version || 'v1.0' }
          ]}
          onEdit={() => editTest(test)}
          onCopy={() => handleDuplicateTest(test)}
          onDelete={() => deleteTest(test)}
          onRun={() => handleRunTest(test)}
        />
      {/each}
    </div>
  {:else}
    <!-- Empty state -->
    <div class="text-center py-12">
      <GitBranch class="w-16 h-16 mx-auto text-gray-600 mb-4" />
      <h3 class="text-xl font-semibold text-gray-300 mb-2">No Tests</h3>
      <p class="text-gray-400 mb-6 max-w-md mx-auto">
        Create your first network test
      </p>
      <button
        on:click={createTest}
        class="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors mx-auto"
      >
        <Plus class="w-4 h-4" />
        Create Test
      </button>
    </div>
  {/if}
</div>