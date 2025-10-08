<script lang="ts">
	import { field } from 'svelte-forms';
	import type { Layer3Binding, Layer4Binding, Service } from '$lib/features/services/types/base';
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
	import { getServicesForInterface } from '$lib/features/services/store';
	import { Layer3BindingDisplay } from '$lib/shared/components/forms/selection/display/Layer3BindingDisplay.svelte';

	export let formApi: FormApi;
	export let formData: Host;
	export let service: Service<Layer3Binding>;
	export let onChange: (updatedService: Service<Layer3Binding>) => void = () => {};

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

	// Calculate available interfaces
	$: availableBindings = formData.interfaces.filter((iface) => {
        // Check if this interface is already bound by ANY service
        // First check services from the store (other services)
        let otherServices = getServicesForInterface(iface.id).filter((s) => s.id !== service.id);
        let bound_iface_ids = otherServices.flatMap((s) => s.bindings).map((b) => b.interface_id);

        !bound_iface_ids.includes(iface.id)
    })

	$: canCreateNewBinding = availableBindings.length > 0;

	// Update service when field values change
	$: if ($nameField) {
		const updatedService: Service<Layer3Binding> = {
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
		const firstAvailable = availableBindings[0];

		const binding = {
			type: "Layer3",
			id: uuidv4(),
			interface_id: firstAvailable.id
		} as Layer3Binding;

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

	function handleUpdateBinding(binding: Layer3Binding, index: number) {
		if (!service) return;

		const updatedBindings = [...service.bindings];
		updatedBindings[index].interface_id = binding.interface_id;

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

		<!-- Bindings -->
		<div class="space-y-4">
			{#key service.id}
				<ListManager
					label="Bindings (Layer 3)"
					helpText="Configure which interfaces this service listens on"
					placeholder="Select a binding to add"
					createNewLabel="New Binding"
					allowDuplicates={false}
					allowItemEdit={() => true}
					allowItemRemove={() => true}
					allowReorder={false}
					allowCreateNew={true}
					allowAddFromOptions={false}
					disableCreateNewButton={!canCreateNewBinding}
					options={[] as Layer3Binding[]}
					optionDisplayComponent={Layer3BindingDisplay}
					itemDisplayComponent={Layer3BindingDisplay}
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
