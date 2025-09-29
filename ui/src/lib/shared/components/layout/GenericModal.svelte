<script lang="ts">
	import { X } from 'lucide-svelte';
	import { onDestroy } from 'svelte';

	export let title: string = 'Modal';
	export let isOpen: boolean = false;
	export let onClose: (() => void) | null = null;
	export let size: 'sm' | 'md' | 'lg' | 'xl' | 'full' = 'lg';
	export let preventCloseOnClickOutside: boolean = false;
	export let showCloseButton: boolean = true;

	$: if (typeof window !== 'undefined' && isOpen) {
		document.body.style.overflow = 'hidden';
	} else if (typeof window !== 'undefined') {
		document.body.style.overflow = '';
	}

	onDestroy(() => {
		if (typeof window !== 'undefined') {
			document.body.style.overflow = '';
		}
	});

	// Size classes
	const sizeClasses = {
		sm: 'max-w-md',
		md: 'max-w-lg',
		lg: 'max-w-2xl',
		xl: 'max-w-4xl',
		full: 'max-w-7xl'
	};

	function handleClose() {
		onClose?.();
	}

	function handleBackdropClick(event: MouseEvent) {
		if (!preventCloseOnClickOutside && event.target === event.currentTarget) {
			handleClose();
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape' && isOpen) {
			handleClose();
		}
	}
</script>

<svelte:window on:keydown={handleKeydown} />

{#if isOpen}
	<!-- Modal backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center overscroll-contain bg-black/50 p-4 backdrop-blur-sm"
		on:click={handleBackdropClick}
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		on:keydown={(e) => e.key === 'Escape' && handleClose()}
		tabindex="-1"
	>
		<!-- Modal content -->
		<div
			class="w-full rounded-lg border border-gray-700 bg-gray-900 shadow-2xl {sizeClasses[
				size
			]} {size === 'full' ? 'h-[95vh]' : 'max-h-[95vh]'} flex flex-col"
		>
			<!-- Header -->
			<div class="flex shrink-0 items-center justify-between border-b border-gray-700 p-6">
				<div class="flex items-center gap-3">
					{#if $$slots['header-icon']}
						<slot name="header-icon" />
					{/if}
					<h2 id="modal-title" class="text-xl font-semibold text-white">
						{title}
					</h2>
				</div>

				{#if showCloseButton}
					<button
						type="button"
						on:click={handleClose}
						class="rounded-lg p-2 text-gray-400 transition-colors hover:bg-gray-700 hover:text-white"
						aria-label="Close modal"
					>
						<X class="h-5 w-5" />
					</button>
				{/if}
			</div>

			<!-- Content slot -->
			<div class="min-h-0 flex-1 overflow-auto scroll-smooth">
				<slot />
			</div>

			<!-- Footer slot -->
			{#if $$slots.footer}
				<div class="shrink-0 border-t border-gray-700 p-6">
					<slot name="footer" />
				</div>
			{/if}
		</div>
	</div>
{/if}
