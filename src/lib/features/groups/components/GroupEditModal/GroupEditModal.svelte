<script lang="ts">
  import { Users } from 'lucide-svelte';
  import { createEmptyGroupFormData } from '../../store';
  import EditModal from '$lib/shared/components/forms/EditModal.svelte';
  import type { Group, ServiceBinding } from '../../types/base';
  import ModalHeaderIcon from '$lib/shared/components/layout/ModalHeaderIcon.svelte';
  import { entities } from '$lib/shared/stores/metadata';
  import { getServiceHost, services } from '$lib/features/services/store';
  import { ServiceBindingDisplay }  from '$lib/shared/components/forms/selection/display/ServiceBindingDisplay.svelte';
  import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import { ServiceDisplay } from '$lib/shared/components/forms/selection/display/ServiceDisplay.svelte';
	import GroupDetailsForm from './GroupDetailsForm.svelte';
	import { pushWarning } from '$lib/shared/stores/feedback';
	import EntityMetadataSection from '$lib/shared/components/forms/EntityMetadataSection.svelte';
  
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

  let formData: Group = createEmptyGroupFormData();
  
  // Initialize form data when group changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    formData = group ? { ...group } : createEmptyGroupFormData();
  }

  // Get services that are available to add (not already in group + has some interface binding OR has externlIp target)
  $: selectableServices = $services.filter(service => 
    !formData.service_bindings.some(binding => binding.service_id === service.id)
    && (service.interface_bindings.length > 0 || getServiceHost(service.id)?.target.type == 'ExternalIp')
  );

  function handleAdd(serviceId: string) {
    const service = $services.find(s => s.id === serviceId);
    if (!service) return;
    
    const host = getServiceHost(service.id);
    // Default to first interface binding if available
    let defaultInterfaceId;
    if (host && host?.target.type == 'ExternalIp') {
      defaultInterfaceId = host.id
    } else if (service.interface_bindings.length > 0) {
      defaultInterfaceId = service.interface_bindings[0];
    } else {
      pushWarning(`Host for service ${service.name} must have an interface or an External IP target`)
      return;
    }
    
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
  let:formApi
>
  <!-- Header icon -->
  <svelte:fragment slot="header-icon">
    <ModalHeaderIcon icon={entities.getIconComponent("Group")} color={colorHelper.string}/>
  </svelte:fragment>
  
  <!-- Content -->
  <div class="h-full flex flex-col overflow-hidden">
    <div class="flex-1 overflow-y-auto">
      <div class="space-y-8 p-6">

        <GroupDetailsForm
          {form}
          {formApi}
          bind:formData={formData} />

        <!-- Services Section -->
        <div class="space-y-4">
          <div class="border-t border-gray-700 pt-6">
            <h3 class="text-lg font-medium text-white mb-2">Service Chain</h3>
            <p class="text-sm text-gray-400 mb-4">
              Define the services in this group. Each service can be configured with a specific interface binding.
            </p>
            <div class="bg-gray-800/50 rounded-lg p-4">
              <ListManager
                label="Services"
                helpText="Select services and configure their interface bindings for this group"
                placeholder="Select a service to add..."
                emptyMessage="No services in this group yet."
                allowReorder={true}
                
                options={selectableServices}
                items={formData.service_bindings}
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

        {#if isEditing}
          <EntityMetadataSection id={formData.id} createdAt={formData.created_at} updatedAt={formData.updated_at}/>
        {/if}
      </div>
    </div>
  </div>
</EditModal>