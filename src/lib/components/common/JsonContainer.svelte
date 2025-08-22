<script lang="ts">
  import { ChevronDown, ChevronRight, Copy, Check } from 'lucide-svelte';
  
  export let data: any;
  export let title: string = "JSON Data";
  export let maxHeight: string = "max-h-96";
  export let initiallyExpanded: boolean = false;
  export let showCopyButton: boolean = true;
  export let indent: number = 2;
  
  let isExpanded = initiallyExpanded;
  let copySuccess = false;
  
  $: prettyJson = JSON.stringify(data, null, indent);
  $: jsonLines = prettyJson ? prettyJson.split('\n').length : 0;
  
  async function copyToClipboard() {
    try {
      await navigator.clipboard.writeText(prettyJson);
      copySuccess = true;
      setTimeout(() => {
        copySuccess = false;
      }, 2000);
    } catch (err) {
      console.error('Failed to copy to clipboard:', err);
      // Fallback for older browsers
      const textArea = document.createElement('textarea');
      textArea.value = prettyJson;
      document.body.appendChild(textArea);
      textArea.select();
      document.execCommand('copy');
      document.body.removeChild(textArea);
      copySuccess = true;
      setTimeout(() => {
        copySuccess = false;
      }, 2000);
    }
  }
  
  function toggleExpanded() {
    isExpanded = !isExpanded;
  }
</script>

<div class="bg-gray-800 rounded-lg overflow-hidden">
  <!-- Header -->
  <div class="flex items-center justify-between px-4 py-3">
    <button 
      class="flex items-center gap-2 text-gray-300 hover:text-white transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 rounded"
      on:click={toggleExpanded}
      type="button"
    >
      {#if isExpanded}
        <ChevronDown size={16} class="text-gray-500 transition-transform" />
      {:else}
        <ChevronRight size={16} class="text-gray-500 transition-transform" />
      {/if}
      <span class="font-medium">{title}</span>
      {#if !isExpanded && jsonLines}
        <span class="text-xs text-gray-500 bg-gray-700 px-2 py-1 rounded-full">
          {jsonLines} lines
        </span>
      {/if}
    </button>
    
    {#if showCopyButton}
      <button
        class="p-2 text-gray-400 hover:text-white hover:bg-gray-600 rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
        on:click={copyToClipboard}
        title={copySuccess ? "Copied!" : "Copy to clipboard"}
        type="button"
      >
        {#if copySuccess}
          <Check size={16} class="text-green-400" />
        {:else}
          <Copy size={16} />
        {/if}
      </button>
    {/if}
  </div>
  
  <!-- Content -->
  {#if isExpanded}
    <div class="relative">
      <div class="p-4">
        <div class="relative">
          <pre 
            class="text-gray-300 text-sm font-mono whitespace-pre overflow-auto bg-gray-900 p-4 rounded-lg {maxHeight} scrollbar-thin scrollbar-track-gray-800 scrollbar-thumb-gray-600 hover:scrollbar-thumb-gray-500"
          ><code class="block leading-relaxed">{prettyJson}</code></pre>
          
          <!-- Optional: Add line numbers -->
          <!-- <div class="absolute top-0 left-0 p-4 text-xs text-gray-500 pointer-events-none select-none">
            {#each Array(jsonLines) as _, i}
              <div class="h-5 leading-relaxed">{i + 1}</div>
            {/each}
          </div> -->
        </div>
      </div>
      
      <!-- Footer with metadata -->
      <div class="px-4 pb-3 flex justify-between items-center text-xs text-gray-500" style="border: none;">
        <span>{jsonLines} lines • {prettyJson.length} characters</span>
        <span>JSON</span>
      </div>
    </div>
  {/if}
</div>