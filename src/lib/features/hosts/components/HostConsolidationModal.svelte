<script lang="ts">
  import { Network, ArrowRight, AlertTriangle, CheckCircle } from 'lucide-svelte';
  import RichSelect from '$lib/shared/components/forms/selection/RichSelect.svelte';
  import type { Host, Interface } from '../types/base';
  import { hosts } from '../store';
  import { getHostTargetString } from '../store';
  import GenericModal from '$lib/shared/components/layout/GenericModal.svelte';
	import EntityDisplay from '$lib/shared/components/forms/selection/display/EntityDisplayWrapper.svelte';
	import { HostDisplay } from '$lib/shared/components/forms/selection/display/HostDisplay.svelte';
	import InlineWarning from '$lib/shared/components/feedback/InlineWarning.svelte';
	import { entities } from '$lib/shared/stores/registry';
	import ModalHeaderIcon from '$lib/shared/components/layout/ModalHeaderIcon.svelte';
  
  export let otherHost: Host | null = null;
  export let isOpen = false;
  export let onConsolidate: (otherHostId: string, destinationHostId: string) => Promise<void> | void;
  export let onClose: () => void;
  
  let selectedDestinationHostId = '';
  let loading = false;
  let showPreview = false;
  
  // Get available hosts (excluding the source host)
  $: availableHosts = otherHost 
    ? $hosts.filter(host => host.id !== otherHost.id)
    : $hosts;
  
  // Get the selected target host
  $: selectedTargetHost = selectedDestinationHostId 
    ? $hosts.find(host => host.id === selectedDestinationHostId) 
    : null;

  // Reset when modal opens/closes
  $: if (isOpen && otherHost) {
    resetForm();
  }
  
  function resetForm() {
    selectedDestinationHostId = '';
    showPreview = false;
    loading = false;
  }
  
  function handleTargetSelection() {
    if (selectedDestinationHostId) {
      showPreview = true;
    }
  }
  
  function handleBack() {
    showPreview = false;
  }
  
  async function handleConsolidate() {
    if (!otherHost || !selectedDestinationHostId) return;
    
    loading = true;
    try {
      await onConsolidate(selectedDestinationHostId, otherHost.id);
      onClose();
    } finally {
      loading = false;
    }
  }
  
  function handleClose() {
    if (!loading) {
      onClose();
    }
  }
  
  function handleHostSelect(hostId: string) {
    selectedDestinationHostId = hostId;
  }

</script>

<GenericModal 
  isOpen={isOpen}
  title="Consolidate Hosts"
  size="lg"
  onClose={handleClose}
  preventCloseOnClickOutside={loading}
>
  <!-- Header icon -->
  <svelte:fragment slot="header-icon">
    <ModalHeaderIcon icon={entities.getIconComponent("Host")} color={entities.getColorString("Host")}/>
  </svelte:fragment>

  <!-- Main content -->
  <div class="p-6">
    {#if !showPreview}
      <!-- Step 1: Target Selection -->
      <div class="space-y-6">
        <!-- Source host info -->
        <div class="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
          <EntityDisplay item={otherHost} displayComponent={HostDisplay} />
        </div>

        <!-- Target selection -->
        <RichSelect
          label="Select host which {otherHost?.name} will be consolidated with:"
          placeholder="Choose a host..."
          selectedValue={selectedDestinationHostId}
          options={availableHosts}
          onSelect={handleHostSelect}
          displayComponent={HostDisplay}
        />
      </div>
    {:else}
      <!-- Step 2: Conversion Preview -->
      <div class="space-y-6">
        <div class="text-center">
          <h3 class="text-lg font-medium text-white mb-2">Consolidation Preview</h3>
          <p class="text-gray-400 text-sm">Review the changes before confirming the consolidation.</p>
        </div>

        <!-- Details of what will happen -->
        <div class="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
          <h4 class="text-sm font-medium text-gray-300 mb-3">What will happen:</h4>
          <ul class="space-y-2 text-sm text-gray-400">
            {#if otherHost && selectedTargetHost}
              <li class="flex items-start gap-2">
                <CheckCircle class="w-4 h-4 text-green-400 mt-0.5 shrink-0" />
                <span>Host "{otherHost.name}" will be deleted</span>
              </li>
              {#if otherHost.services?.length > 0}
                <li class="flex items-start gap-2">
                  <CheckCircle class="w-4 h-4 text-green-400 mt-0.5 shrink-0" />
                  <span>Services will be migrated to "{selectedTargetHost.name}".</span>
                </li>
              {/if}
              {#if otherHost.interfaces?.length > 0}
                <li class="flex items-start gap-2">
                  <CheckCircle class="w-4 h-4 text-green-400 mt-0.5 shrink-0" />
                  <span>Any interfaces for subnets which do not exist on "{selectedTargetHost.name}" will be created.</span>
                </li>
              {/if}
            {/if}
          </ul>
        </div>

        <!-- Warning -->
        <InlineWarning title="This action cannot be undone" body="The source host will be permanently deleted and converted to an interface. Make sure this is what you want before proceeding." />
      </div>
    {/if}
  </div>

  <!-- Footer -->
  <svelte:fragment slot="footer">
    <div class="flex items-center justify-between">
      <div>
        <!-- Empty space for alignment -->
      </div>
      
      <div class="flex items-center gap-3">
        {#if showPreview}
          <button
            type="button"
            disabled={loading}
            on:click={handleBack}
            class="px-4 py-2 text-gray-400 hover:text-white hover:bg-gray-700 
                   border border-gray-600 rounded-lg transition-colors
                   disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Back
          </button>
        {/if}
        
        <button
          type="button"
          disabled={loading}
          on:click={handleClose}
          class="px-4 py-2 text-gray-400 hover:text-white hover:bg-gray-700 
                 border border-gray-600 rounded-lg transition-colors
                 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Cancel
        </button>
        
        {#if !showPreview}
          <button
            type="button"
            disabled={!selectedDestinationHostId}
            on:click={handleTargetSelection}
            class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg 
                   transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Next
          </button>
        {:else}
          <button
            type="button"
            disabled={loading || !selectedDestinationHostId}
            on:click={handleConsolidate}
            class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg 
                   transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? 'Consolidating...' : 'Consolidate Hosts'}
          </button>
        {/if}
      </div>
    </div>
  </svelte:fragment>
</GenericModal>