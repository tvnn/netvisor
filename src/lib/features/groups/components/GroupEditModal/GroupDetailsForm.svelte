<script lang="ts">
  import { onMount } from 'svelte';
  import { AlertCircle } from 'lucide-svelte';
  import { field } from 'svelte-forms';
  import { required } from 'svelte-forms/validators';
  import type { Host } from '$lib/features/hosts/types/base';
  import { maxLength } from '$lib/shared/components/forms/validators';
	import { entities } from '$lib/shared/stores/metadata';
	import type { FieldType, FormApi, FormType } from '$lib/shared/components/forms/types';
	import type { Group } from '../../types/base';
	import TextInput from '$lib/shared/components/forms/input/TextInput.svelte';
	import TextArea from '$lib/shared/components/forms/input/TextArea.svelte';
  
  export let formApi: FormApi;
  export let form: FormType;
  export let formData: Group;
  
  // Create form fields with validation
  const name = field('name', formData.name, [required(), maxLength(100)]);
  const description = field('description', formData.description || '', [maxLength(500)]);
    
  // Update formData when field values change
  $: formData.name = $name.value;
  $: formData.description = $description.value;
  
</script>

<!-- Basic Information -->
<div class="space-y-4">
    <h3 class="text-lg font-medium text-white">Group Details</h3>
    
    <TextInput 
      label="Group Name" 
      id="name" 
      {formApi}
      placeholder="e.g., DNS Resolution Path, Web Access Chain"
      required={true}
      field={name} />

    <TextArea
      label="Description" 
      id="description" 
      {formApi}
      placeholder="Describe the data flow or purpose of this service chain..."
      field={description} />
</div>