import type { CircleQuestionMark } from 'lucide-svelte';

export type IconComponent =
	| typeof CircleQuestionMark
	| (($$payload: Record<string, unknown>, $$props: Record<string, unknown>) => unknown);
