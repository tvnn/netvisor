<script lang="ts">
	import { AlertTriangle } from "lucide-svelte";

    export let title: string = "Warning";
    export let body: string;

</script>
<div class="bg-yellow-900/20 border border-yellow-600/30 rounded-lg p-4">
    <div class="flex items-start gap-2">
    <AlertTriangle class="w-4 h-4 text-yellow-400 mt-0.5 shrink-0" />
    <div>
        <p class="text-yellow-400 text-sm font-medium">{title}</p>
        <p class="text-yellow-300/80 text-sm mt-1">{body}</p>
    </div>
    </div>
</div>