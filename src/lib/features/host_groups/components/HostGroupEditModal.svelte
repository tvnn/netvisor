<script lang="ts">
  import { Users } from 'lucide-svelte';
  import { createEmptyHostGroupFormData } from '../store';
  import EditModal from '$lib/shared/components/forms/EditModal.svelte';
  import HostManager from './HostManager.svelte';
  import type { HostGroup } from '../types/base';
  
  export let group: HostGroup | null = null;
  export let isOpen = false;
  export let onCreate: (data: HostGroup) => Promise<void> | void;
  export let onUpdate: (id: string, data: HostGroup) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let loading = false;
  let deleting = false;
  
  $: isEditing = group !== null;
  $: title = isEditing ? `Edit ${group?.name}` : 'Create Host Group';
  
  let formData: HostGroup = createEmptyHostGroupFormData();
  
  // Initialize form data when group changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    formData = group ? { ...group } : createEmptyHostGroupFormData();
  }
  
  async function handleSubmit() {
    // Clean up the data before sending
    const groupData: HostGroup = {
      ...formData,
      name: formData.name.trim(),
      description: formData.description?.trim() || '',
    };
    
    loading = true;
    try {
      if (isEditing && group) {
        await onUpdate(group.id, groupData);
      } else {
        await onCreate(groupData);
      }
    } finally {
      loading = false;
    }
  }
  
  async function handleDelete() {
    if (onDelete && group) {
      deleting = true;
      try {
        await onDelete(group.id);
      } finally {
        deleting = false;
      }
    }
  }
  
  // Dynamic labels based on create/edit mode
  $: saveLabel = isEditing ? 'Update Group' : 'Create Group';
  $: cancelLabel = 'Cancel';
</script>

<EditModal
  {isOpen}
  {title}
  {loading}
  {deleting}
  {saveLabel}
  {cancelLabel}
  onSave={handleSubmit}
  onCancel={onClose}
  onDelete={isEditing ? handleDelete : null}
  size="xl"
  let:form
>
  <!-- Header icon -->
  <svelte:fragment slot="header-icon">
    <div class="p-2 bg-purple-600/20 rounded-lg">
      <Users class="w-5 h-5 text-purple-400" />
    </div>
  </svelte:fragment>
  
  <!-- Content -->
  <div class="h-full flex flex-col overflow-hidden">
    <div class="flex-1 overflow-y-auto">
      <div class="space-y-8 p-6">
        <!-- Basic Information -->
        <div class="space-y-4">
          <h3 class="text-lg font-medium text-white">Group Details</h3>
          
          <!-- Name -->
          <div>
            <label for="name" class="block text-sm font-medium text-gray-300 mb-2">
              Group Name <span class="text-red-400">*</span>
            </label>
            <input
              id="name"
              type="text"
              bind:value={formData.name}
              placeholder="e.g., Production Servers, Web Cluster"
              required
              class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
            />
          </div>
          
          <!-- Description -->
          <div>
            <label for="description" class="block text-sm font-medium text-gray-300 mb-2">
              Description
            </label>
            <textarea
              id="description"
              bind:value={formData.description}
              placeholder="Optional description of this host group..."
              rows="3"
              class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent resize-none"
            ></textarea>
          </div>
        </div>

        <!-- Hosts Section -->
        <div class="space-y-4">
          <div class="border-t border-gray-700 pt-6">
            <h3 class="text-lg font-medium text-white mb-4">Hosts</h3>
            <div class="bg-gray-800/50 rounded-lg p-4">
              <HostManager
                {form}
                bind:hostIds={formData.hosts}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</EditModal>