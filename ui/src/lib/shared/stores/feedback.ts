import { get, writable } from 'svelte/store';

export type ToastType = 'error' | 'warning' | 'info' | 'success';

export interface Toast {
	id: string;
	type: ToastType;
	message: string;
	timeout?: number;
	dismissible?: boolean;
	actions?: ToastAction[];
}

export interface ToastAction {
	label: string;
	action: () => void;
	style?: 'primary' | 'secondary';
}

export const toastStore = writable<Toast[]>([]);

let toastIdCounter = 0;

function generateToastId(): string {
	return `toast-${++toastIdCounter}-${Date.now()}`;
}

function addToast(toast: Omit<Toast, 'id'>): string {
	const id = generateToastId();
	const fullToast: Toast = {
		id,
		dismissible: true,
		...toast
	};

	toastStore.update((toasts) => [...toasts, fullToast]);

	// Auto-dismiss if timeout is set
	if (fullToast.timeout && fullToast.timeout > 0) {
		setTimeout(() => {
			dismissToast(id);
		}, fullToast.timeout);
	}

	return id;
}

export function pushError(message: string, timeout: number = 10000): string {
	console.error(message);
	return addToast({
		type: 'error',
		message,
		timeout
	});
}

export function pushWarning(message: string, timeout: number = 10000): string {
	console.warn(message);
	return addToast({
		type: 'warning',
		message,
		timeout
	});
}

export function pushInfo(message: string, timeout: number = 10000): string {
	console.info(message);
	return addToast({
		type: 'info',
		message,
		timeout
	});
}

export function pushSuccess(message: string, timeout: number = 10000): string {
	return addToast({
		type: 'success',
		message,
		timeout
	});
}

export function pushToast(toast: Omit<Toast, 'id'>): string {
	return addToast(toast);
}

export function dismissToast(id: string) {
	toastStore.update((toasts) => toasts.filter((t) => t.id !== id));
}

export function clearAllToasts() {
	toastStore.set([]);
}

// Legacy support
export function clearError(message: string) {
	toastStore.update((toasts) =>
		toasts.filter((t) => !(t.type === 'error' && t.message === message))
	);
}

// Keep errorStore for backward compatibility
export const errorStore = writable<string[]>([]);

// Sync errorStore with error toasts for backward compatibility
toastStore.subscribe((toasts) => {
	const errorMessages = toasts.filter((t) => t.type === 'error').map((t) => t.message);
	errorStore.set(errorMessages);
});
