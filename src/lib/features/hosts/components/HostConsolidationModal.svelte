<script lang="ts">
  import { Network, ArrowRight, AlertTriangle, CheckCircle } from 'lucide-svelte';
  import RichSelect from '$lib/shared/components/forms/selection/RichSelect.svelte';
  import type { Host } from '../types/base';
  import { hosts } from '../store';
  import { getHostTargetString } from '../store';
  import GenericModal from '$lib/shared/components/layout/GenericModal.svelte';
	import EntityDisplay from '$lib/shared/components/forms/selection/display/EntityDisplayWrapper.svelte';
	import { HostDisplay } from '$lib/shared/components/forms/selection/display/HostDisplay.svelte';
  
  export let otherHost: Host | null = null;
  export let isOpen = false;
  export let onConvert: (otherHostId: string, destinationHostId: string) => Promise<void> | void;
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
  
  async function handleConvert() {
    if (!otherHost || !selectedDestinationHostId) return;
    
    loading = true;
    try {
      await onConvert(otherHost.id, selectedDestinationHostId);
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
  
  // Generate a preview interface name
  $: previewInterfaceName = otherHost ? 
    `${otherHost.name.toLowerCase().replace(/\s+/g, '_')}_interface` : '';
    
  // Get target string for display
  function getDisplayTarget(host: Host): string {
    return getHostTargetString(host);
  }
</script>

<GenericModal 
  isOpen={isOpen}
  title="Convert Host to Interface"
  size="lg"
  onClose={handleClose}
  preventCloseOnClickOutside={loading}
>
  <!-- Header icon -->
  <svelte:fragment slot="header-icon">
    <div class="p-2 bg-green-600/20 rounded-lg">
      <Network class="w-5 h-5 text-green-400" />
    </div>
  </svelte:fragment>

  <!-- Main content -->
  <div class="p-6">
    {#if !showPreview}
      <!-- Step 1: Target Selection -->
      <div class="space-y-6">
        <!-- Source host info -->
        <div class="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
          {#if otherHost}
            <EntityDisplay item={otherHost} displayComponent={HostDisplay} />
          {/if}
        </div>

        <!-- Target selection -->
        <RichSelect
          label="Select Target Host to Add Interface To:"
          placeholder="Choose a host to convert the interface to"
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
          <h3 class="text-lg font-medium text-white mb-2">Conversion Preview</h3>
          <p class="text-gray-400 text-sm">Review the changes before confirming the conversion.</p>
        </div>

        <!-- Visual flow -->
        <div class="flex items-center justify-center gap-4 py-4">
          {#if otherHost}
            <!-- Source host -->
            <EntityDisplay item={otherHost} displayComponent={HostDisplay} />

            <!-- Arrow -->
            <div class="flex items-center gap-2 px-4">
              <ArrowRight class="w-5 h-5 text-gray-400" />
              <span class="text-xs text-gray-400">Converts to</span>
              <ArrowRight class="w-5 h-5 text-gray-400" />
            </div>

            <!-- Target structure -->
            <div class="text-center">
              <div class="relative">
                <!-- Target host -->
                <div class="p-3 bg-green-600/20 rounded-lg border-2 border-green-500/30 mb-2">
                  <Network class="w-6 h-6 text-green-400 mx-auto" />
                </div>
                {#if selectedTargetHost}
                  <p class="text-sm font-medium text-white">{selectedTargetHost.name}</p>
                {/if}
                <p class="text-xs text-gray-400">Target Host</p>

                <!-- New interface indicator -->
                <div class="absolute -bottom-2 -right-2 p-1.5 bg-yellow-600/20 rounded border border-yellow-500/30">
                  <Network class="w-3 h-3 text-yellow-400" />
                </div>
              </div>
            </div>
          {/if}
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
              <li class="flex items-start gap-2">
                <CheckCircle class="w-4 h-4 text-green-400 mt-0.5 shrink-0" />
                <span>A new interface "{previewInterfaceName}" will be added to "{selectedTargetHost.name}"</span>
              </li>
              <li class="flex items-start gap-2">
                <CheckCircle class="w-4 h-4 text-green-400 mt-0.5 shrink-0" />
                <span>Target: {getDisplayTarget(otherHost)} will be preserved in the new interface</span>
              </li>
              {#if otherHost.services?.length > 0}
                <li class="flex items-start gap-2">
                  <CheckCircle class="w-4 h-4 text-green-400 mt-0.5 shrink-0" />
                  <span>Services and configurations will be migrated</span>
                </li>
              {/if}
            {/if}
          </ul>
        </div>

        <!-- Warning -->
        <div class="bg-yellow-900/20 border border-yellow-600/30 rounded-lg p-4">
          <div class="flex items-start gap-2">
            <AlertTriangle class="w-4 h-4 text-yellow-400 mt-0.5 shrink-0" />
            <div>
              <p class="text-yellow-400 text-sm font-medium">This action cannot be undone</p>
              <p class="text-yellow-300/80 text-sm mt-1">
                The source host will be permanently deleted and converted to an interface. 
                Make sure this is what you want before proceeding.
              </p>
            </div>
          </div>
        </div>
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
            on:click={handleConvert}
            class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg 
                   transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? 'Converting...' : 'Convert to Interface'}
          </button>
        {/if}
      </div>
    </div>
  </svelte:fragment>
</GenericModal>