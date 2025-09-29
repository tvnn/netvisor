<script lang="ts">
	import { field } from 'svelte-forms';
	import type { Interface } from '$lib/features/hosts/types/base';
	import { ipAddress, ipAddressInCidr, maxLength } from '$lib/shared/components/forms/validators';
	import { required } from 'svelte-forms/validators';
	import type { FormApi } from '$lib/shared/components/forms/types';
	import TextInput from '$lib/shared/components/forms/input/TextInput.svelte';
	import ConfigHeader from '$lib/shared/components/forms/config/ConfigHeader.svelte';

	export let formApi: FormApi;
	export let iface: Interface;
	export let subnet: Subnet;
	export let onChange: (updatedIface: Interface) => void = () => {};

	const getIpField = () => {
		return field(
			`interface_ip_${currentInterfaceId}`,
			iface.ip_address || '',
			[required(), ipAddress(), ipAddressInCidr(subnet.cidr)],
			{
				checkOnInit: false
			}
		);
	};

	const getNameField = () => {
		return field(
			`interface_name_${currentInterfaceId}`,
			iface.name || '',
			[required(), maxLength(100)],
			{
				checkOnInit: false
			}
		);
	};

	let currentInterfaceId: string = iface.id;
	let ipAddressField = getIpField();
	let nameField = getNameField();

	// Only sync field values when interface ID changes (new interface selected)
	$: if (iface.id !== currentInterfaceId) {
		currentInterfaceId = iface.id;
		ipAddressField = getIpField();
		nameField = getNameField();
	}

	$: if ($ipAddressField && $nameField) {
		const updatedIface: Interface = {
			...iface,
			ip_address: $ipAddressField.value,
			name: $nameField.value
		};

		// Only trigger onChange if values actually changed
		if (
			updatedIface.ip_address !== iface.ip_address ||
			updatedIface.mac_address !== iface.mac_address ||
			updatedIface.name !== iface.name
		) {
			onChange(updatedIface);
		}
	}
</script>

{#if subnet}
	<div class="space-y-6">
		<ConfigHeader
			title={'Subnet ' + (subnet?.name ? subnet.name : subnet.cidr)}
			subtitle={subnet?.description}
		/>

		<div class="space-y-4">
			<h4 class="text-sm font-medium text-gray-300">Interface Configuration</h4>

			{#if $nameField}
				<TextInput
					label="Name"
					id="interface_{iface.id}"
					{formApi}
					placeholder="en0"
					field={nameField}
				/>
			{/if}

			{#if $ipAddressField}
				<TextInput
					label="IP Address"
					id="interface_ip_{iface.id}"
					{formApi}
					required={true}
					placeholder="192.168.1.100"
					field={ipAddressField}
					helpText="Must be within {subnet.cidr}"
				/>
			{/if}
		</div>
	</div>
{/if}
