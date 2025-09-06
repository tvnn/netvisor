<script lang="ts">
  import { ChevronDown, ChevronRight, ToggleLeft, ToggleRight, AlertCircle } from 'lucide-svelte';
  import { field } from 'svelte-forms';
  import { getCapabilityConfig, getCapabilityType, updateCapabilityConfig, type Capability } from '$lib/features/capabilities/types/base';
  import type { CapabilityConfigForm } from '$lib/features/capabilities/types/forms';
  import { createStyle } from '$lib/shared/utils/styling';
  import { criticalityLevels } from '$lib/shared/stores/registry';
  import Tag from '$lib/shared/components/data/Tag.svelte';
  import { capabilityName } from '$lib/shared/components/forms/validators';
	import DynamicField from '$lib/shared/components/forms/DynamicField.svelte';

  export let form: any;
  export let capability: Capability | null = null;
  export let schema: CapabilityConfigForm | null = null;
  export let onChange: (updatedCapability: Capability) => void = () => {};

  let expandedSections: Set<string> = new Set();
  let capabilityNameField: any;
  let capabilityConfig: Record<string, any> = {};

  // Initialize form fields for capability name
  $: if (capability && schema) {
    const config = getCapabilityConfig(capability);
    const isSystemAssigned = config.system_assigned;
    
    capabilityNameField = field(
      `capability_name_${getCapabilityType(capability)}`, 
      config.name, 
      [capabilityName(isSystemAssigned)]
    );
    
    if (form && !isSystemAssigned) {
      form[`capability_name_${getCapabilityType(capability)}`] = capabilityNameField
    }
  }

  // Initialize form data when capability changes
  $: if (capability) {
    const config = getCapabilityConfig(capability);
    capabilityConfig = { ...config };
    
    // Update capability name field value if it exists
    if (capabilityNameField && $capabilityNameField.value !== config.name) {
      capabilityNameField.set(config.name);
    }
  }

  // Update capability when form data changes
  $: if (capability && schema) {
    const currentConfig = getCapabilityConfig(capability);
    const newName = capabilityNameField ? $capabilityNameField.value : currentConfig.name;
    const hasNameChanged = newName !== currentConfig.name;
    const hasConfigChanged = JSON.stringify(capabilityConfig) !== JSON.stringify(currentConfig);
    
    if (hasNameChanged || hasConfigChanged) {
      const updatedCapability = updateCapabilityConfig(capability, {
        ...capabilityConfig,
        name: newName
      });
      onChange(updatedCapability);
    }
  }

  function toggleSection(sectionId: string) {
    if (expandedSections.has(sectionId)) {
      expandedSections.delete(sectionId);
    } else {
      expandedSections.add(sectionId);
    }
    expandedSections = expandedSections; // Trigger reactivity
  }

  function updateTest(sectionIndex: number, updates: Partial<{
    enabled: boolean;
    criticality: string;
    config: Record<string, any>;
  }>) {
    if (!capability || !schema) return;
    
    const capabilityConfig = getCapabilityConfig(capability);
    // const updatedTests = [...capabilityConfig.tests];
    // const section = schema.test_sections[sectionIndex];
    
    // // Ensure test exists with proper structure
    // if (!updatedTests[sectionIndex]) {
    //   updatedTests[sectionIndex] = {
    //     test: {
    //       type: section.test_type,
    //       config: getTestConfigFromSchema(section)
    //     },
    //     criticality: section.test_fields.find(f => f.id === 'criticality')?.default_value || 'Important',
    //     enabled: section.enabled_by_default
    //   };
    // }
    
    // Apply updates
    // if (updates.enabled !== undefined) {
    //   updatedTests[sectionIndex].enabled = updates.enabled;
    // }
    
    // if (updates.criticality !== undefined) {
    //   updatedTests[sectionIndex].criticality = updates.criticality;
    // }
    
    // if (updates.config) {
    //   updatedTests[sectionIndex].test.config = {
    //     ...updatedTests[sectionIndex].test.config,
    //     ...updates.config
    //   };
    // }
    
    // Update the capability
    const updatedCapability = updateCapabilityConfig(capability, {
      ...capabilityConfig,
      // tests: updatedTests
    });
    
    onChange(updatedCapability);
  }

  function toggleTest(sectionIndex: number) {
    if (!capability || !schema) return;
    const config = getCapabilityConfig(capability);
    const currentTest = config.tests[sectionIndex];
    const newEnabledState = currentTest ? !currentTest.enabled : true;
    
    updateTest(sectionIndex, { enabled: newEnabledState });
  }

  function updateTestConfig(sectionIndex: number, fieldId: string, value: any) {
    if (fieldId === 'criticality') {
      updateTest(sectionIndex, { criticality: value });
    } else {
      // This is a test config field
      updateTest(sectionIndex, { 
        config: { [fieldId]: value } 
      });
    }
  }
</script>

{#if !capability || !schema}
  <div class="flex-1 min-h-0 flex items-center justify-center text-gray-400">
    <div class="text-center">
      <div class="text-lg mb-2">No capability selected</div>
      <div class="text-sm">Select a capability from the list to configure it</div>
    </div>
  </div>
{:else}
  {@const config = getCapabilityConfig(capability)}
  <div class="h-full flex flex-col min-h-0">
    <!-- Header -->
    <div class="border-b border-gray-600 pb-4 mb-6">
      <div class="flex items-center gap-3 mb-2">
        {#if schema.capability_info.icon}
          {@const iconStyle = createStyle(schema.capability_info.color, schema.capability_info.icon)}
          <svelte:component this={iconStyle.IconComponent} class="w-6 h-6 {iconStyle.colors.icon}" />
        {/if}
        <h3 class="text-lg font-medium text-white">
          {schema.capability_info.display_name}
        </h3>
      </div>
      {#if schema.capability_info.description}
        <p class="text-sm text-gray-400">{schema.capability_info.description}</p>
      {/if}
    </div>

    <!-- Scrollable Content -->
    <div class="flex-1 overflow-auto space-y-6 min-h-0">
      <!-- Capability Name -->
      {#if !getCapabilityConfig(capability).system_assigned && capabilityNameField}
        <div class="space-y-2">
          <label for="capability_name" class="block text-sm font-medium text-gray-300">
            Name <span class="text-red-400 ml-1">*</span>
          </label>
          <input
            id="capability_name"
            type="text"
            bind:value={$capabilityNameField.value}
            class="w-full px-3 py-2 bg-gray-700 border rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500
                   {$capabilityNameField.errors.length > 0 ? 'border-red-500' : 'border-gray-600'}"
            placeholder="Enter a descriptive name..."
          />
          {#if $capabilityNameField.errors.length > 0}
            <div class="flex items-center gap-2 text-red-400">
              <AlertCircle size={16} />
              <p class="text-xs">{$capabilityNameField.errors[0]}</p>
            </div>
          {/if}
          <p class="text-xs text-gray-400">
            Give this capability a meaningful name like "API Server" or "Admin Panel"
          </p>
        </div>
      {/if}

      <!-- Capability Configuration Fields -->
      {#if schema.capability_fields.length > 0}
        <div>
          <h4 class="text-sm font-medium text-gray-300 mb-4">Configuration</h4>
          <div class="space-y-4">
            {#each schema.capability_fields as field}
              <DynamicField
                {form}
                {field}
                fieldId={`${getCapabilityType(capability)}_${field.id}`}
                value={capabilityConfig[field.id]}
                onUpdate={(value: any) => capabilityConfig[field.id] = value}
              />
            {/each}
          </div>
        </div>
      {/if}

      <!-- Tests -->
      <!-- {#if schema.test_sections.length > 0}
        <div>
          <h4 class="text-sm font-medium text-gray-300 mb-4">Tests</h4>
          <div class="space-y-4">
            {#each schema.test_sections as section, sectionIndex}
              {@const isExpanded = expandedSections.has(section.test_type)}
              {@const testConfig = config.tests[sectionIndex]}
              {@const testStyle = createStyle(section.test_info.color, section.test_info.icon)}
              
              <div class="border border-gray-600 rounded-lg overflow-hidden">

                <div class="p-4 bg-gray-700/50">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">

                      <button
                        type="button"
                        on:click={() => toggleTest(sectionIndex)}
                        class="flex items-center"
                        title={testConfig?.enabled ? 'Disable test' : 'Enable test'}
                      >
                        {#if testConfig?.enabled}
                          <ToggleRight class="w-8 h-8 text-green-400" />
                        {:else}
                          <ToggleLeft class="w-8 h-8 text-gray-500" />
                        {/if}
                      </button>


                      <div class="flex items-center gap-3">
                        <svelte:component this={testStyle.IconComponent} class="w-8 h-8 {testStyle.colors.icon}" />
                        <div class="flex-col">
                          <div class="flex items-center gap-2">
                            <span class="font-medium text-white">{section.test_info.display_name}</span>
                            <Tag 
                              bgColor={criticalityLevels.getColor(testConfig?.criticality || 'Important').bg}
                              textColor={criticalityLevels.getColor(testConfig?.criticality || 'Important').text}
                              label={testConfig?.criticality || 'Important'} />
                          </div>
                          <span class="text-sm text-gray-400">{section.test_info.description}</span>
                        </div>
                      </div>
                    </div>


                    <button
                      type="button"
                      on:click={() => toggleSection(section.test_type)}
                      class="p-1 text-gray-400 hover:text-white hover:bg-gray-600 rounded"
                      title={isExpanded ? 'Collapse' : 'Expand'}
                    >
                      {#if isExpanded}
                        <ChevronDown class="w-4 h-4" />
                      {:else}
                        <ChevronRight class="w-4 h-4" />
                      {/if}
                    </button>
                  </div>

                  {#if section.description}
                    <p class="text-sm text-gray-400 mt-2">{section.description}</p>
                  {/if}
                </div>


                {#if isExpanded}
                  <div class="p-4 border-t border-gray-600 bg-gray-800/30">
                    <div class="space-y-4">
                      {#each section.test_fields as field}
                        <DynamicField
                          {form}
                          {field}
                          fieldId={`${getCapabilityType(capability)}_${section.test_type}_${field.id}`}
                          value={testConfig?.[field.id as keyof typeof testConfig]}
                          onUpdate={(value: any) => updateTestConfig(sectionIndex, field.id, value)}
                          disabled={!testConfig?.enabled}
                        />
                      {/each}
                    </div>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/if} -->

      <!-- Warnings and Errors -->
      {#if schema.warnings.length > 0}
        <div class="rounded-lg bg-yellow-900/20 border border-yellow-600 p-4">
          <h5 class="text-sm font-medium text-yellow-400 mb-2">Warnings</h5>
          <div class="space-y-1">
            {#each schema.warnings as warning}
              <p class="text-sm text-yellow-300">{warning.message}</p>
            {/each}
          </div>
        </div>
      {/if}

      {#if schema.errors.length > 0}
        <div class="rounded-lg bg-red-900/20 border border-red-600 p-4">
          <h5 class="text-sm font-medium text-red-400 mb-2">Errors</h5>
          <div class="space-y-1">
            {#each schema.errors as error}
              <p class="text-sm text-red-300">{error.message}</p>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}