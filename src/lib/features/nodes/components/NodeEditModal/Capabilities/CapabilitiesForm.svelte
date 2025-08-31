<!-- src/lib/features/nodes/components/NodeEditModal/Capabilities/CapabilitiesForm.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus } from 'lucide-svelte';
  import ListManager from '$lib/shared/components/forms/ListManager.svelte';
  import RichSelect from '$lib/shared/components/forms/RichSelect.svelte';
  import type { Capability } from '$lib/features/capabilities/types/base';
  import type { CapabilityConfigForm } from '$lib/features/capabilities/types/forms';
  import type { NodeContext } from '$lib/features/nodes/types/base';
  import { getCapabilityForms } from '$lib/features/capabilities/store';
  import { createStyle } from '$lib/shared/utils/styling';
	import CapabilitiesConfigPanel from './CapabilitiesConfigPanel.svelte';

  export let selectedCapabilities: Capability[] = [];
  export let nodeContext: NodeContext;

  // Component state
  let availableSchemas: Record<string, CapabilityConfigForm> = {};
  let selectedCapabilityIndex = -1;
  let showAddDropdown = false;
  let addingCapabilityType = '';
  let loading = true;
  let error: string | null = null;

  // Computed values
  $: selectedCapability = selectedCapabilityIndex >= 0 ? selectedCapabilities[selectedCapabilityIndex] : null;
  $: selectedSchema = selectedCapability ? availableSchemas[selectedCapability.capability_type] : null;
  $: availableCapabilityTypes = Object.keys(availableSchemas).filter(type => 
    !selectedCapabilities.some(cap => cap.capability_type === type) || 
    selectedCapabilities.filter(cap => cap.capability_type === type).length === 0
  );

  // Load capability schemas on mount and when node context changes
  $: if (nodeContext) {
    loadCapabilitySchemas();
  }

  async function loadCapabilitySchemas() {
    try {
      loading = true;
      error = null;
      
      // Get all available capability types for this node context
      const allCapabilityTypes = ['Http', 'Https', 'Ssh', 'Node', 'Daemon']; // This would come from registry
      
      const response = await getCapabilityForms({
        capability_types: allCapabilityTypes,
        node_context: nodeContext
      });
      
      if (response && response.data) {
        availableSchemas = response.data;
        
        // Auto-assign capabilities that should be auto-created
        autoAssignCapabilities();
      }
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load capability schemas';
      console.error('Failed to load capability schemas:', err);
    } finally {
      loading = false;
    }
  }

  function autoAssignCapabilities() {
    const autoAssignTypes = Object.entries(availableSchemas)
      .filter(([_, schema]) => schema.auto_assign)
      .map(([type, _]) => type);
    
    for (const capabilityType of autoAssignTypes) {
      // Only auto-assign if we don't already have this capability type
      if (!selectedCapabilities.some(cap => cap.capability_type === capabilityType)) {
        const schema = availableSchemas[capabilityType];
        const newCapability = createCapabilityFromSchema(capabilityType, schema);
        selectedCapabilities = [...selectedCapabilities, newCapability];
      }
    }
  }

  function createCapabilityFromSchema(capabilityType: string, schema: CapabilityConfigForm): Capability {
    // Initialize config with default values
    const config: Record<string, any> = {};
    schema.capability_fields.forEach(field => {
      if (field.default_value !== undefined) {
        config[field.id] = field.default_value;
      }
    });

    // Initialize tests with default values
    const tests = schema.test_sections.map(section => ({
      test: section.test_type,
      criticality: section.test_fields.find(f => f.id === 'criticality')?.default_value || 'Important',
      enabled: section.enabled_by_default,
      monitoring_interval: section.test_fields.find(f => f.id === 'monitoring_interval')?.default_value
    }));

    return {
      id: crypto.randomUUID(),
      name: getDefaultCapabilityName(capabilityType, schema),
      capability_type: capabilityType,
      config,
      tests,
      removable: !schema.auto_assign // Auto-assigned capabilities are not removable
    };
  }

  function getDefaultCapabilityName(capabilityType: string, schema: CapabilityConfigForm): string {
    // Generate a default name based on capability type
    const typeMap: Record<string, string> = {
      'Node': 'Node-Level Tests',
      'Http': 'HTTP Service',
      'Https': 'HTTPS Service', 
      'Ssh': 'SSH Access',
      'Daemon': 'Daemon Service'
    };
    
    return typeMap[capabilityType] || schema.capability_info.display_name || capabilityType;
  }

  function handleAddCapability() {
    if (!addingCapabilityType || !availableSchemas[addingCapabilityType]) return;
    
    const schema = availableSchemas[addingCapabilityType];
    const newCapability = createCapabilityFromSchema(addingCapabilityType, schema);
    
    selectedCapabilities = [...selectedCapabilities, newCapability];
    selectedCapabilityIndex = selectedCapabilities.length - 1;
    
    // Reset add state
    addingCapabilityType = '';
    showAddDropdown = false;
  }

  function handleEditCapability(capability: Capability, index: number) {
    selectedCapabilityIndex = index;
  }

  function handleRemoveCapability(index: number) {
    const capability = selectedCapabilities[index];
    if (!capability.removable) {
      console.warn('Attempted to remove non-removable capability:', capability.name);
      return;
    }
    
    selectedCapabilities = selectedCapabilities.filter((_, i) => i !== index);
    
    // Update selected index
    if (selectedCapabilityIndex === index) {
      selectedCapabilityIndex = -1;
    } else if (selectedCapabilityIndex > index) {
      selectedCapabilityIndex--;
    }
  }

  function handleCapabilityChange(updatedCapability: Capability) {
    if (selectedCapabilityIndex < 0) return;
    
    selectedCapabilities[selectedCapabilityIndex] = updatedCapability;
    selectedCapabilities = selectedCapabilities; // Trigger reactivity
  }

  // Display functions for ListManager
  function getDisplayName(capability: Capability): string {
    return capability.name || 'Unnamed Capability';
  }

  function getDisplayDetails(capability: Capability): string {
    const parts = [];
    
    // Add capability type
    parts.push(capability.capability_type);
    
    // Add key config details
    if (capability.config.port) parts.push(`Port ${capability.config.port}`);
    if (capability.config.path && capability.config.path !== '/') parts.push(capability.config.path);
    if (capability.config.hostname) parts.push(capability.config.hostname);
    
    return parts.join(' • ');
  }

  function getDisplayBadges(capability: Capability) {
    const badges = [];
    
    // Capability type badge
    const schema = availableSchemas[capability.capability_type];
    const typeStyle = schema ? createStyle(schema.capability_info.color, null) : createStyle('gray', null);
    badges.push({
      text: capability.capability_type,
      color: typeStyle.colors.text,
      bgColor: typeStyle.colors.bg
    });
    
    // Test status badge
    const enabledTests = capability.tests.filter(t => t.enabled).length;
    const totalTests = capability.tests.length;
    badges.push({
      text: `${enabledTests}/${totalTests} tests`,
      color: enabledTests === totalTests ? 'text-green-400' : 'text-yellow-400',
      bgColor: enabledTests === totalTests ? 'bg-green-900/20' : 'bg-yellow-900/20'
    });
    
    return badges;
  }

  // Available capability types for dropdown
  $: capabilityTypeOptions = availableCapabilityTypes.map(type => {
    const schema = availableSchemas[type];
    return {
      value: type,
      label: schema.capability_info.display_name,
      description: schema.capability_info.description,
      metadata: schema.capability_info
    };
  });

  onMount(() => {
    // Auto-expand first section on mount
    if (selectedCapabilities.length > 0 && selectedCapabilityIndex === -1) {
      selectedCapabilityIndex = 0;
    }
  });
</script>

{#if loading}
  <div class="h-96 flex items-center justify-center">
    <div class="flex items-center gap-3 text-gray-400">
      <div class="w-5 h-5 border-2 border-gray-400 border-t-transparent rounded-full animate-spin"></div>
      Loading capability configurations...
    </div>
  </div>
{:else if error}
  <div class="h-96 flex items-center justify-center">
    <div class="text-center">
      <div class="text-red-400 mb-2">Failed to load capabilities</div>
      <div class="text-sm text-gray-400">{error}</div>
      <button 
        on:click={loadCapabilitySchemas}
        class="mt-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
      >
        Retry
      </button>
    </div>
  </div>
{:else}
  <div class="h-96 flex gap-6">
    <!-- Left Panel - Capability List -->
    <div class="w-1/2 flex flex-col">
      <!-- Add New Capability -->
      <div class="mb-4">
        {#if showAddDropdown}
          <div class="flex gap-2">
            <div class="flex-1">
              <RichSelect
                label=""
                selectedValue={addingCapabilityType}
                options={capabilityTypeOptions}
                placeholder="Select capability type..."
                onSelect={(value) => addingCapabilityType = value}
                getOptionIcon={(opt) => createStyle(opt.metadata.color, opt.metadata.icon).IconComponent}
                getOptionIconColor={(opt) => createStyle(opt.metadata.color, opt.metadata.icon).colors.icon}
              />
            </div>
            <button
              type="button"
              on:click={handleAddCapability}
              disabled={!addingCapabilityType}
              class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Add
            </button>
            <button
              type="button"
              on:click={() => { showAddDropdown = false; addingCapabilityType = ''; }}
              class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700"
            >
              Cancel
            </button>
          </div>
        {:else}
          <button
            type="button"
            on:click={() => showAddDropdown = true}
            disabled={availableCapabilityTypes.length === 0}
            class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed w-full justify-center"
          >
            <Plus size={16} />
            Add Capability
          </button>
        {/if}
        
        {#if availableCapabilityTypes.length === 0}
          <p class="text-xs text-gray-400 mt-2 text-center">
            All available capability types have been added
          </p>
        {/if}
      </div>

      <!-- Capability List -->
      <div class="flex-1">
        <ListManager
          label="Capabilities"
          helpText="Configure services and their monitoring tests"
          bind:items={selectedCapabilities}
          availableOptions={[]}
          allowReorder={false}
          allowEdit={true}
          allowDirectAdd={false}
          {getDisplayName}
          {getDisplayDetails}
          {getDisplayBadges}
          onEdit={handleEditCapability}
          onRemove={handleRemoveCapability}
          highlightedIndex={selectedCapabilityIndex}
          emptyMessage="No capabilities configured. Add one to get started."
        />
      </div>
    </div>

    <!-- Right Panel - Capability Configuration -->
    <div class="w-1/2 border-l border-gray-600 pl-6">
      <CapabilitiesConfigPanel
        capability={selectedCapability}
        schema={selectedSchema}
        onChange={handleCapabilityChange}
      />
    </div>
  </div>
{/if}