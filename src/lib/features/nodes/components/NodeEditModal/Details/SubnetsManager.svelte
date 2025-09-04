<script lang="ts">
	import type { Node } from "$lib/features/nodes/types/base";
	import ListManager from "$lib/shared/components/forms/ListManager.svelte";
    import { subnets } from "$lib/features/subnets/store";

    export let formData: Node;

    let subnetSelectOptions: Subnet[] = $subnets.filter(s => !formData.subnets.find(membership => membership.subnet_id == s.id));
    let nodeSubnets: Subnet[] = formData.subnets.map(membership => $subnets.find(s => s.id === membership.subnet_id)).filter(s => s !== undefined);

    function getNodeSubnetDescription(option: Subnet) {
        console.log(option)
        let membership = formData.subnets.find(s => s.subnet_id === option.id)
        return `${membership?.ip_address} • ${membership?.mac_address}`;
    }
</script>

<div>
    <ListManager
        bind:items={nodeSubnets} 
        label="Subnets" 
        options={subnetSelectOptions}
        allowReorder={false}
        allowDirectAdd={true}
        allowItemEdit={() => false}
        getOptionId={(option) => option.id}
        getOptionLabel={(option) => option.name}
        getOptionDescription={(option) => option.cidr}
        
        getItemId={(item) => item.id}
        getItemLabel={(item) => item.name}
        getItemDescription={getNodeSubnetDescription}
        getItemTags={(option) => {
            return [
                {
                    label: option.cidr,
                    color: "yellow"
                }
            ]
        }} 
        />
</div>

