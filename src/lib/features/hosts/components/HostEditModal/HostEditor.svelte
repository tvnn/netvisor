<script lang="ts">
  import { Server, Settings, Shield, Info, Network } from 'lucide-svelte';
  import type { Host } from "$lib/features/hosts/types/base";
  import { createEmptyHostFormData } from "$lib/features/hosts/store";
  import DetailsForm from './Details/DetailsForm.svelte';
	import EditModal from '$lib/shared/components/forms/EditModal.svelte';
	import SubnetsForm from './Subnets/SubnetsForm.svelte';
	import ServicesForm from './Services/ServicesForm.svelte';
	import { registry } from '$lib/shared/stores/registry';
  
  export let host: Host | null = null;
  export let isOpen = false;
  export let onCreate: (data: Host) => Promise<void> | void;
  export let onUpdate: (id: string, data: Host) => Promise<void> | void;
  export let onClose: () => void;
  export let onDelete: ((id: string) => Promise<void> | void) | null = null;
  
  let loading = false;
  let deleting = false;
  
  // Tab management
  let activeTab = 'details';
  const tabs = [
    { 
      id: 'details', 
      label: 'Details',
      icon: Info,
      description: 'Basic host information and connection details'
    },
    { 
      id: 'services', 
      label: 'Services',
      icon: Server,
      description: 'Services and monitoring configuration'
    },
    { 
      id: 'subnets', 
      label: 'Subnets',
      icon: Network,
      description: 'Subnet membership'
    }
  ];

  $: currentTabIndex = tabs.findIndex(t => t.id === activeTab) || 0;

  function nextTab() {
    if (currentTabIndex < tabs.length - 1) {
      activeTab = tabs[currentTabIndex + 1].id;
    }
  }

  function previousTab() {
    if (currentTabIndex > 0) {
      activeTab = tabs[currentTabIndex - 1].id;
    }
  }
  
  $: isEditing = host !== null;
  $: title = isEditing ? `Edit ${host?.name}` : 'Create Host';
  
  let formData: Host = createEmptyHostFormData();
  
  // Initialize form data when host changes or modal opens
  $: if (isOpen) {
    resetForm();
  }
  
  function resetForm() {
    // Work directly with Host - no conversion needed
    formData = host ? { ...host } : createEmptyHostFormData();
    activeTab = 'details'; // Reset to first tab
  }
  
  async function handleSubmit() {
    // Clean up the data before sending
    const hostData: Host = {
      ...formData,
      name: formData.name.trim(),
      description: formData.description?.trim() || '',
    };
    
    loading = true;
    try {
      if (isEditing && host) {
        await onUpdate(host.id, hostData);
      } else {
        await onCreate(hostData);
      }
    } finally {
      loading = false;
    }
  }
  
  async function handleDelete() {
    if (onDelete && host) {
      deleting = true;
      try {
        await onDelete(host.id);
      } finally {
        deleting = false;
      }
    }
  }
  
  // Handle form-based submission for create flow with steps
  function handleFormSubmit() {
    if (isEditing || currentTabIndex === tabs.length - 1) {
      handleSubmit();
    } else {
      nextTab();
    }
  }
  
  function handleFormCancel() {
    if (isEditing) {
      onClose();
    } else {
      previousTab();
    }
  }
  
  // Dynamic labels based on create/edit mode and tab position
  $: saveLabel = isEditing ? 'Update Host' : (
    currentTabIndex === tabs.length - 1 ? 'Create Host' : 'Next'
  );
  $: cancelLabel = isEditing ? 'Cancel' : 'Previous';
  $: showCancel = isEditing ? true : activeTab !== 'details';
</script>

<EditModal
  {isOpen}
  {title}
  {loading}
  {deleting}
  {saveLabel}
  {cancelLabel}
  onSave={handleFormSubmit}
  onCancel={showCancel ? handleFormCancel : null}
  onDelete={isEditing ? handleDelete : null}
  size="full"
  let:form
>
  <!-- Header icon -->
  <svelte:fragment slot="header-icon">
    <div class="p-2 bg-blue-600/20 rounded-lg">
      <Server class="w-5 h-5 text-blue-400" />
    </div>
  </svelte:fragment>
  
  <!-- Content -->
  <div class="h-full flex flex-col min-h-0">
    <!-- Tab Navigation (only show for editing) -->
    {#if isEditing}
      <div class="border-b border-gray-700 px-6">
        <nav class="flex space-x-8" aria-label="Host editor tabs">
          {#each tabs as tab}
            <button
              type="button"
              on:click={() => {
                activeTab = tab.id;
              }}
              class="py-4 px-1 border-b-2 font-medium text-sm transition-colors
                     {activeTab === tab.id 
                       ? 'border-blue-500 text-blue-400' 
                       : 'border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-600'}"
              aria-current={activeTab === tab.id ? 'page' : undefined}
            >
              <div class="flex items-center gap-2">
                <svelte:component this={tab.icon} class="w-4 h-4" />
                <span>{tab.label}</span>
                
                <!-- Show capability count indicator -->
                {#if tab.id === 'services' && formData.services.length > 0}
                  <span class="ml-1 px-1.5 py-0.5 text-xs bg-blue-600 text-white rounded-full">
                    {formData.services.length}
                  </span>
                {/if}

                {#if tab.id === 'subnets' && formData.subnets.length > 0}
                  <span class="ml-1 px-1.5 py-0.5 text-xs bg-blue-600 text-white rounded-full">
                    {formData.subnets.length}
                  </span>
                {/if}
              </div>
            </button>
          {/each}
        </nav>
      </div>
    {/if}
    
    <!-- Tab content -->
    <div class="flex-1 overflow-hidden min-h-0">
      {#if activeTab === 'details'}
        <div class="h-full overflow-auto">
          <div class="p-6">
            {#if !isEditing}
              <div class="mb-6">
                <h3 class="text-lg font-medium text-white mb-2">Host Details</h3>
                <p class="text-sm text-gray-400">
                  Configure basic information about this host including its connection details and metadata.
                </p>
              </div>
            {/if}
            
            <DetailsForm 
              {form}
              bind:formData={formData}
            />
          </div>
        </div>
        
      {:else if activeTab === 'services'}
        <div class="h-full overflow-hidden">
          {#if !isEditing}
            <div class="p-6 pb-4 border-b border-gray-700 flex-shrink-0">
              <h3 class="text-lg font-medium text-white mb-2">Capabilities & Monitoring</h3>
              <p class="text-sm text-gray-400">
                Define what services this host provides.
              </p>
            </div>
          {/if}
          
          <div class="flex-1 relative">
            <ServicesForm 
              {form}
              bind:formData={formData}
            />
          </div>
        </div>
      {:else if activeTab == 'subnets'}
      <div class="h-full overflow-hidden">
          {#if !isEditing}
            <div class="p-6 pb-4 border-b border-gray-700 flex-shrink-0">
              <h3 class="text-lg font-medium text-white mb-2">Subnets</h3>
              <p class="text-sm text-gray-400">
                Select the subnets that this host is a member of.
              </p>
            </div>
          {/if}
          
          <div class="flex-1 relative">
            <SubnetsForm 
              {form}
              bind:formData={formData}/>
          </div>
        </div>
      {/if}
    </div>
  </div>
</EditModal>