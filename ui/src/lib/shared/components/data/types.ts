export interface TagProps {
  label: string,
  textColor?: string,
  bgColor?: string
  color?: string
}

export interface CardAction {
  label: string;
  icon: any; // Svelte component
  color?: string;
  hoverColor?: string;
  bgHover?: string;
  onClick: () => void;
  disabled?: boolean;
  animation?: string;
}

export interface CardSection {
  label: string;
  value: string;
}

export interface CardListItem {
  id: string;
  label: string;
  icon?: any; // Svelte component instead of HTML
  iconColor?: string;
  bgColor?: string;
  color?: string;
  disabled?: boolean;
  metadata?: any;
  badge?: string; // For things like "5m", "Critical", etc.
  badgeColor?: string;
}

export interface CardList {
  label: string;
  items: CardListItem[];
  emptyText?: string;
  itemActions?: (item: CardListItem) => CardAction[];
}