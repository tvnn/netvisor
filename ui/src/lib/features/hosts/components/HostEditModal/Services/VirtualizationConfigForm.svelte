<script lang="ts">
	import { Container, Server } from 'lucide-svelte';
	import type { FormApi } from '$lib/shared/components/forms/types';
	import TextInput from '$lib/shared/components/forms/input/TextInput.svelte';
	import { field } from 'svelte-forms';
	import { maxLength } from '$lib/shared/components/forms/validators';
	import type { ServiceVirtualization } from '$lib/features/services/types/base';

	export let formApi: FormApi;
	export let formData: {
		virtualization: ServiceVirtualization | null;
	};

	// Form fields for Docker
	const containerName = field(
		'container_name',
		formData.virtualization?.type === 'Docker'
			? formData.virtualization.details.container_name || ''
			: '',
		[maxLength(100)]
	);
	const containerId = field(
		'container_id',
		formData.virtualization?.type === 'Docker'
			? formData.virtualization.details.container_id || ''
			: '',
		[maxLength(100)]
	);

	// Form fields for Proxmox
	// const vmName = field(
	// 	'vm_name',
	// 	formData.virtualization?.type === 'Proxmox'
	// 		? formData.virtualization.details.vm_name || ''
	// 		: '',
	// 	[maxLength(100)]
	// );
	// const vmId = field(
	// 	'vm_id',
	// 	formData.virtualization?.type === 'Proxmox' ? formData.virtualization.details.vm_id || '' : '',
	// 	[maxLength(100)]
	// );

	// Virtualization type options
	const virtualizationTypes = [
		{
			value: 'None',
			label: 'None',
			description: 'No virtualization configured',
			disabled: false,
			icon: Server
		},
		{
			value: 'Docker',
			label: 'Docker',
			description: 'Docker container virtualization',
			disabled: false,
			icon: Container
		},
		{
			value: 'Proxmox',
			label: 'Proxmox',
			description: 'Proxmox VM virtualization',
			disabled: false,
			icon: Server
		}
	];

	// Get current virtualization type
	$: currentType = formData.virtualization?.type || 'None';

	// Update formData when Docker fields change
	$: if (formData.virtualization?.type === 'Docker') {
		formData.virtualization.details = {
			container_name: $containerName.value || '',
			container_id: $containerId.value || ''
		};
	}

	// Update formData when Proxmox fields change
	// $: if (formData.virtualization?.type === 'Proxmox') {
	// 	formData.virtualization.details = {
	// 		vm_name: $vmName.value || '',
	// 		vm_id: $vmId.value || ''
	// 	};
	// }

	// Handle virtualization type changes
	function handleVirtualizationTypeChange(event: Event) {
		const targetElement = event.target as HTMLSelectElement;
		const newType = targetElement.value;

		// Reset virtualization config when type changes
		if (newType === 'Docker') {
			formData.virtualization = {
				type: 'Docker',
				details: {
					container_name: '',
					container_id: ''
				}
			};
			containerName.set('');
			containerId.set('');
		}
		// else if (newType === 'Proxmox') {
		// 	formData.virtualization = {
		// 		type: 'Proxmox',
		// 		details: {
		// 			vm_name: '',
		// 			vm_id: ''
		// 		}
		// 	};
		// 	vmName.set('');
		// 	vmId.set('');
		// }
		else {
			formData.virtualization = null;
		}

		// Force reactivity update
		formData = { ...formData };
	}
</script>

<div class="flex items-start gap-6">
	<!-- Virtualization Type Selection -->
	<div class="flex w-1/3 flex-col space-y-2">
		<label for="virtualization_type" class="block text-sm font-medium text-gray-300">
			Virtualization Type
		</label>
		<select
			id="virtualization_type"
			value={currentType}
			on:change={handleVirtualizationTypeChange}
			class="w-full rounded-md border border-gray-600 bg-gray-700 px-3 py-2 text-white
              focus:outline-none focus:ring-2"
		>
			{#each virtualizationTypes as virtType (virtType.value)}
				<option disabled={virtType.disabled} value={virtType.value}>{virtType.label}</option>
			{/each}
		</select>
		<p class="text-xs text-gray-400">Configure virtualization settings for this service</p>
	</div>

	<!-- Virtualization Configuration -->
	<div class="flex flex-grow flex-col">
		<div class="space-y-4">
			{#if formData.virtualization && formData.virtualization.type === 'Docker'}
				<!-- Docker Configuration -->
				<div class="grid grid-cols-2 gap-4">
					<TextInput
						label="Container Name"
						id="container_name"
						{formApi}
						placeholder="my-container"
						field={containerName}
					/>
					<TextInput
						label="Container ID"
						id="container_id"
						{formApi}
						placeholder="abc123..."
						field={containerId}
					/>
				</div>
				<!-- {:else if formData.virtualization && formData.virtualization.type === 'Proxmox'} -->
				<!-- Proxmox Configuration -->
				<!-- <div class="grid grid-cols-2 gap-4">
					<TextInput label="VM Name" id="vm_name" {formApi} placeholder="my-vm" field={vmName} />
					<TextInput label="VM ID" id="vm_id" {formApi} placeholder="100" field={vmId} />
				</div> -->
			{/if}
		</div>
	</div>
</div>
