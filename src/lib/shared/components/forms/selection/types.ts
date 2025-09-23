import type { TagProps } from "../../data/types";
import type { FormApi } from "../types";

export interface EntityDisplayComponent<T> {
  // Required methods
  getId(item: T): string;
  getLabel(item: T): string;
  
  // Optional methods with defaults
  getDescription?(item: T): string;
  getIcon?(item: T): any | null;
  getIconColor?(item: T): string | null;
  getTags?(item: T): TagProps[];
  getIsDisabled?(item: T): boolean;
  getCategory?(item: T): string | null;
  
  // Optional inline editing support
  supportsInlineEdit?: boolean;
  renderInlineEdit?(item: T, onUpdate: (updates: Partial<T>) => void): any;
}

export interface DisplayComponentProps<T> {
  item: T;
}