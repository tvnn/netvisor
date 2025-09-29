<script lang="ts">
	import { formatId, formatTimestamp } from '$lib/shared/utils/formatting';
	import { Calendar, Clock, Hash, ChevronDown, ChevronRight } from 'lucide-svelte';

	export let title: string;
	export let subtitle: string;
</script>

<div class="flex min-h-0 flex-1 items-center justify-center text-gray-400">
	<div class="text-center">
		<div class="mb-2 text-lg">{title}</div>
		<div class="text-sm">{subtitle}</div>
	</div>
</div>
