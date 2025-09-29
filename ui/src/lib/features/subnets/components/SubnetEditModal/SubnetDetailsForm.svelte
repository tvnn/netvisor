<script lang="ts">
	import { field } from 'svelte-forms';
	import { required } from 'svelte-forms/validators';
	import { cidr as cidrValidator, maxLength } from '$lib/shared/components/forms/validators';
	import { subnetTypes } from '$lib/shared/stores/metadata';
	import type { FormApi } from '$lib/shared/components/forms/types';
	import TextInput from '$lib/shared/components/forms/input/TextInput.svelte';
	import TextArea from '$lib/shared/components/forms/input/TextArea.svelte';
	import { isContainerSubnet } from '../../store';

	export let formApi: FormApi;
	export let formData: Subnet;

	// Create form fields with validation
	const name = field('name', formData.name, [required(), maxLength(100)]);
	const cidr = field('cidr', formData.cidr, [required(), cidrValidator()]);
	const description = field('description', formData.description || '', [maxLength(500)]);

	// Update formData when field values change
	$: formData.name = $name.value;
	$: formData.description = $description.value;
</script>

<!-- Basic Information -->
<div class="space-y-4">
	<h3 class="text-lg font-medium text-white">Subnet Details</h3>

	<TextInput
		label="Name"
		id="name"
		{formApi}
		placeholder="e.g., Home LAN, VPN Network"
		required={true}
		field={name}
	/>

	<TextInput
		label="CIDR"
		id="name"
		{formApi}
		disabled={!!isContainerSubnet(formData.id)}
		placeholder="192.168.1.0/24"
		helpText="Network address and prefix length (e.g., 192.168.1.0/24)"
		required={true}
		field={cidr}
	/>

	<!-- Subnet Type -->
	<label for="subnet_type" class="mb-2 block text-sm font-medium text-gray-300">
		Network Type
	</label>
	<select
		id="subnet_type"
		bind:value={formData.subnet_type}
		class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-white focus:border-transparent focus:outline-none focus:ring-2"
	>
		{#each subnetTypes.getItems() as subnet_type (subnet_type.id)}
			<option value={subnet_type.id}>{subnet_type.name}</option>
		{/each}
	</select>

	<TextArea
		label="Description"
		id="description"
		{formApi}
		placeholder="Describe the purpose of this subnet..."
		field={description}
	/>
</div>
