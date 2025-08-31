<script lang="ts">
  import { ChevronDown, ChevronRight, ToggleLeft, ToggleRight } from 'lucide-svelte';
  import DynamicField from '$lib/shared/components/forms/DynamicField.svelte';
  import { getCapabilityConfig, getCapabilityType, updateCapabilityConfig, type Capability } from '$lib/features/capabilities/types/base';
  import type { CapabilityConfigForm } from '$lib/features/capabilities/types/forms';
  import { createStyle } from '$lib/shared/utils/styling';

  export let capability: Capability | null = null;
  export let schema: CapabilityConfigForm | null = null;
  export let onChange: (updatedCapability: Capability) => void = () => {};

  let expandedSections: Set<string> = new Set();
  let capabilityName = '';
  let capabilityConfig: Record<string, any> = {};

  // Initialize form data when capability changes
  $: if (capability) {
    const config = getCapabilityConfig(capability);
    
    capabilityName = config.name;
    capabilityConfig = { ...config };
  }

  // Update capability when form data changes
  $: if (capability && schema) {
    const currentConfig = getCapabilityConfig(capability);
    const hasNameChanged = capabilityName !== currentConfig.name;
    const hasConfigChanged = JSON.stringify(capabilityConfig) !== JSON.stringify(currentConfig);
    
    if (hasNameChanged || hasConfigChanged) {
      const updatedCapability = updateCapabilityConfig(capability, {
        ...capabilityConfig,
        name: capabilityName
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

  function toggleTest(testIndex: number) {
    if (!capability) return;
    
    const config = getCapabilityConfig(capability);
    const updatedTests = [...config.tests];
    updatedTests[testIndex] = {
      ...updatedTests[testIndex],
      enabled: !updatedTests[testIndex].enabled
    };
    
    const updatedCapability = updateCapabilityConfig(capability, { tests: updatedTests });
    onChange(updatedCapability);
  }

  function updateTestConfig(testIndex: number, field: string, value: any) {
    if (!capability) return;
    
    const config = getCapabilityConfig(capability);
    const updatedTests = [...config.tests];
    const currentTest = updatedTests[testIndex];
    
    // Type-safe update based on known CapabilityTest fields
    if (field === 'test' || field === 'criticality') {
      updatedTests[testIndex] = {
        ...currentTest,
        [field]: value as string
      };
    } else if (field === 'enabled') {
      updatedTests[testIndex] = {
        ...currentTest,
        enabled: value as boolean
      };
    } else {
      // For any other fields, create a new object with the field
      updatedTests[testIndex] = {
        ...currentTest,
        [field]: value
      } as any;
    }
    
    const updatedCapability = updateCapabilityConfig(capability, { tests: updatedTests });
    onChange(updatedCapability);
  }

  function getConfigSummary(config: Record<string, any>): string {
    const keys = Object.keys(config);
    if (keys.length === 0) return '';
    
    const parts = [];
    if (config.port) parts.push(`Port ${config.port}`);
    if (config.path) parts.push(config.path);
    if (config.hostname) parts.push(config.hostname);
    
    return parts.join(' • ') || `${keys.length} settings`;
  }
</script>

{#if !capability || !schema}
  <div class="h-full flex items-center justify-center text-gray-400">
    <div class="text-center">
      <div class="text-lg mb-2">No capability selected</div>
      <div class="text-sm">Select a capability from the list to configure it</div>
    </div>
  </div>
{:else}
  {@const config = getCapabilityConfig(capability)}
  <div class="h-full flex flex-col">
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
        {#if !config.removable}
          <span class="text-xs px-2 py-1 bg-orange-900/20 text-orange-400 rounded border border-orange-600">
            System
          </span>
        {/if}
      </div>
      {#if schema.capability_info.description}
        <p class="text-sm text-gray-400">{schema.capability_info.description}</p>
      {/if}
    </div>

    <!-- Scrollable Content -->
    <div class="flex-1 overflow-auto space-y-6">
      <!-- Capability Name -->
      <div>
        <div class="block text-sm font-medium text-gray-300 mb-2">
          Name
          <span class="text-red-400 ml-1">*</span>
        </div>
        <input
          type="text"
          bind:value={capabilityName}
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Enter a descriptive name..."
        />
        <p class="text-xs text-gray-400 mt-1">
          Give this capability a meaningful name like "API Server" or "Admin Panel"
        </p>
      </div>

      <!-- Capability Configuration Fields -->
      {#if schema.capability_fields.length > 0}
        <div>
          <h4 class="text-sm font-medium text-gray-300 mb-4">Service Configuration</h4>
          <div class="space-y-4">
            {#each schema.capability_fields as field}
              <DynamicField
                {field}
                value={capabilityConfig[field.id]}
                onUpdate={(value) => capabilityConfig[field.id] = value}
              />
            {/each}
          </div>
        </div>
      {/if}

      <!-- Auto-Assigned Tests -->
      {#if schema.test_sections.length > 0}
        <div>
          <h4 class="text-sm font-medium text-gray-300 mb-4">Auto-Assigned Tests</h4>
          <div class="space-y-4">
            {#each schema.test_sections as section, sectionIndex}
              {@const isExpanded = expandedSections.has(section.test_type)}
              {@const testConfig = config.tests[sectionIndex]}
              {@const testStyle = createStyle(section.test_info.color, section.test_info.icon)}
              
              <div class="border border-gray-600 rounded-lg overflow-hidden">
                <!-- Test Header -->
                <div class="p-4 bg-gray-700/50">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                      <!-- Test Enable/Disable Toggle -->
                      <button
                        type="button"
                        on:click={() => toggleTest(sectionIndex)}
                        class="flex items-center"
                        title={testConfig?.enabled ? 'Disable test' : 'Enable test'}
                      >
                        {#if testConfig?.enabled}
                          <ToggleRight class="w-6 h-6 text-blue-400" />
                        {:else}
                          <ToggleLeft class="w-6 h-6 text-gray-500" />
                        {/if}
                      </button>

                      <!-- Test Info -->
                      <div class="flex items-center gap-2">
                        <svelte:component this={testStyle.IconComponent} class="w-5 h-5 {testStyle.colors.icon}" />
                        <span class="font-medium text-white">{section.test_info.display_name}</span>
                        <span class="text-xs px-2 py-1 bg-gray-800 text-gray-300 rounded">
                          {testConfig?.criticality || 'Not configured'}
                        </span>
                      </div>
                    </div>

                    <!-- Expand/Collapse Button -->
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

                <!-- Test Configuration -->
                {#if isExpanded}
                  <div class="p-4 border-t border-gray-600 bg-gray-800/30">
                    <div class="space-y-4">
                      {#each section.test_fields as field}
                        <DynamicField
                          {field}
                          value={testConfig?.[field.id as keyof typeof testConfig]}
                          onUpdate={(value) => updateTestConfig(sectionIndex, field.id, value)}
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
      {/if}

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

      <!-- Configuration Summary -->
      <div class="rounded-lg bg-gray-700/50 p-4">
        <h5 class="text-sm font-medium text-gray-300 mb-2">Configuration Summary</h5>
        <div class="text-sm text-gray-400 space-y-1">
          <div>Name: <span class="text-white">{capabilityName || 'Unnamed'}</span></div>
          <div>Type: <span class="text-white">{schema.capability_info.display_name}</span></div>
          {#if Object.keys(capabilityConfig).filter(k => k !== 'name' && k !== 'removable' && k !== 'tests' && k !== 'port' && k !== 'process' && k !== 'discovery_ports').length > 0}
            <div>Settings: <span class="text-white">{getConfigSummary(capabilityConfig)}</span></div>
          {/if}
          <div>Tests: <span class="text-white">{config.tests.filter(t => t.enabled).length}/{config.tests.length} enabled</span></div>
        </div>
      </div>
    </div>
  </div>
{/if}