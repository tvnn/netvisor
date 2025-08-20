<script lang="ts">
  import { onMount } from 'svelte';
  import { 
    AlertCircle, 
    AlertTriangle, 
    ChevronDown, 
    ChevronUp,
    Loader2,
    X,
    Plus,
    Settings
  } from 'lucide-svelte';
  import { fetchTestSchema } from '$lib/stores/schema';
  import type { TestConfigSchema } from '$lib/stores/schema';
  import type { AssignedTest, NodeFormData } from '$lib/types/nodes';
  import DynamicField from './DynamicField.svelte';
  import CompatibilityIndicator from './CompatibilityIndicator.svelte';
  import TestTypeDropdown from './TestTypeDropdown.svelte';
  
  export let test: AssignedTest | null = null;
  export let node: NodeFormData;
  export let onClose: () => void;
  export let onChange: (test: AssignedTest) => void;
  
  let schema: TestConfigSchema | null = null;
  let loading = false;
  let error: string | null = null;
  let formData: Record<string, any> = {};
  let validationErrors: Record<string, string> = {};
  let showAdvanced = false;
  let mounted = false;
  
  onMount(() => {
    mounted = true;
  });
  
  // Convert NodeFormData to the simple context needed by schema API
  $: nodeContext = {
    node_id: undefined,
    node_type: node.node_type,
    capabilities: node.capabilities,
    target: node.target,
    assigned_tests: node.assigned_tests.map(t => t.test.type),
  };
  
  // Load schema when test type changes
  $: if (mounted && test?.test.type) {
    loadSchemaForTest(test.test.type);
  }
  
  // Initialize form data from test config
  $: if (test?.test.config) {
    formData = { ...formData, ...test.test.config };
  }
  
  async function loadSchemaForTest(testType: string) {
    if (!testType) return;
    
    loading = true;
    error = null;
    schema = null;
    
    try {
      schema = await fetchTestSchema(testType, nodeContext);
      
      // Initialize form data with defaults merged with existing values
      const defaults = Object.fromEntries(
        schema.fields
          .filter(field => field.default_value !== null && field.default_value !== undefined)
          .map(field => [field.id, field.default_value])
      );
      
      formData = { ...defaults, ...(test?.test.config || {}) };
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load test configuration';
    } finally {
      loading = false;
    }
  }
  
  $: basicFields = schema?.fields.filter(f => !f.advanced) || [];
  $: advancedFields = schema?.fields.filter(f => f.advanced) || [];
  
  function handleTestTypeChange(newTestType: string) {
    if (!test) return;
    
    // Update test type and clear config when type changes
    const updatedTest: AssignedTest = {
      ...test,
      test: {
        type: newTestType as any,
        config: {}
      }
    };
    onChange(updatedTest);
  }
  
  function handleCriticalityChange(event: Event) {
    if (!test) return;
    
    const target = event.target as HTMLSelectElement;
    const updatedTest: AssignedTest = {
      ...test,
      criticality: target.value as any
    };
    onChange(updatedTest);
  }
  
  function updateField(fieldId: string, value: any) {
    formData = { ...formData, [fieldId]: value };
    
    // Clear validation error for this field
    if (validationErrors[fieldId]) {
      const { [fieldId]: removed, ...rest } = validationErrors;
      validationErrors = rest;
    }
    
    // Validate field
    const field = schema?.fields.find(f => f.id === fieldId);
    if (field) {
      const errorMsg = validateField(field, value);
      if (errorMsg) {
        validationErrors = { ...validationErrors, [fieldId]: errorMsg };
      }
    }
    
    // Real-time update - save immediately
    if (test) {
      const updatedTest: AssignedTest = {
        ...test,
        test: {
          ...test.test,
          config: formData
        }
      };
      onChange(updatedTest);
    }
  }
  
  function validateField(field: any, value: any): string | null {
    if (field.required && (!value || value === '')) {
      return `${field.label} is required`;
    }
    
    if (value && value !== '') {
      switch (field.field_type.base_type) {
        case 'integer':
          const num = parseInt(value);
          if (isNaN(num)) return 'Must be a valid number';
          if (field.field_type.constraints.min && num < field.field_type.constraints.min) {
            return `Must be at least ${field.field_type.constraints.min}`;
          }
          if (field.field_type.constraints.max && num > field.field_type.constraints.max) {
            return `Must be at most ${field.field_type.constraints.max}`;
          }
          break;
        case 'string':
          if (field.id === 'expected_ip' && !isValidIpAddress(value)) {
            return 'Must be a valid IP address';
          }
          if (field.id === 'domain' && !isValidDomain(value)) {
            return 'Must be a valid domain name';
          }
          if (field.id === 'expected_subnet' && !isValidCidr(value)) {
            return 'Must be a valid CIDR subnet (e.g., 10.100.0.0/24)';
          }
          break;
      }
    }
    
    return null;
  }
  
  function isValidIpAddress(ip: string): boolean {
    const ipv4Regex = /^(\d{1,3}\.){3}\d{1,3}$/;
    if (!ipv4Regex.test(ip)) return false;
    const parts = ip.split('.');
    return parts.every(part => {
      const num = parseInt(part);
      return num >= 0 && num <= 255;
    });
  }
  
  function isValidDomain(domain: string): boolean {
    const domainRegex = /^[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.([a-zA-Z]{2,}|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])$/;
    return domainRegex.test(domain) || domain === 'localhost';
  }
  
  function isValidCidr(cidr: string): boolean {
    const cidrRegex = /^(\d{1,3}\.){3}\d{1,3}\/\d{1,2}$/;
    if (!cidrRegex.test(cidr)) return false;
    const [ip, prefix] = cidr.split('/');
    const prefixNum = parseInt(prefix);
    return isValidIpAddress(ip) && prefixNum >= 0 && prefixNum <= 32;
  }
</script>

<div class="h-full">
  {#if !test}
    <!-- Empty state - prompt to create or select a test -->
    <div class="h-full flex items-center justify-center">
      <div class="text-center max-w-sm">
        <div class="w-16 h-16 bg-gray-700 rounded-full flex items-center justify-center mx-auto mb-4">
          <Settings class="w-8 h-8 text-gray-400" />
        </div>
        <h3 class="text-lg font-medium text-white mb-2">Configure Test</h3>
        <p class="text-sm text-gray-400 mb-4">
          Select a test from the list to configure it, or click "Add Test" to create a new one.
        </p>
        <div class="flex items-center justify-center gap-2 text-xs text-gray-500">
          <Plus class="w-3 h-3" />
          <span>Click "Add Test" to get started</span>
        </div>
      </div>
    </div>
  {:else}
    <!-- Test configuration form -->
    <div class="space-y-6">
      <!-- Header with close button -->
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-medium text-white">Configure Test</h3>
        <button
          on:click={onClose}
          class="text-gray-400 hover:text-white transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>
      
      <!-- Test Type Selection -->
      <TestTypeDropdown
        selectedTestType={test.test.type}
        onTestTypeChange={handleTestTypeChange}
      />

      {#if schema}
        <CompatibilityIndicator {schema} />
      {/if}
      
      <!-- Criticality Selection -->
      <div>
        <div class="block text-sm font-medium text-gray-300 mb-2">
          Test Criticality
        </div>
        <select
          value={test.criticality}
          on:change={handleCriticalityChange}
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white 
                 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="Critical">Critical</option>
          <option value="Important">Important</option>
          <option value="Informational">Informational</option>
        </select>
        <p class="text-xs text-gray-400 mt-1">
          Critical tests affect node status when they fail. Important tests show as degraded.
        </p>
      </div>
      
      {#if loading}
        <!-- Loading State -->
        <div class="flex items-center justify-center py-8">
          <div class="flex items-center gap-3 text-gray-400">
            <Loader2 class="w-5 h-5 animate-spin" />
            Loading test configuration...
          </div>
        </div>

      {:else if error}
        <!-- Error State -->
        <div class="p-4 bg-red-900/20 border border-red-600 rounded-lg">
          <div class="flex items-center gap-2">
            <AlertCircle class="w-4 h-4 text-red-400" />
            <span class="text-sm text-red-200">Failed to load test configuration</span>
          </div>
          <p class="text-xs text-red-300 mt-1">{error}</p>
        </div>

      {:else if schema}        
        {#if schema.compatibility === 'Compatible'}
          <!-- Basic Configuration -->
          {#if basicFields.length > 0}
            <div class="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
              <h4 class="text-sm font-medium text-gray-200 mb-4">Configuration</h4>
              
              <div class="space-y-4">
                {#each basicFields as field}
                  <DynamicField
                    {field}
                    value={formData[field.id]}
                    error={validationErrors[field.id]}
                    onUpdate={(value) => updateField(field.id, value)}
                  />
                {/each}
              </div>
            </div>
          {/if}
          
          <!-- Advanced Configuration -->
          {#if advancedFields.length > 0}
            <div class="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
              <button
                type="button"
                on:click={() => showAdvanced = !showAdvanced}
                class="flex items-center justify-between w-full text-left"
              >
                <h4 class="text-sm font-medium text-gray-200">Advanced Options</h4>
                {#if showAdvanced}
                  <ChevronUp class="w-4 h-4 text-gray-400" />
                {:else}
                  <ChevronDown class="w-4 h-4 text-gray-400" />
                {/if}
              </button>
              
              {#if showAdvanced}
                <div class="space-y-4 mt-4">
                  {#each advancedFields as field}
                    <DynamicField
                      {field}
                      value={formData[field.id]}
                      error={validationErrors[field.id]}
                      onUpdate={(value) => updateField(field.id, value)}
                    />
                  {/each}
                </div>
              {/if}
            </div>
          {/if}
          
          <!-- Validation Messages -->
          {#if schema.warnings.length > 0}
            <div class="space-y-2">
              {#each schema.warnings as warning}
                <div class="flex items-center gap-2 p-3 bg-yellow-900/20 border border-yellow-600 rounded-lg">
                  <AlertTriangle class="w-4 h-4 text-yellow-400" />
                  <span class="text-sm text-yellow-200">{warning.message}</span>
                </div>
              {/each}
            </div>
          {/if}
        {/if}
      {/if}
    </div>
  {/if}
</div>