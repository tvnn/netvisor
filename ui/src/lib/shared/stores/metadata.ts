import { writable, derived, get } from 'svelte/store';
import { api } from '../utils/api';
import {
	createColorHelper,
	createHomarrIconComponent,
	createIconComponent,
	createStyle,
	type ColorStyle
} from '../utils/styling';

export interface TypeMetadata {
	id: string;
	name: string;
	description: string;
	category: string;
	icon: string;
	color: string;
	metadata: Record<string, any>;
}

export interface EntityMetadata {
	id: string;
	color: string;
	icon: string;
}

export interface MetadataRegistry {
	service_definitions: TypeMetadata[];
	subnet_types: TypeMetadata[];
	edge_types: TypeMetadata[];
	entities: EntityMetadata[];
	ports: TypeMetadata[];
}

export const metadata = writable<MetadataRegistry>();

// Shared color helper functions that work for both TypeMetadata and EntityMetadata
function createSharedHelpers<T extends keyof MetadataRegistry>(category: T) {
	return {
		getColorString: (id: string | null): string => {
			const $registry = get(metadata);
			const item = $registry?.[category]?.find((item) => item.id === id);
			return item?.color || 'gray';
		},

		getColorHelper: (id: string | null): ColorStyle => {
			const $registry = get(metadata);
			const item = $registry?.[category]?.find((item) => item.id === id);
			const baseColor = item?.color || null;
			return createColorHelper(baseColor);
		},

		getIcon: (id: string | null) => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as EntityMetadata[])?.find((item) => item.id === id)?.icon ||
				'HelpCircle'
			);
		},

		getIconComponent: (id: string | null) => {
			const $registry = get(metadata);
			const item = ($registry?.[category] as EntityMetadata[])?.find((item) => item.id === id);
			const iconName = item?.icon || null;
			return createIconComponent(iconName);
		},

		getStyle: (id: string | null) => {
			const $registry = get(metadata);
			const item = ($registry?.[category] as EntityMetadata[])?.find((item) => item.id === id);
			const color = item?.color || null;
			const icon = item?.icon || null;
			return createStyle(color, icon);
		}
	};
}

// Type helpers to constrain generic types
type TypeMetadataKeys = {
	[K in keyof MetadataRegistry]: MetadataRegistry[K][number] extends TypeMetadata ? K : never;
}[keyof MetadataRegistry];

type EntityMetadataKeys = {
	[K in keyof MetadataRegistry]: MetadataRegistry[K][number] extends EntityMetadata ? K : never;
}[keyof MetadataRegistry];

// Full TypeMetadata helpers (includes color methods + other methods)
function createTypeMetadataHelpers<T extends TypeMetadataKeys>(category: T) {
	const items = derived(metadata, ($registry) => $registry?.[category] || []);
	const sharedHelpers = createSharedHelpers(category);

	const helpers = {
		// Include the shared methods
		...sharedHelpers,

		getIconComponent: (id: string | null) => {
			const $registry = get(metadata);
			const item = ($registry?.[category] as TypeMetadata[])?.find((item) => item.id === id);
			const iconName = item?.icon || null;

			if (item?.metadata && item.metadata.has_homarr_icon) {
				return createHomarrIconComponent(iconName);
			} else {
				return createIconComponent(iconName);
			}
		},

		getItems: () => {
			const $registry = get(metadata);
			return $registry?.[category] as TypeMetadata[];
		},

		getItem: (id: string | null) => {
			const $registry = get(metadata);
			return ($registry?.[category] as TypeMetadata[])?.find((item) => item.id === id) || null;
		},

		getName: (id: string | null) => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as TypeMetadata[])?.find((item) => item.id === id)?.name || id || ''
			);
		},

		getDescription: (id: string | null) => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as TypeMetadata[])?.find((item) => item.id === id)?.description || ''
			);
		},

		getCategory: (id: string | null) => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as TypeMetadata[])?.find((item) => item.id === id)?.category || ''
			);
		},

		getMetadata: (id: string | null) => {
			const $registry = get(metadata);
			return (
				($registry?.[category] as TypeMetadata[])?.find((item) => item.id === id)?.metadata || {}
			);
		}
	};

	return helpers;
}

// EntityMetadata helpers (only color methods)
function createEntityMetadataHelpers<T extends EntityMetadataKeys>(category: T) {
	const items = derived(metadata, ($registry) => $registry?.[category] || []);
	const sharedHelpers = createSharedHelpers(category);

	const helpers = {
		getItems: () => {
			const $registry = get(metadata);
			return $registry?.[category] as EntityMetadata[];
		},

		getItem: (id: string | null) => {
			const $registry = get(metadata);
			return ($registry?.[category] as EntityMetadata[])?.find((item) => item.id === id) || null;
		},

		// Only include the shared color methods
		...sharedHelpers
	};

	return helpers;
}

// Create all the helpers
export const serviceDefinitions = createTypeMetadataHelpers('service_definitions');
export const subnetTypes = createTypeMetadataHelpers('subnet_types');
export const edgeTypes = createTypeMetadataHelpers('edge_types');
export const entities = createEntityMetadataHelpers('entities');
export const ports = createTypeMetadataHelpers('ports');

export async function getMetadata() {
	const result = await api.request<MetadataRegistry>(
		'/metadata',
		metadata,
		(metadata) => metadata,
		{ method: 'GET' }
	);
}
