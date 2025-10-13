<script lang="ts">
	import { field } from 'svelte-forms';
	import { required } from 'svelte-forms/validators';
	import { maxLength } from '$lib/shared/components/forms/validators';
	import type { FormApi } from '$lib/shared/components/forms/types';
	import type { Group } from '../../types/base';
	import TextInput from '$lib/shared/components/forms/input/TextInput.svelte';
	import TextArea from '$lib/shared/components/forms/input/TextArea.svelte';
	import { groupTypes } from '$lib/shared/stores/metadata';

	export let formApi: FormApi;
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
		field={name}
	/>

	<!-- Subnet Type -->
	<label for="group_type" class="mb-2 block text-sm font-medium text-gray-300">
		Network Type
	</label>
	<select
		id="group_type"
		bind:value={formData.group_type}
		class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-white focus:border-transparent focus:outline-none focus:ring-2"
	>
		{#each groupTypes.getItems() as group_type (group_type.id)}
			<option value={group_type.id}>{group_type.name}</option>
		{/each}
	</select>
	<p class="text-xs text-gray-400">{groupTypes.getDescription(formData.group_type)}</p>

	<TextArea
		label="Description"
		id="description"
		{formApi}
		placeholder="Describe the data flow or purpose of this service chain..."
		field={description}
	/>
</div>
