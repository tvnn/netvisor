<script lang="ts" generics="T">
	import Tag from '../../data/Tag.svelte';
	import type { EntityDisplayComponent } from './types';

	export let item: T;
	export let displayComponent: EntityDisplayComponent<T>;

	$: icon = displayComponent.getIcon?.(item);
	$: tags = displayComponent.getTags?.(item) || [];
	$: description = displayComponent.getDescription?.(item) || '';
</script>

<div class="flex min-w-0 flex-1 items-center gap-3">
	<!-- Icon -->
	{#if icon}
		<div class="flex h-7 w-7 flex-shrink-0 items-center justify-center">
			<svelte:component
				this={icon}
				class="h-5 w-5 {displayComponent.getIconColor?.(item) || 'text-gray-300'}"
			/>
		</div>
	{/if}

	<!-- Label and description -->
	<div class="min-w-0 flex-1 text-left">
		<div class="flex items-center gap-3">
			<span class="block truncate">{displayComponent.getLabel(item)}</span>
			<!-- Tags -->
			{#if tags.length > 0}
				<div class="flex gap-1">
					{#each tags as tag (tag.label)}
						<Tag
							label={tag.label}
							color={tag.color}
							textColor={tag.textColor}
							bgColor={tag.bgColor}
						/>
					{/each}
				</div>
			{/if}
		</div>
		{#if description.length > 0}
			<span class="mt-2 block truncate text-xs text-gray-400">{description}</span>
		{/if}
	</div>
</div>
