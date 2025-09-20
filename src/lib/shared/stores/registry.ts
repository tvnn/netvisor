import { writable, derived, get } from 'svelte/store';
import { api } from '../utils/api';
import { createColorHelper, createIconComponent, createStyle, type ColorStyle } from '../utils/styling';

export interface TypeMetadata {
  id: string;
  display_name: string;
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

export interface TypeRegistry {
  service_types: TypeMetadata[];
  subnet_types: TypeMetadata[];
  edge_types: TypeMetadata[];
  entities: EntityMetadata[];
}

export const registry = writable<TypeRegistry>();

// Shared color helper functions that work for both TypeMetadata and EntityMetadata
function createSharedHelpers<T extends keyof TypeRegistry>(category: T) {
  return {
    getColorString: (id: string | null): string => {
      const $registry = get(registry);
      const item = $registry?.[category]?.find(item => item.id === id);
      return item?.color || "gray";
    },

    getColorHelper: (id: string | null): ColorStyle => {
      const $registry = get(registry);
      const item = $registry?.[category]?.find(item => item.id === id);
      const baseColor = item?.color || null;
      return createColorHelper(baseColor);
    },

    getIcon: (id: string | null) => {
      const $registry = get(registry);
      return ($registry?.[category] as EntityMetadata[])?.find(item => item.id === id)?.icon || 'HelpCircle';
    },

    getIconComponent: (id: string | null) => {
      const $registry = get(registry);
      const item = ($registry?.[category] as EntityMetadata[])?.find(item => item.id === id);
      const iconName = item?.icon || null;
      return createIconComponent(iconName);
    },

    getStyle: (id: string | null) => {
      const $registry = get(registry);
      const item = ($registry?.[category] as EntityMetadata[])?.find(item => item.id === id);
      const color = item?.color || null;
      const icon = item?.icon || null;
      return createStyle(color, icon);
    },
  };
}

// Type helpers to constrain generic types
type TypeMetadataKeys = {
  [K in keyof TypeRegistry]: TypeRegistry[K][number] extends TypeMetadata ? K : never;
}[keyof TypeRegistry];

type EntityMetadataKeys = {
  [K in keyof TypeRegistry]: TypeRegistry[K][number] extends EntityMetadata ? K : never;
}[keyof TypeRegistry];

// Full TypeMetadata helpers (includes color methods + other methods)
function createTypeMetadataHelpers<T extends TypeMetadataKeys>(category: T) {
  const items = derived(registry, $registry => $registry?.[category] || []);
  const colorHelpers = createSharedHelpers(category);
  
  const helpers = {
    getItems: () => {
      const $registry = get(registry)
      return $registry?.[category] as TypeMetadata[]
    },
    
    getItem: (id: string | null) => {
      const $registry = get(registry);
      return ($registry?.[category] as TypeMetadata[])?.find(item => item.id === id) || null;
    },
    
    getDisplay: (id: string | null) => {
      const $registry = get(registry);
      return ($registry?.[category] as TypeMetadata[])?.find(item => item.id === id)?.display_name || id || "";
    },
    
    getDescription: (id: string | null) => {
      const $registry = get(registry);
      return ($registry?.[category] as TypeMetadata[])?.find(item => item.id === id)?.description || "";
    },

    getCategory: (id: string | null) => {
      const $registry = get(registry);
      return ($registry?.[category] as TypeMetadata[])?.find(item => item.id === id)?.category || "";
    },
    
    getMetadata: (id: string | null) => {
      const $registry = get(registry);
      return ($registry?.[category] as TypeMetadata[])?.find(item => item.id === id)?.metadata || {};
    },

    // Include the shared color methods
    ...colorHelpers
  };

  return helpers;
}

// EntityMetadata helpers (only color methods)
function createEntityMetadataHelpers<T extends EntityMetadataKeys>(category: T) {
  const items = derived(registry, $registry => $registry?.[category] || []);
  const colorHelpers = createSharedHelpers(category);
  
  const helpers = {
    getItems: () => {
      const $registry = get(registry)
      return $registry?.[category] as EntityMetadata[]
    },
    
    getItem: (id: string | null) => {
      const $registry = get(registry);
      return ($registry?.[category] as EntityMetadata[])?.find(item => item.id === id) || null;
    },
    
    // Only include the shared color methods
    ...colorHelpers
  };

  return helpers;
}

// Create all the helpers
export const serviceTypes = createTypeMetadataHelpers('service_types');
export const subnetTypes = createTypeMetadataHelpers('subnet_types');
export const edgeTypes = createTypeMetadataHelpers('edge_types');
export const entities = createEntityMetadataHelpers('entities');

export async function getRegistry() {
  const result = await api.request<TypeRegistry>(
    '/registry',
    registry,
    (registry) => registry,
    { method: 'GET', },
  )
}