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

export interface TypeRegistry {
  service_types: TypeMetadata[];
  subnet_types: TypeMetadata[];
  edge_types: TypeMetadata[];
}

export const registry = writable<TypeRegistry>();

function createRegistryHelpers<T extends keyof TypeRegistry>(category: T) {
  const items = derived(registry, $registry => $registry?.[category] || []);
  
  const helpers = {
    getItems: () => {
      const $registry = get(registry)
      return $registry?.[category]
    },
    
    getItem: (id: string | null) => {
      const $registry = get(registry);
      return $registry?.[category]?.find(item => item.id === id) || null;
    },
    
    getDisplay: (id: string | null) => {
      const $registry = get(registry);
      return $registry?.[category]?.find(item => item.id === id)?.display_name || id || "";
    },
    
    getDescription: (id: string | null) => {
      const $registry = get(registry);
      return $registry?.[category]?.find(item => item.id === id)?.description || "";
    },
    
    getIcon: (id: string | null) => {
      const $registry = get(registry);
      return $registry?.[category]?.find(item => item.id === id)?.icon || 'help-circle';
    },

    getCategory: (id: string | null) => {
      const $registry = get(registry);
      return $registry?.[category]?.find(item => item.id === id)?.category || "";
    },
    
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
    
    getIconComponent: (id: string | null) => {
      const $registry = get(registry);
      const item = $registry?.[category]?.find(item => item.id === id);
      const iconName = item?.icon || null;
      return createIconComponent(iconName);
    },
    
    getStyle: (id: string | null) => {
      const $registry = get(registry);
      const item = $registry?.[category]?.find(item => item.id === id);
      const color = item?.color || null;
      const icon = item?.icon || null;
      return createStyle(color, icon);
    },
    
    getMetadata: (id: string | null) => {
      const $registry = get(registry);
      return $registry?.[category]?.find(item => item.id === id)?.metadata || {};
    }
  };

  return helpers;
}

// Create all the helpers
export const serviceTypes = createRegistryHelpers('service_types');
export const subnetTypes = createRegistryHelpers('subnet_types');
export const edgeTypes = createRegistryHelpers('edge_types');

export async function getRegistry() {
  const result = await api.request<TypeRegistry>(
    '/registry',
    registry,
    (registry) => registry,
    { method: 'GET', },
  )
}