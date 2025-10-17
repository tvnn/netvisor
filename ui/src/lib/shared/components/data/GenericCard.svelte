<script lang="ts">
	import type { CardAction, CardSection, CardList } from './types';
	import Tag from './Tag.svelte';
	import type { Component } from 'svelte';
	import { type IconComponent } from '$lib/shared/utils/types';

	export let title: string;
	export let link: string = '';
	export let subtitle: string = '';
	export let status: string = '';
	export let statusColor: string = 'text-gray-400';
	export let icon: IconComponent | null = null; // Expects Svelte component, not string
	export let iconColor: string = 'text-blue-400';
	export let actions: CardAction[] = [];
	export let sections: CardSection[] = [];
	export let lists: CardList[] = [];
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	export let footerComponent: Component<any> | null = null; // Optional footer component
	export let footerProps: Record<string, unknown> = {}; // Props to pass to footer component
</script>

<div
	class="flex h-full flex-col rounded-lg border border-gray-700 bg-gray-800 p-6 transition-colors hover:border-gray-600"
>
	<!-- Header -->
	<div class="mb-4 flex items-start justify-between">
		<div class="flex items-center space-x-3">
			{#if icon}
				<svelte:component this={icon} size={24} class={iconColor} />
			{/if}
			<div>
				{#if link}
					<a
						href={link}
						class="text-lg font-semibold text-white hover:text-blue-400"
						target="_blank">{title}</a
					>
				{:else}
					<h3 class="text-lg font-semibold text-white">{title}</h3>
				{/if}
				{#if subtitle}
					<p class="text-sm text-gray-400">{subtitle}</p>
				{/if}
			</div>
		</div>
		{#if status}
			<span class="text-sm font-medium {statusColor}">{status}</span>
		{/if}
	</div>

	<!-- Content - grows to fill available space -->
	<div class="flex-grow space-y-3">
		<!-- Basic info sections -->
		{#each sections as section (section.value)}
			<div class="text-sm text-gray-300">
				<span class="text-gray-400">{section.label}:</span>
				<span class="ml-2">{section.value}</span>
			</div>
		{/each}

		<!-- List sections -->
		{#each lists as list (list.label)}
			{#if list.label || list.items}
				<div class="text-sm">
					<div class="flex flex-wrap items-center gap-2">
						{#if list.label}
							<span class="text-gray-400">{list.label}:</span>
						{/if}
						{#if list.items.length > 0}
							{#each list.items as item (item.id)}
								<div class="flex items-center justify-between">
									<div class="flex items-center space-x-2">
										<Tag
											icon={item.icon}
											disabled={item.disabled}
											color={item.color}
											badge={item.badge}
											label={item.label}
										/>
									</div>

									<!-- Item actions -->
									{#if list.itemActions}
										<div class="flex items-center space-x-1">
											{#each list.itemActions(item) as action (action.label)}
												<button
													on:click={action.onClick}
													disabled={action.disabled}
													class="p-1 {action.color || 'text-gray-400'} hover:{action.hoverColor ||
														'text-white'} {action.bgHover ||
														'hover:bg-gray-700'} rounded transition-colors disabled:opacity-50"
													title={action.label}
												>
													<svelte:component
														this={action.icon}
														size={16}
														class={action.animation || ''}
													/>
												</button>
											{/each}
										</div>
									{/if}
								</div>
							{/each}
						{:else if list.label}
							<span class="text-gray-500">{list.emptyText || `No ${list.label.toLowerCase()}`}</span
							>
						{/if}
					</div>
				</div>
			{/if}
		{/each}
	</div>

	<!-- Footer Component -->
	{#if footerComponent}
		<div class="mt-4 border-t border-gray-700 pt-4">
			<svelte:component this={footerComponent} {...footerProps} />
		</div>
	{/if}

	<!-- Action Buttons -->
	{#if actions.length > 0}
		<div class="mt-4 flex items-center justify-between border-t border-gray-700 pt-4">
			{#each actions as action (action.label)}
				<button
					on:click={action.onClick}
					disabled={action.disabled}
					class="p-2 {action.color || 'text-gray-400'} hover:{action.hoverColor ||
						'text-white'} {action.bgHover ||
						'hover:bg-gray-700'} rounded transition-colors disabled:opacity-50"
					title={action.label}
				>
					<svelte:component this={action.icon} size={16} class={action.animation || ''} />
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	button:disabled:hover {
		background-color: transparent;
		color: inherit;
	}
</style>
