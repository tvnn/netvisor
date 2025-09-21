<script lang="ts">
  import { Users } from 'lucide-svelte';
  import { createEmptyGroupFormData } from '../store';
  import EditModal from '$lib/shared/components/forms/EditModal.svelte';
  import type { Group, ServiceBinding } from '../types/base';
  import ModalHeaderIcon from '$lib/shared/components/layout/ModalHeaderIcon.svelte';
  import { entities } from '$lib/shared/stores/registry';
  import { services } from '$lib/features/services/store';
  import { ServiceBindingDisplay }  from '$lib/shared/components/forms/selection/display/ServiceBindingDisplay.svelte';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import { ServiceDisplay } from '$lib/shared/components/forms/selection/display/ServiceDisplay.svelte';
  
  export let group: Group | null = null;
  export let isOpen = false;
  export let onCreate: (data: Group) => Promise<void> | void;
  export let onUpdate: (id: string, data: Group) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let loading = false;
  let deleting = false;
  
  $: isEditing = group !== null;
  $: title = isEditing ? `Edit ${group?.name}` : 'Create Group';

  // Get services that are available to add (not already in group)
  $: selectableServices = $services.filter(service => 
    !formData.service_bindings.some(binding => binding.service_id === service.id)
  );
  
  // Convert service bindings to the format expected by ListManager
  $: selectedBindings = formData.service_bindings;
  
  let formData: Group = createEmptyGroupFormData();
  
  // Initialize form data when group changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    formData = group ? { ...group } : createEmptyGroupFormData();
  }

  function handleAdd(serviceId: string) {
    const service = $services.find(s => s.id === serviceId);
    if (!service) return;
    
    // Default to first interface binding if available
    const defaultInterfaceId = service.interface_bindings[0] || '';
    
    const newBinding: ServiceBinding = {
      service_id: serviceId,
      interface_id: defaultInterfaceId
    };
    
    formData.service_bindings = [...formData.service_bindings, newBinding];
  }
  
  function handleRemove(index: number) {
    formData.service_bindings = formData.service_bindings.filter((_, i) => i !== index);
  }
  
  function handleMoveUp(fromIndex: number, toIndex: number) {
    const newBindings = [...formData.service_bindings];
    const [movedBinding] = newBindings.splice(fromIndex, 1);
    newBindings.splice(toIndex, 0, movedBinding);
    formData.service_bindings = newBindings;
  }
  
  function handleMoveDown(fromIndex: number, toIndex: number) {
    const newBindings = [...formData.service_bindings];
    const [movedBinding] = newBindings.splice(fromIndex, 1);
    newBindings.splice(toIndex, 0, movedBinding);
    formData.service_bindings = newBindings;
  }
  
  function handleEdit(item: ServiceBinding, index: number) {
    // This callback is used for simple edit operations
    // For inline editing, the updates come through the renderInlineEdit onUpdate
    // We need to handle binding updates when the interface selection changes
    const updatedBindings = [...formData.service_bindings];
    updatedBindings[index] = item;
    formData.service_bindings = updatedBindings;
  }
  
  async function handleSubmit() {
    // Clean up the data before sending
    const groupData: Group = {
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

  let colorHelper = entities.getColorHelper("Group");
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
    <ModalHeaderIcon icon={entities.getIconComponent("Group")} color={colorHelper.string}/>
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
              placeholder="e.g., DNS Resolution Path, Web Access Chain"
              required
              class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:border-transparent"
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
              placeholder="Describe the data flow or purpose of this service chain..."
              rows="3"
              class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:border-transparent resize-none"
            ></textarea>
          </div>
        </div>

        <!-- Services Section -->
        <div class="space-y-4">
          <div class="border-t border-gray-700 pt-6">
            <h3 class="text-lg font-medium text-white mb-2">Service Chain</h3>
            <p class="text-sm text-gray-400 mb-4">
              Define the sequence of services in this data flow path. Each service can be configured with a specific interface binding.
            </p>
            <div class="bg-gray-800/50 rounded-lg p-4">
              <ListManager
                label="Services"
                helpText="Select services and configure their interface bindings for this chain"
                placeholder="Select a service to add..."
                emptyMessage="No services in this chain yet."
                allowReorder={true}
                
                options={selectableServices}
                items={selectedBindings}
                allowItemEdit={() => true}
                
                optionDisplayComponent={ServiceDisplay}
                itemDisplayComponent={ServiceBindingDisplay}
                
                onAdd={handleAdd}
                onRemove={handleRemove}
                onMoveUp={handleMoveUp}
                onMoveDown={handleMoveDown}
                onEdit={handleEdit}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</EditModal>