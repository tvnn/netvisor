<script lang="ts">
  import { CheckCircle, AlertTriangle, Clock, Loader2 } from 'lucide-svelte';
	import { getStatusBadgeClass, getStatusClass, getStatusIcon } from './store';
  
  export let status: string;

  $: StatusIcon = getStatusIcon(status);
</script>

<div class="flex items-center gap-2 whitespace-nowrap">
  <svelte:component 
    this={StatusIcon} 
    size={16} 
    class={getStatusClass(status)}
  />
  <span class="inline-flex px-2 py-1 text-xs font-medium rounded border {getStatusBadgeClass(status)}">
    {status}
  </span>
</div>