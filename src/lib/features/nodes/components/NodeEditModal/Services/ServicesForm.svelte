<script lang="ts">
  import { onMount } from 'svelte';
  import type { Capability, CapabilityConfig } from '$lib/features/capabilities/types/base';
  import type { NodeContext } from '$lib/features/nodes/types/base';
  import type { CapabilityConfigForm } from '$lib/features/capabilities/types/forms';
  import { getCapabilityConfig, getCapabilityType, createCapability } from '$lib/features/capabilities/types/base';
  import CapabilitiesConfigPanel from './ServicesConfigPanel.svelte';
  import { capabilities } from '$lib/shared/stores/registry';
  import { createStyle } from '$lib/shared/utils/styling';
  import { getCapabilityForms } from '$lib/features/capabilities/store';
  import { loading, pushError } from '$lib/shared/stores/feedback';
  import ListConfigEditor from '$lib/shared/components/forms/ListConfigEditor.svelte';
  
  export let form: any;
  export let selectedCapabilities: Capability[] = [];
  export let nodeContext: NodeContext;
  
  let availableSchemas: Record<string, CapabilityConfigForm> = {};
  
  // Available capability types for dropdown
  $: capabilitySelectOptions = Object.keys(availableSchemas)
    .filter(type => !availableSchemas[type]?.system_assigned);

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

  function capabilityFromType(capabilityType: string) {
    const schema = availableSchemas[capabilityType]
    const baseConfig = {
      name: capabilities.getDisplay(capabilityType),
      // tests: schema?.test_sections?.map(section => ({
      //   test: {
      //     type: section.test_type,
      //     config: getTestConfigFromSchema(section)
      //   },
      //   criticality: section.test_fields.find(f => f.id === 'criticality')?.default_value || 'Important',
      //   enabled: section.enabled_by_default ?? false
      // })) || [],
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
    console.log('handleAddCapability called with:', capabilityType);
    const newCapability = capabilityFromType(capabilityType);
    console.log('Created capability:', newCapability);
    selectedCapabilities = [...selectedCapabilities, newCapability];
    console.log('Updated selectedCapabilities length:', selectedCapabilities.length);
  }

  function handleCapabilityChange(updatedCapability: Capability, index: number) {
    if (index >= 0 && index < selectedCapabilities.length) {
      selectedCapabilities[index] = updatedCapability;
      selectedCapabilities = selectedCapabilities; // Trigger reactivity
    }
  }

  // Display functions for options (dropdown)
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

  // Display functions for items (list)
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
  });
</script>

<ListConfigEditor
  {form}
  bind:items={selectedCapabilities}
  options={capabilitySelectOptions}
  loading={$loading}
  label="Capabilities"
  helpText="Configure services and their monitoring tests"
  
  allowDuplicates={true}
  allowItemRemove={(selected) => !getCapabilityConfig(selected).system_assigned}
  emptyMessage="No capabilities configured. Add one to get started."
  
  {getOptionId}
  {getOptionLabel}
  {getOptionDescription}
  {getOptionIcon}
  {getOptionIconColor}
  
  {getItemId}
  {getItemLabel}
  {getItemDescription}
  {getItemIcon}
  {getItemIconColor}
  
  onAdd={handleAddCapability}
  onChange={handleCapabilityChange}
>
  <CapabilitiesConfigPanel
    slot="config"
    let:selectedItem
    let:selectedIndex
    let:onChange
    
    {form}
    capability={selectedItem}
    schema={selectedItem ? availableSchemas[getCapabilityType(selectedItem)] : null}
    onChange={(updatedCapability) => onChange(updatedCapability)}
  />
</ListConfigEditor>