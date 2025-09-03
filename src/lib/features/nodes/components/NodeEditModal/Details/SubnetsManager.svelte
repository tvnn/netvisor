<script lang="ts">
	import type { Node } from "$lib/features/nodes/types/base";
	import ListManager from "$lib/shared/components/forms/ListManager.svelte";
    import { subnets } from "$lib/features/subnets/store";
	import SubnetListEditor from "$lib/shared/components/forms/SubnetListEditor.svelte";

    export let formData: Node;

    let subnetSelectOptions: Subnet[] = $subnets;
    let nodeSubnets: Subnet[] = formData.subnets.map(membership => $subnets.find(s => s.id === membership.subnet_id)).filter(s => s !== undefined);


</script>

<div>
    <ListManager<Subnet>
        bind:items={formData.subnets} 
        label="Subnets" 
        getOptionId={(option) => } />
</div>

