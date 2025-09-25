<script lang="ts">
  import { onMount } from 'svelte';
  import { AlertCircle } from 'lucide-svelte';
  import { field } from 'svelte-forms';
  import { required } from 'svelte-forms/validators';
  import type { Host } from '$lib/features/hosts/types/base';
  import { hostname as hostnameValidator, maxLength } from '$lib/shared/components/forms/validators';
  import TargetConfigForm from './TargetConfigForm.svelte';
	import { entities } from '$lib/shared/stores/metadata';
	import type { FormApi, FormType } from '$lib/shared/components/forms/types';
	import TextInput from '$lib/shared/components/forms/input/TextInput.svelte';
	import TextArea from '$lib/shared/components/forms/input/TextArea.svelte';
	import EntityMetadataSection from '$lib/shared/components/forms/EntityMetadataSection.svelte';
  
  export let formApi: FormApi;
  export let form: FormType;
  export let formData: Host;
  export let isEditing: boolean;
  
  // Create form fields with validation
  const name = field('name', formData.name, [required(), maxLength(100)]);
  const description = field('description', formData.description || '', [maxLength(500)]);
  const hostname = field('hostname', formData.hostname || '', [hostnameValidator()]);
  
  // Update formData when field values change
  $: formData.name = $name.value;
  $: formData.description = $description.value;
  $: formData.hostname = $hostname.value;
</script>

<div class="space-y-6 p-6">    
  <div class="grid grid-cols-2 gap-6">

    <TextInput 
      label="Name" 
      id="name" 
      {formApi}
      placeholder="Enter a name for this host..."
      required={true}
      field={name} />

    <TextInput 
      label="Hostname" 
      id="hostname" 
      {formApi}
      placeholder="api.example.com"
      field={hostname} />

  </div>

  <TextArea
      label="Description" 
      id="description" 
      {formApi}
      placeholder="Describe what this host does or its role in your infrastructure"
      field={description} />
  
  <TargetConfigForm 
    {formApi}
    {form}
    {formData}
  />
  
  {#if isEditing}
    <EntityMetadataSection id={formData.id} createdAt={formData.created_at} updatedAt={formData.updated_at}/>
  {/if}
</div>