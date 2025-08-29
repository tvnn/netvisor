<script lang="ts">
  import { Server } from 'lucide-svelte';
  import { testTypes } from '$lib/api/registry';
  import RichSelect from '../../../common/forms/RichSelect.svelte'
  
  export let selectedTestType: string;
  export let onTestTypeChange: (testType: string) => void;
  export let schemaCache: Map<string, any> = new Map(); // Schema cache from parent
  
  $: types = $testTypes.sort((a, b) => {
    const aSchema = schemaCache.get(a.id);
    const bSchema = schemaCache.get(b.id);
    if (aSchema?.compatibility === 'Compatible' && bSchema?.compatibility !== 'Compatible') return -1;
    if (bSchema?.compatibility === 'Compatible' && aSchema?.compatibility !== 'Compatible') return 1;
    return 0;
  });
  
  // Transform test types into RichSelectOption format
  $: richOptions = types.map(testType => ({
    value: testType.id,
    label: testType.display_name,
    description: testType.description,
    disabled: false,
    metadata: {
      category: testType.category,
      color: testType.color,
      schema: schemaCache.get(testType.id)
    }
  }));
  
  function getOptionIcon(option: any) {
    return Server;
  }
  
  function getOptionIconColor(option: any) {
    return option.metadata?.color || 'text-gray-300';
  }
  
  function getOptionBadge(option: any) {
    return option.metadata?.category || null;
  }
  
  function getOptionBadgeColor(option: any) {
    return 'bg-gray-600 text-gray-300';
  }
  
  function getOptionTag(option: any) {
    const schema = option.metadata?.schema;
    if (!schema) return null;
    
    const status = schema.compatibility;
    switch (status) {
      case 'Compatible':
        return {
          text: status,
          textColor: 'text-green-300',
          bgColor: 'bg-green-900/30'
        };
      case 'Conditional':
        return {
          text: status,
          textColor: 'text-yellow-300',
          bgColor: 'bg-yellow-900/30'
        };
      case 'Incompatible':
        return {
          text: status,
          textColor: 'text-red-300',
          bgColor: 'bg-red-900/30'
        };
      default:
        return null;
    }
  }
  
  function getOptionStatusText(option: any) {
    const schema = option.metadata?.schema;
    return schema?.compatibility_reason || null;
  }
</script>

<RichSelect
  label="Test Type"
  selectedValue={selectedTestType}
  options={richOptions}
  placeholder="Select a test type..."
  required={true}
  onSelect={onTestTypeChange}
  {getOptionIcon}
  {getOptionIconColor}
  {getOptionBadge}
  {getOptionBadgeColor}
  {getOptionTag}
  {getOptionStatusText}
/>