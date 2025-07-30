<script lang="ts">
  import { Save, AlertCircle } from 'lucide-svelte';
  import { nodeActions, validateNode } from '../../stores/nodes';
  import { modalActions, notificationActions } from '../../stores/ui';

  interface NodeData {
    id?: string;
    name: string;
    domain: string;
    ip: string;
    defaultPort: number;
    path: string;
    description: string;
  }

  export let mode: 'create' | 'edit' = 'create';
  export let node: NodeData | null = null;

  // Initialize form data - no type field, no NODE_TYPES logic
  let formData: NodeData = {
    name: '',
    domain: '',
    ip: '',
    defaultPort: 443,
    path: '',
    description: '',
    ...node
  };

  let errors: string[] = [];
  let saving: boolean = false;

  function validateForm(): boolean {
    errors = validateNode(formData);
    return errors.length === 0;
  }

  async function handleSave(): Promise<void> {
    if (!validateForm()) {
      return;
    }

    saving = true;
    try {
      // Clean up form data - remove empty strings and convert port to number
      const cleanData: Partial<NodeData> = { ...formData };
      Object.keys(cleanData).forEach((key) => {
        const typedKey = key as keyof NodeData;
        if (cleanData[typedKey] === '') {
          delete cleanData[typedKey];
        }
      });

      if (cleanData.defaultPort) {
        cleanData.defaultPort = parseInt(cleanData.defaultPort.toString());
      }

      if (mode === 'create') {
        await nodeActions.add(cleanData as NodeData);
        notificationActions.success(`Created node: ${cleanData.name}`);
      } else {
        if (!formData.id) {
          throw new Error('Node ID is required for updates');
        }
        await nodeActions.update(formData.id, cleanData);
        notificationActions.success(`Updated node: ${cleanData.name}`);
      }

      modalActions.close();
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      notificationActions.error(`Failed to save node: ${errorMessage}`);
    } finally {
      saving = false;
    }
  }

  function handleCancel(): void {
    modalActions.close();
  }
</script>

<div class="p-6">
  <form on:submit|preventDefault={handleSave} class="space-y-6">
    <!-- Name -->
    <div>
      <label for="node-name" class="block text-sm font-medium text-gray-300 mb-2">
        Name <span class="text-red-400">*</span>
      </label>
      <input
        id="node-name"
        type="text"
        bind:value={formData.name}
        required
        class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        placeholder="e.g., Google DNS, Cloudflare, Pi-hole"
      />
    </div>

    <!-- Domain -->
    <div>
      <label for="node-domain" class="block text-sm font-medium text-gray-300 mb-2">
        Domain
      </label>
      <input
        id="node-domain"
        type="text"
        bind:value={formData.domain}
        class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        placeholder="e.g., google.com, 1.1.1.1, localhost"
      />
      <p class="text-xs text-gray-400 mt-1">
        Primary hostname or domain for this node
      </p>
    </div>

    <!-- IP Address -->
    <div>
      <label for="node-ip" class="block text-sm font-medium text-gray-300 mb-2">
        IP Address
      </label>
      <input
        id="node-ip"
        type="text"
        bind:value={formData.ip}
        class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        placeholder="e.g., 8.8.8.8, 192.168.1.1"
      />
      <p class="text-xs text-gray-400 mt-1">
        Static IP address for direct connectivity tests
      </p>
    </div>

    <!-- Default Port -->
    <div>
      <label for="node-port" class="block text-sm font-medium text-gray-300 mb-2">
        Default Port
      </label>
      <input
        id="node-port"
        type="number"
        bind:value={formData.defaultPort}
        min="1"
        max="65535"
        class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        placeholder="443"
      />
      <p class="text-xs text-gray-400 mt-1">
        Default port for connectivity tests (1-65535)
      </p>
    </div>

    <!-- Path -->
    <div>
      <label for="node-path" class="block text-sm font-medium text-gray-300 mb-2">
        Path
      </label>
      <input
        id="node-path"
        type="text"
        bind:value={formData.path}
        class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        placeholder="e.g., /dns-query, /admin, /api/health"
      />
      <p class="text-xs text-gray-400 mt-1">
        For DNS over HTTPS endpoints, service health paths, or API endpoints
      </p>
    </div>

    <!-- Description -->
    <div>
      <label for="node-description" class="block text-sm font-medium text-gray-300 mb-2">
        Description
      </label>
      <textarea
        id="node-description"
        bind:value={formData.description}
        rows="3"
        class="w-full bg-gray-700 border border-gray-600 text-white rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        placeholder="Optional description of this network node"
      ></textarea>
    </div>

    <!-- Validation Errors -->
    {#if errors.length > 0}
      <div class="bg-red-900/20 border border-red-700 rounded-lg p-4">
        <div class="flex items-center gap-2 mb-2">
          <AlertCircle class="w-4 h-4 text-red-400" />
          <span class="font-medium text-red-400">Validation Errors</span>
        </div>
        <ul class="text-sm text-red-300 space-y-1">
          {#each errors as error}
            <li>• {error}</li>
          {/each}
        </ul>
      </div>
    {/if}

    <!-- Action Buttons -->
    <div class="flex items-center justify-end gap-3 pt-4 border-t border-gray-700">
      <button
        type="button"
        on:click={handleCancel}
        class="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors"
      >
        Cancel
      </button>
      <button
        type="submit"
        disabled={saving}
        class="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-800 disabled:opacity-50 text-white rounded-lg transition-colors"
      >
        <Save class="w-4 h-4" />
        {saving ? 'Saving...' : mode === 'create' ? 'Create Node' : 'Update Node'}
      </button>
    </div>
  </form>
</div>