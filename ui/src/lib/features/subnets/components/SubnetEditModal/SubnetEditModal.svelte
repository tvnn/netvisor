<script lang="ts">
	import { createEmptySubnetFormData } from '../../store';
	import EditModal from '$lib/shared/components/forms/EditModal.svelte';
	import ListManager from '$lib/shared/components/forms/selection/ListManager.svelte';
	import { entities, serviceDefinitions } from '$lib/shared/stores/metadata';
	import {
		getServiceBindingsFromService,
		serviceHasInterfaceOnSubnet,
		services
	} from '$lib/features/services/store';
	import ModalHeaderIcon from '$lib/shared/components/layout/ModalHeaderIcon.svelte';
	import SubnetDetailsForm from './SubnetDetailsForm.svelte';
	import EntityMetadataSection from '$lib/shared/components/forms/EntityMetadataSection.svelte';
	import type { Subnet } from '../../types/base';
	import { serviceBindingIdToObj, serviceBindingToId } from '$lib/features/hosts/store';
	import { ServiceBindingDisplay } from '$lib/shared/components/forms/selection/display/ServiceBindingDisplay.svelte';
	import { ServiceWithHostDisplay } from '$lib/shared/components/forms/selection/display/ServiceWithHostDisplay.svelte';

	export let subnet: Subnet | null = null;
	export let isOpen = false;
	export let onCreate: (data: Subnet) => Promise<void> | void;
	export let onUpdate: (id: string, data: Subnet) => Promise<void> | void;
	export let onClose: () => void;
	export let onDelete: ((id: string) => Promise<void> | void) | null = null;

	let loading = false;
	let deleting = false;

	$: isEditing = subnet !== null;
	$: title = isEditing ? `Edit ${subnet?.name}` : 'Create Subnet';

	let formData: Subnet = createEmptySubnetFormData();

	// Initialize form data when subnet changes or modal opens
	$: if (isOpen) {
		resetForm();
	}

	function resetForm() {
		formData = subnet ? { ...subnet } : createEmptySubnetFormData();
	}

	$: dnsServiceBindings = $services
		.filter((service) => {
			const isDnsResolver = serviceDefinitions.getMetadata(
				service.service_definition
			)?.is_dns_resolver;
			const hasInterfaceOnSubnet = serviceHasInterfaceOnSubnet(service, formData.id);
			return isDnsResolver && hasInterfaceOnSubnet;
		})
		.flatMap((service) => getServiceBindingsFromService(service))

	$: gatewayServiceBindings = $services
		.filter((service) => {
			const isGateway = serviceDefinitions.getMetadata(
				service.service_definition
			)?.is_gateway;
			const hasInterfaceOnSubnet = serviceHasInterfaceOnSubnet(service, formData.id);
			return isGateway && hasInterfaceOnSubnet;
		})
		.flatMap((service) => getServiceBindingsFromService(service))

	$: reverseProxyServiceBindings = $services
		.filter((service) => {
			const isReverseProxy = serviceDefinitions.getMetadata(
				service.service_definition
			)?.is_reverse_proxy;
			const hasInterfaceOnSubnet = serviceHasInterfaceOnSubnet(service, formData.id);
			return isReverseProxy && hasInterfaceOnSubnet;
		})
		.flatMap((service) => getServiceBindingsFromService(service));

	// Available services (filtered out already selected)
	$: availableDns = dnsServiceBindings.filter(
		(sb) =>
			!formData.dns_resolvers.some(
				(resolver) => serviceBindingToId(resolver) == serviceBindingToId(sb)
			)
	);
	$: availableReverseProxies = reverseProxyServiceBindings.filter(
		(sb) =>
			!formData.reverse_proxies.some((proxy) => serviceBindingToId(proxy) == serviceBindingToId(sb))
	);
	$: availableGateways = gatewayServiceBindings.filter(
		(sb) =>
			!formData.gateways.some((gateway) => serviceBindingToId(gateway) == serviceBindingToId(sb))
	);

	async function handleSubmit() {
		// Clean up the data before sending
		const subnetData: Subnet = {
			...formData,
			name: formData.name.trim(),
			description: formData.description?.trim() || '',
			cidr: formData.cidr.trim()
		};

		loading = true;
		try {
			if (isEditing && subnet) {
				await onUpdate(subnet.id, subnetData);
			} else {
				await onCreate(subnetData);
			}
		} finally {
			loading = false;
		}
	}

	async function handleDelete() {
		if (onDelete && subnet) {
			deleting = true;
			try {
				await onDelete(subnet.id);
			} finally {
				deleting = false;
			}
		}
	}

	// Event handlers for DNS resolvers
	function handleAddDnsResolver(serviceBindingId: string) {
		let serviceBindingObj = serviceBindingIdToObj(serviceBindingId);

		if (serviceBindingObj) {
			formData.dns_resolvers = [...(formData.dns_resolvers || []), serviceBindingObj];
		}
	}

	function handleRemoveDnsResolver(index: number) {
		formData.dns_resolvers = formData.dns_resolvers?.filter((_, i) => i !== index) || [];
	}

	// Event handlers for gateways
	function handleAddGateway(serviceBindingId: string) {
		let serviceBindingObj = serviceBindingIdToObj(serviceBindingId);

		if (serviceBindingObj) {
			formData.gateways = [...(formData.gateways || []), serviceBindingObj];
		}
	}

	function handleRemoveGateway(index: number) {
		formData.gateways = formData.gateways?.filter((_, i) => i !== index) || [];
	}

	// Event handlers for reverse proxies
	function handleAddReverseProxy(serviceBindingId: string) {
		let serviceBindingObj = serviceBindingIdToObj(serviceBindingId);

		if (serviceBindingObj) {
			formData.reverse_proxies = [...(formData.reverse_proxies || []), serviceBindingObj];
		}
	}

	function handleRemoveReverseProxy(index: number) {
		formData.reverse_proxies = formData.reverse_proxies?.filter((_, i) => i !== index) || [];
	}

	// Dynamic labels based on create/edit mode
	$: saveLabel = isEditing ? 'Update Subnet' : 'Create Subnet';

	let colorHelper = entities.getColorHelper('Subnet');
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
		<ModalHeaderIcon Icon={entities.getIconComponent('Subnet')} color={colorHelper.string} />
	</svelte:fragment>

	<!-- Content -->
	<div class="flex h-full flex-col overflow-hidden">
		<div class="flex-1 overflow-y-auto">
			<div class="space-y-8 p-6">
				<SubnetDetailsForm {formApi} bind:formData />

				<!-- DNS Resolvers Section -->
				<div class="space-y-4">
					<div class="border-t border-gray-700 pt-6">
						<h3 class="mb-4 text-lg font-medium text-white">DNS Resolvers</h3>
						<div class="rounded-lg bg-gray-800/50 p-4">
							<ListManager
								label="DNS Resolvers"
								helpText="Select hosts that provide DNS resolution services for this subnet"
								placeholder="Select a DNS server to add..."
								emptyMessage="No DNS resolvers configured. DNS capable hosts will appear here."
								allowReorder={false}
								showSearch={true}
								options={availableDns}
								items={formData.dns_resolvers}
								allowItemEdit={() => false}
								optionDisplayComponent={ServiceBindingDisplay}
								itemDisplayComponent={ServiceBindingDisplay}
								onAdd={handleAddDnsResolver}
								onRemove={handleRemoveDnsResolver}
								onEdit={() => {}}
							/>
						</div>
					</div>
				</div>

				<!-- Gateways Section -->
				<div class="space-y-4">
					<div class="border-t border-gray-700 pt-6">
						<h3 class="mb-4 text-lg font-medium text-white">Gateways</h3>
						<div class="rounded-lg bg-gray-800/50 p-4">
							<ListManager
								label="Gateways"
								helpText="Select hosts that provide gateway/routing services for this subnet"
								placeholder="Select a gateway to add..."
								emptyMessage="No gateways configured. Gateway-capable hosts will appear here."
								allowReorder={false}
								showSearch={true}
								options={availableGateways}
								items={formData.gateways}
								allowItemEdit={() => false}
								optionDisplayComponent={ServiceBindingDisplay}
								itemDisplayComponent={ServiceBindingDisplay}
								onAdd={handleAddGateway}
								onRemove={handleRemoveGateway}
								onEdit={() => {}}
							/>
						</div>
					</div>
				</div>

				<!-- Reverse Proxies Section -->
				<div class="space-y-4">
					<div class="border-t border-gray-700 pt-6">
						<h3 class="mb-4 text-lg font-medium text-white">Reverse Proxies</h3>
						<div class="rounded-lg bg-gray-800/50 p-4">
							<ListManager
								label="Reverse Proxies"
								helpText="Select hosts that provide reverse proxy services for this subnet"
								placeholder="Select a reverse proxy to add..."
								emptyMessage="No reverse proxies configured. Reverse proxy-capable hosts will appear here."
								allowReorder={false}
								showSearch={true}
								options={availableReverseProxies}
								items={formData.reverse_proxies}
								allowItemEdit={() => false}
								optionDisplayComponent={ServiceBindingDisplay}
								itemDisplayComponent={ServiceBindingDisplay}
								onAdd={handleAddReverseProxy}
								onRemove={handleRemoveReverseProxy}
								onEdit={() => {}}
							/>
						</div>
					</div>
				</div>

				{#if isEditing}
					<EntityMetadataSection entities={[subnet]} />
				{/if}
			</div>
		</div>
	</div>
</EditModal>
