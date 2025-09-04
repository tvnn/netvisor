<!-- src/lib/features/nodes/components/NodeEditModal/Capabilities/CapabilitiesForm.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import type { Capability, CapabilityConfig } from '$lib/features/capabilities/types/base';
  import type { NodeContext } from '$lib/features/nodes/types/base';
  import type { CapabilityConfigForm } from '$lib/features/capabilities/types/forms';
  import { getCapabilityConfig, getCapabilityType, createCapability, getTestConfigFromSchema } from '$lib/features/capabilities/types/base';
  import CapabilitiesConfigPanel from './CapabilitiesConfigPanel.svelte';
  import { capabilities } from '$lib/shared/stores/registry';
  import { createStyle } from '$lib/shared/utils/styling';
  import { getCapabilityForms } from '$lib/features/capabilities/store';
  import { loading, pushError } from '$lib/shared/stores/feedback';
	import ListManager from '$lib/shared/components/forms/ListManager.svelte';
  
  export let form: any;
  export let selectedCapabilities: Capability[] = [];
  export let nodeContext: NodeContext;
  
  let availableSchemas: Record<string, CapabilityConfigForm> = {};
  let selectedCapabilityIndex = -1;
  
$: selectedCapability = selectedCapabilityIndex >= 0 ? selectedCapabilities[selectedCapabilityIndex] : null;
$: selectedCapabilityType = selectedCapability ? getCapabilityType(selectedCapability) : null;
$: selectedSchema = selectedCapabilityType ? availableSchemas[selectedCapabilityType] || null : null;

// Available capability types for dropdown
let capabilitySelectOptions: string[] = [];

  $: if (availableSchemas && Object.keys(availableSchemas).length > 0 && capabilitySelectOptions.length === 0) {
    // Only compute once when schemas are loaded
    capabilitySelectOptions = Object.keys(availableSchemas)
      .filter(type => !availableSchemas[type]?.system_assigned)
      .map(type => type); // Just strings, not full objects
  }

  async function loadCapabilitySchemas() {
    try {            
      const response = await getCapabilityForms({
        node_context: nodeContext
      });
      
      if (response && response?.data) {
        availableSchemas = response.data;
      }
    } catch (err) {
      pushError(err instanceof Error ? err.message : 'Failed to load capability schemas')
    }
  }

function handleEditCapability(_: any, index: number) {
  selectedCapabilityIndex = index;
}

function capabilityFromType(capabilityType: string) {
  const schema = availableSchemas[capabilityType]
  const baseConfig = {
    name: capabilities.getDisplay(capabilityType),
    tests: schema?.test_sections?.map(section => ({
      test: {
        type: section.test_type,
        config: getTestConfigFromSchema(section)
      },
      criticality: section.test_fields.find(f => f.id === 'criticality')?.default_value || 'Important',
      enabled: section.enabled_by_default ?? false
    })) || [],
    system_assigned: schema?.system_assigned ?? false,
    port: undefined,
    process: undefined,
    discovery_ports: undefined
  };

    let config: CapabilityConfig = { ...baseConfig };

    // Add capability-specific default values from schema
    schema?.capability_fields?.forEach(field => {
      if (field.default_value !== undefined) {
        config[field.id] = field.default_value;
      }
    });

    return createCapability(capabilityType, config);
  }

function handleAddCapability(capabilityType: string) {  
  const newCapability = capabilityFromType(capabilityType);
  selectedCapabilities = [...selectedCapabilities, newCapability];
  selectedCapabilityIndex = selectedCapabilities.length - 1;
}

  function handleRemoveCapability(index: number) {
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
    if (selectedCapabilities[selectedCapabilityIndex] == updatedCapability) return;

    selectedCapabilities[selectedCapabilityIndex] == updatedCapability
    selectedCapabilities = selectedCapabilities; // Trigger reactivity
  }

  function getOptionId(capabilityType: string): string {
    return capabilityType;
  }

  function getOptionLabel(capabilityType: string): string {
    return capabilities.getDisplay(capabilityType);
  }

  function getOptionDescription(capabilityType: string): string {
    return capabilities.getDescription(capabilityType);
  }

  function getOptionIcon(capabilityType: string) {
    return createStyle(null, capabilities.getIcon(capabilityType)).IconComponent;
  }

  function getOptionIconColor(capabilityType: string) {
    return capabilities.getColor(capabilityType).icon;
  }

  function getItemId(capability: Capability): string {
    return getCapabilityType(capability);
  }

  function getItemLabel(capability: Capability): string {
    const config = getCapabilityConfig(capability);
    return config.name || 'Unnamed Capability';
  }

  function getItemDescription(capability: Capability): string {
    const parts = [];
    const type = getCapabilityType(capability);
    const config = getCapabilityConfig(capability);
    
    // Add capability type
    parts.push(type);
    
    // Add key config details
    if (config.port) parts.push(`Port ${config.port}`);
    if (config.path && config.path !== '/') parts.push(config.path);
    if (config.hostname) parts.push(config.hostname);
    
    return parts.join(' • ');
  }

  function getItemIcon(capability: Capability) {
    let iconName = capabilities.getIcon(getCapabilityType(capability));
    return createStyle(null, iconName).IconComponent;
  }

  function getItemIconColor(capability: Capability) {
    let colorStyle = capabilities.getColor(getCapabilityType(capability));
    return colorStyle.icon;
  }

  onMount(() => {
    loadCapabilitySchemas();
    if (selectedCapabilities.length > 0 && selectedCapabilityIndex === -1) {
      selectedCapabilityIndex = 0;
    }
  });
</script>

{#if $loading}
  <div class="h-full flex items-center justify-center">
    <div class="flex items-center gap-3 text-gray-400">
      <div class="w-5 h-5 border-2 border-gray-400 border-t-transparent rounded-full animate-spin"></div>
      Loading capability configurations...
    </div>
  </div>
{:else}
  <div class="h-full flex gap-6 min-h-0">
    <!-- Left Panel - Capability List -->
    <div class="w-2/5 flex flex-col min-h-0 overflow-hidden">
      <div class="flex-1 min-h-0 overflow-auto p-6">
        <ListManager
          label="Capabilities"
          helpText="Configure services and their monitoring tests"
          items={selectedCapabilities}
          options={capabilitySelectOptions}
          allowDuplicates={true}
          allowReorder={false}
          allowItemRemove={(selected: Capability) => !getCapabilityConfig(selected).system_assigned}
          allowDirectAdd={true}
          placeholder="Select capability type..."
          onEdit={handleEditCapability}
          onRemove={handleRemoveCapability}
          onAdd={handleAddCapability}
          highlightedIndex={selectedCapabilityIndex}
          emptyMessage="No capabilities configured. Add one to get started."
          
          {getOptionId}
          {getOptionLabel}
          {getOptionDescription}
          {getOptionIcon}
          {getOptionIconColor}

          {getItemId}
          {getItemIcon}
          {getItemIconColor}
          {getItemLabel}
          {getItemDescription}
          
        />
      </div>
    </div>

    <!-- Right Panel - Capability Configuration -->
    <div class="w-3/5 border-l border-gray-600 p-6 min-h-0 overflow-hidden">
      <CapabilitiesConfigPanel
        {form}
        capability={selectedCapability}
        schema={selectedSchema}
        onChange={handleCapabilityChange}
      />
    </div>
  </div>
{/if}