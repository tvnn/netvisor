<script lang="ts">
  import { onMount } from 'svelte';
  import ListManager from '$lib/shared/components/forms/ListManager.svelte';
  import { createCapability, getCapabilityConfig, type Capability, getCapabilityType, type CapabilityConfigBase, type CapabilityConfig, getTestConfigFromSchema } from '$lib/features/capabilities/types/base';
  import type { CapabilityConfigForm } from '$lib/features/capabilities/types/forms';
  import type { NodeContext } from '$lib/features/nodes/types/base';
  import { getCapabilityForms } from '$lib/features/capabilities/store';
  import { createStyle } from '$lib/shared/utils/styling';
	import CapabilitiesConfigPanel from './CapabilitiesConfigPanel.svelte';
	import { loading, pushError } from '$lib/shared/stores/feedback';
	import { capabilities } from '$lib/shared/stores/registry';
	import type { TagProps } from '$lib/shared/components/data/types';

  export let selectedCapabilities: Capability[] = [];
  export let nodeContext: NodeContext;

  // Component state
  let availableSchemas: Record<string, CapabilityConfigForm> = {};
  let selectedCapabilityIndex = -1;

  // Computed values
  $: selectedCapability = selectedCapabilityIndex >= 0 ? selectedCapabilities[selectedCapabilityIndex] : null;
  $: selectedCapabilityType = selectedCapability ? getCapabilityType(selectedCapability) : null;
  $: selectedSchema = selectedCapabilityType ? availableSchemas[selectedCapabilityType] : null;

  // Available capability types for dropdown
  $: selectOptions = Object.keys(availableSchemas)
    .map(type => capabilityFromType(type))  
    .filter(type => !getCapabilityConfig(type).system_assigned)

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
    const baseConfig: CapabilityConfigBase = {
      name: capabilities.getDisplay(capabilityType),
      tests: schema.test_sections.map(section => ({
        test: {
          type: section.test_type,
          config: getTestConfigFromSchema(section)
        },
        criticality: section.test_fields.find(f => f.id === 'criticality')?.default_value,
        enabled: section.enabled_by_default ?? false
      })),
      system_assigned: schema.system_assigned ?? false,
      port: undefined,
      process: undefined,
      discovery_ports: undefined
    };

    let config: CapabilityConfig = {
      ...baseConfig
    }

    // Add capability-specific default values from schema
    schema.capability_fields.forEach(field => {
      if (field.default_value !== undefined) {
        config[field.id] = field.default_value;
      }
    });

    return createCapability(capabilityType, config)
  }

  function handleAddCapability(capabilityType: string) {  
    selectedCapabilities = [...selectedCapabilities, capabilityFromType(capabilityType)];
    selectedCapabilityIndex = selectedCapabilities.length - 1;
  }

  function handleRemoveCapability(index: number) {
    const capability = selectedCapabilities[index];
    // const config = getCapabilityConfig(capability);
    
    // selectedCapabilities = selectedCapabilities.filter((_, i) => i !== index);
    
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

  function getItemId(capability: Capability): string {
    return getCapabilityType(capability)
  }

  // Display functions for ListManager

  // Options
  function getOptionLabel(capability: Capability): string {
    return capabilities.getDisplay( getCapabilityType(capability) );
  }

  function getOptionDescription(capability: Capability) {
    return capabilities.getDescription( getCapabilityType(capability) );
  }

  // Items
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

  function getItemTags(capability: Capability): TagProps[] {
    const tags = [];
    const config = getCapabilityConfig(capability);
        
    // Test status tag
    const enabledTests = config.tests.filter(t => t.enabled).length;
    const totalTests = config.tests.length;
    if (totalTests > 0) {
      tags.push({
        label: `${enabledTests}/${totalTests} tests enabled`,
        textColor: enabledTests === totalTests ? 'text-green-400' : 'text-yellow-400',
        bgColor: enabledTests === totalTests ? 'bg-green-900/20' : 'bg-yellow-900/20'
      });
    }
    
    return tags;
  }

  function getItemIcon(capability: Capability) {
    let iconName = capabilities.getIcon( getCapabilityType(capability) );
    return createStyle(null, iconName).IconComponent;
  }

  function getItemIconColor(capability: Capability) {
    let colorStyle = capabilities.getColor( getCapabilityType(capability) );
    return colorStyle.icon;
  }

  onMount(() => {
    // Auto-expand first section on mount
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
    <div class="w-2/5 flex flex-col min-h-0">
      <!-- Add New Capability -->
      <div class="flex-1 min-h-0">
        <ListManager
          label="Capabilities"
          helpText="Configure services and their monitoring tests"
          bind:items={selectedCapabilities}
          options={selectOptions}
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
          
          getOptionId={getItemId}
          getOptionIcon={getItemIcon}
          getOptionIconColor={getItemIconColor}
          {getOptionLabel}
          {getOptionDescription}

          {getItemId}
          {getItemIcon}
          {getItemIconColor}
          {getItemLabel}
          {getItemDescription}
          {getItemTags}
        />
      </div>
    </div>

    <!-- Right Panel - Capability Configuration -->
    <div class="w-3/5 border-l border-gray-600 pl-6 min-h-0">
      <CapabilitiesConfigPanel
        capability={selectedCapability}
        schema={selectedSchema}
        onChange={handleCapabilityChange}
      />
    </div>
  </div>
{/if}