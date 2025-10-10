<script lang="ts">
	import { createEmptyGroupFormData } from '../../store';
	import EditModal from '$lib/shared/components/forms/EditModal.svelte';
	import type { Group } from '../../types/base';
	import type { ServiceBinding } from '$lib/features/hosts/types/base';
	import ModalHeaderIcon from '$lib/shared/components/layout/ModalHeaderIcon.svelte';
	import { entities } from '$lib/shared/stores/metadata';
	import { getServiceBindingsFromService, services } from '$lib/features/services/store';
	import { ServiceBindingDisplay } from '$lib/shared/components/forms/selection/display/ServiceBindingDisplay.svelte';
	import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import GroupDetailsForm from './GroupDetailsForm.svelte';
	import EntityMetadataSection from '$lib/shared/components/forms/EntityMetadataSection.svelte';
	import { serviceBindingIdToObj, serviceBindingToId } from '$lib/features/hosts/store';

	export let group: Group | null = null;
	export let isOpen = false;
	export let onCreate: (data: Group) => Promise<void> | void;
	export let onUpdate: (id: string, data: Group) => Promise<void> | void;
	export let onClose: () => void;
	export let onDelete: ((id: string) => Promise<void> | void) | null = null;

	let loading = false;
	let deleting = false;

	$: isEditing = group !== null;
	$: title = isEditing ? `Edit ${group?.name}` : 'Create Group';

	let formData: Group = createEmptyGroupFormData();

	// Initialize form data when group changes or modal opens
	$: if (isOpen) {
		resetForm();
	}

	function resetForm() {
		formData = group ? { ...group } : createEmptyGroupFormData();
	}

	$: serviceBindings = $services
		.flatMap((s) => getServiceBindingsFromService(s))
		.filter(
			(sb) =>
				!formData.service_bindings.some(
					(binding) => serviceBindingToId(binding) == serviceBindingToId(sb)
				)
		);

	function handleAdd(serviceBindingId: string) {
		let newBinding = serviceBindingIdToObj(serviceBindingId);
		if (newBinding) {
			formData.service_bindings = [...formData.service_bindings, newBinding];
		}
	}

	function handleRemove(index: number) {
		formData.service_bindings = formData.service_bindings.filter((_, i) => i !== index);
	}

	function handleMoveUp(fromIndex: number, toIndex: number) {
		const newBindings = [...formData.service_bindings];
		const [movedBinding] = newBindings.splice(fromIndex, 1);
		newBindings.splice(toIndex, 0, movedBinding);
		formData.service_bindings = newBindings;
	}

	function handleMoveDown(fromIndex: number, toIndex: number) {
		const newBindings = [...formData.service_bindings];
		const [movedBinding] = newBindings.splice(fromIndex, 1);
		newBindings.splice(toIndex, 0, movedBinding);
		formData.service_bindings = newBindings;
	}

	function handleEdit(item: ServiceBinding, index: number) {
		const updatedBindings = [...formData.service_bindings];
		updatedBindings[index] = item;
		formData.service_bindings = updatedBindings;
	}

	async function handleSubmit() {
		// Clean up the data before sending
		const groupData: Group = {
			...formData,
			name: formData.name.trim(),
			description: formData.description?.trim() || ''
		};

		loading = true;
		try {
			if (isEditing && group) {
				await onUpdate(group.id, groupData);
			} else {
				await onCreate(groupData);
			}
		} finally {
			loading = false;
		}
	}

	async function handleDelete() {
		if (onDelete && group) {
			deleting = true;
			try {
				await onDelete(group.id);
			} finally {
				deleting = false;
			}
		}
	}

	// Dynamic labels based on create/edit mode
	$: saveLabel = isEditing ? 'Update Group' : 'Create Group';

	let colorHelper = entities.getColorHelper('Group');
</script>

<EditModal
	{isOpen}
	{title}
	{loading}
	{deleting}
	{saveLabel}
	cancelLabel="Cancel"
	onSave={handleSubmit}
	onCancel={onClose}
	onDelete={isEditing ? handleDelete : null}
	size="xl"
	let:formApi
>
	<!-- Header icon -->
	<svelte:fragment slot="header-icon">
		<ModalHeaderIcon Icon={entities.getIconComponent('Group')} color={colorHelper.string} />
	</svelte:fragment>

	<!-- Content -->
	<div class="flex h-full flex-col overflow-hidden">
		<div class="flex-1 overflow-y-auto">
			<div class="space-y-8 p-6">
				<GroupDetailsForm {formApi} bind:formData />

				<!-- Services Section -->
				<div class="space-y-4">
					<div class="border-t border-gray-700 pt-6">
						<h3 class="mb-2 text-lg font-medium text-white">Services</h3>
						<p class="mb-4 text-sm text-gray-400">
							Define the services in this group. Only services with at least one port and one
							interface binding ca be selected.
						</p>
						<div class="rounded-lg bg-gray-800/50 p-4">
							<ListManager
								label="Services"
								helpText="Select services and configure their interface bindings for this group"
								placeholder="Select a service to add..."
								emptyMessage="No services in this group yet."
								allowReorder={true}
								showSearch={true}
								options={serviceBindings}
								items={formData.service_bindings}
								allowItemEdit={() => true}
								optionDisplayComponent={ServiceBindingDisplay}
								itemDisplayComponent={ServiceBindingDisplay}
								onAdd={handleAdd}
								onRemove={handleRemove}
								onMoveUp={handleMoveUp}
								onMoveDown={handleMoveDown}
								onEdit={handleEdit}
							/>
						</div>
					</div>
				</div>

				{#if isEditing}
					<EntityMetadataSection entities={[group]} />
				{/if}
			</div>
		</div>
	</div>
</EditModal>
