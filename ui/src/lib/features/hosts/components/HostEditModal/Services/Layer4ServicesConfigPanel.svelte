<script lang="ts">
	import { field } from 'svelte-forms';
	import type { Layer4Binding, Service } from '$lib/features/services/types/base';
	import type { Host } from '$lib/features/hosts/types/base';
	import { serviceDefinitions } from '$lib/shared/stores/metadata';
	import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import type { FormApi } from '$lib/shared/components/forms/types';
	import { required } from 'svelte-forms/validators';
	import { pushWarning } from '$lib/shared/stores/feedback';
	import TextInput from '$lib/shared/components/forms/input/TextInput.svelte';
	import { maxLength } from '$lib/shared/components/forms/validators';
	import ConfigHeader from '$lib/shared/components/forms/config/ConfigHeader.svelte';
	import { v4 as uuidv4 } from 'uuid';
	import { getServicesForPort } from '$lib/features/services/store';
	import { Layer4BindingDisplay } from '$lib/shared/components/forms/selection/display/Layer4BindingDisplay.svelte';

	export let formApi: FormApi;
	export let formData: Host;
	export let service: Service<Layer4Binding>;
	export let onChange: (updatedService: Service<Layer4Binding>) => void = () => {};

	let currentServiceId: string = service.id;

	const getNameField = () => {
		return field(`service_name_${currentServiceId}`, service.name, [required(), maxLength(100)]);
	};

	let nameField = getNameField();

	$: serviceMetadata = service ? serviceDefinitions.getItem(service.service_definition) : null;

	$: if (service.id !== currentServiceId) {
		currentServiceId = service.id;
		nameField = getNameField();
	}

	// Calculate available port+interface combinations
	$: availableCombinations = formData.interfaces.flatMap((iface) => {
		return formData.ports
			.filter((port) => {
				// Check if this port is already bound to this interface by ANY service
				// First check services from the store (other services)
				let otherServices = getServicesForPort(port.id).filter((s) => s.id !== service.id);
				let bound_iface_ids = otherServices.flatMap((s) => s.bindings).map((b) => b.interface_id);

				// Also check current service's bindings (from formData)
				let currentServiceBindings = service.bindings
					.filter((b) => b.port_id === port.id)
					.map((b) => b.interface_id);

				// Combine both checks
				let allBoundIfaceIds = [...bound_iface_ids, ...currentServiceBindings];

				return !allBoundIfaceIds.includes(iface.id);
			})
			.map((port) => ({ port, iface }));
	});

	$: canCreateNewBinding = availableCombinations.length > 0;

	// Update service when field values change
	$: if ($nameField) {
		const updatedService: Service<Layer4Binding> = {
			...service,
			name: $nameField.value
		};

		// Only trigger onChange if values actually changed
		if (updatedService.name !== service.name) {
			onChange(updatedService);
		}
	}

	function handleCreateNewBinding() {
		if (!service) {
			pushWarning('Could not find service to create binding for');
			return;
		}

		if (formData.interfaces.length == 0) {
			pushWarning("Host does not have any interfaces, can't create binding");
			return;
		}

		if (formData.ports.length == 0) {
			pushWarning("Host does not have any ports, can't create binding");
			return;
		}

		if (!canCreateNewBinding) {
			pushWarning('No available port+interface combinations to bind');
			return;
		}

		// Use the first available combination
		const firstAvailable = availableCombinations[0];

		const binding = {
			type: 'Layer4',
			id: uuidv4(),
			port_id: firstAvailable.port.id,
			interface_id: null
		} as Layer4Binding;

		const updatedService = {
			...service,
			bindings: [...service.bindings, binding]
		};

		onChange(updatedService);
	}

	function handleRemoveBinding(index: number) {
		if (!service) {
			pushWarning('Could not find service to remove binding for');
			return;
		}

		const updatedService = {
			...service,
			bindings: service.bindings.filter((_, i) => i !== index)
		};

		onChange(updatedService);
	}

	function handleUpdateBinding(binding: Layer4Binding, index: number) {
		if (!service) return;

		const updatedBindings = [...service.bindings];
		updatedBindings[index].interface_id = binding.interface_id;
		updatedBindings[index].port_id = binding.port_id;

		const updatedService = {
			...service,
			bindings: updatedBindings
		};

		onChange(updatedService);
	}
</script>

{#if service && serviceMetadata}
	<div class="space-y-6">
		<ConfigHeader title={serviceMetadata.name} subtitle={serviceMetadata.description} />

		<!-- Basic Configuration -->
		<div class="space-y-4">
			<!-- Service Name Field -->
			{#if $nameField}
				<TextInput
					label="Name"
					id="service_name_{service.id}"
					{formApi}
					required={true}
					placeholder="Enter a descriptive name..."
					field={nameField}
				/>
			{/if}
		</div>

		<!-- <VirtualizationConfigForm formData={service} {formApi} /> -->

		<!-- Bindings -->
		<div class="space-y-4">
			{#key service.id}
				<ListManager
					label="Bindings (Layer 4)"
					helpText="Configure which ports and interfaces this service listens on"
					placeholder="Select a binding to add"
					createNewLabel="New Binding"
					allowDuplicates={false}
					allowItemEdit={() => true}
					allowItemRemove={() => true}
					allowReorder={false}
					allowCreateNew={true}
					allowAddFromOptions={false}
					disableCreateNewButton={!canCreateNewBinding}
					options={[] as Layer4Binding[]}
					optionDisplayComponent={Layer4BindingDisplay}
					itemDisplayComponent={Layer4BindingDisplay}
					items={service.bindings}
					getItemContext={() => ({ service, host: formData })}
					onCreateNew={handleCreateNewBinding}
					onRemove={handleRemoveBinding}
					onEdit={handleUpdateBinding}
				/>
			{/key}
		</div>
	</div>
{/if}
