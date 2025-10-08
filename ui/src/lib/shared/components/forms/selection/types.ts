import type { Component } from 'svelte';
import type { TagProps } from '../../data/types';
import type { IconComponent } from '$lib/shared/utils/types';

export interface EntityDisplayComponent<T> {
	// Required methods
	getId(item: T): string;
	getLabel(item: T): string;

	// Optional methods with defaults
	getDescription?(item: T): string;
	getIcon?(item: T): IconComponent | null;
	getIconColor?(item: T): string | null;
	getTags?(item: T): TagProps[];
	getIsDisabled?(item: T): boolean;
	getCategory?(item: T): string | null;

	// Optional inline editing support
	supportsInlineEdit?: boolean;
	renderInlineEdit?(
		item: T,
		onUpdate: (updates: Partial<T>) => void,
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		context: Record<string, any> | null
	): {
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		component: Component<any>;
		props: Record<string, unknown>;
	};
}

export interface DisplayComponentProps<T> {
	item: T;
}
