<script lang="ts" generics="T">
	import ListSelectItem from '$lib/shared/components/forms/selection/ListSelectItem.svelte';
	import type { EntityDisplayComponent } from '../types';

	export let item: T;
	export let displayComponent: EntityDisplayComponent<T>;
</script>

<ListSelectItem {item} {displayComponent} />
