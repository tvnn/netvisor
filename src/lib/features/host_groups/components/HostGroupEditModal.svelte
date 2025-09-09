<script lang="ts">
	import { hosts } from "$lib/features/hosts/store";
		import { getHostTargetString } from "$lib/features/hosts/store";
	import EditModal from "$lib/shared/components/forms/EditModal.svelte";
	import ListManager from "$lib/shared/components/forms/ListManager.svelte";
	import type { HostGroup } from "../types/base";

  
  export let group: HostGroup | null = null;
  export let isOpen = false;
  export let onCreate: (data: HostGroup) => Promise<void> | void;
  export let onUpdate: (data: HostGroup) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let formData = createEmptyFormData();
  
  let loading = false;
  let deleting = false;
  let errors: Record<string, string> = {};
  
  $: isEditing = group !== null;
  $: title = isEditing ? `Edit ${group?.name}` : 'Create Host Group';
  
  // Initialize form data when group changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    formData = group ? HostGroupToFormData(group) : createEmptyFormData();
    errors = {};
  }
  
  function validateForm(): boolean {
    errors = {};
    
    if (!formData.name.trim()) {
      errors.name = 'Name is required';
    }
    
    if (formData.hosts.length === 0) {
      errors.hosts = 'At least one host is required';
    }
    
    return Object.keys(errors).length === 0;
  }
  
  async function handleSubmit() {
    const groupData: HostGroup = {
      name: formData.name.trim(),
      description: formData.description.trim(),
      hosts: formData.hosts,
      id: group?.id || '',
      created_at: group?.created_at || '',
      updated_at: group?.updated_at || '',
    };
    
    if (!validateForm()) {
      return;
    }
    
    loading = true;
    try {
      if (isEditing && group) {
        await onUpdate(groupData);
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
  
  function getHostName(hostId: string): string {
    const host = $hosts.find(n => n.id === hostId);
    return host ? host.name : `Host ${hostId.slice(0, 8)}...`;
  }


	function HostGroupToFormData(group: HostGroup): any {
		throw new Error("Function not implemented.");
	}


	function createEmptyFormData(): any {
		throw new Error("Function not implemented.");
	}
</script>

<EditModal
  {isOpen}
  {title}
  {loading}
  {deleting}
  onSubmit={handleSubmit}
  onCancel={onClose}
  {onClose}
  onDelete={isEditing ? handleDelete : null}
  submitLabel={isEditing ? 'Update Group' : 'Create Group'}
>
  <!-- Add proper spacing container around all form elements -->
  <div class="space-y-6">
    <!-- Basic Information -->
    <div>
      <label for="name" class="block text-sm font-medium text-gray-300 mb-1">
        Group Name *
      </label>
      <input
        id="name"
        name="name"
        bind:value={formData.name}
        type="text"
        required
        placeholder="VPN Access Path"
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        class:border-red-500={errors.name}
      />
      {#if errors.name}
        <p class="text-red-400 text-xs mt-1">{errors.name}</p>
      {/if}
    </div>
    
    <div>
      <label for="description" class="block text-sm font-medium text-gray-300 mb-1">
        Description
      </label>
      <textarea
        id="description"
        name="description"
        bind:value={formData.description}
        rows="3"
        placeholder="Describe the purpose of this diagnostic sequence..."
        class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
      ></textarea>
    </div>
        
    <!-- Host Manager -->
    <div>
      <ListManager
        label="Hosts"
        helpText=""
        bind:items={formData.hosts}
        availableOptions={$hosts.map(host => ({
          id: host.id,
          label: host.name,
          subtitle: getHostTargetString(host.target)
        }))}
        placeholder="Select a host to add"
        required={true}
        allowReorder={true}
        getDisplayName={getHostName}
        error={errors.hosts}
      />
    </div>
  </div>
</EditModal>