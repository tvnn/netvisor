<script lang="ts">
  import type { TestConfigSchema } from '$lib/components/tests/types';
  export let schema: TestConfigSchema;
</script>

<div class="space-y-3">
  <!-- Compatibility Status -->
  <div class="flex items-center gap-3 p-3 rounded-lg border
              {schema.compatibility === 'Compatible' ? 'bg-green-900/20 border-green-600' :
               schema.compatibility === 'Conditional' ? 'bg-yellow-900/20 border-yellow-600' :
               'bg-red-900/20 border-red-600'}">
    <iconify-icon 
      icon="mdi:{schema.compatibility === 'Compatible' ? 'check-circle' :
                 schema.compatibility === 'Conditional' ? 'alert' :
                 'close-circle'}"
      class="{schema.compatibility === 'Compatible' ? 'text-green-400' :
              schema.compatibility === 'Conditional' ? 'text-yellow-400' :
              'text-red-400'}"
    ></iconify-icon>
    
    <div class="flex-1">
      <span class="text-sm font-medium 
                   {schema.compatibility === 'Compatible' ? 'text-green-200' :
                     schema.compatibility === 'Conditional' ? 'text-yellow-200' :
                     'text-red-200'}">
        {#if schema.compatibility === 'Compatible'}
          Compatible
        {:else if schema.compatibility === 'Conditional'}
          Conditionally Compatible
        {:else}
          Incompatible
        {/if}
      </span>
      
      {#if schema.compatibility_reason}
        <p class="text-xs mt-1
                  {schema.compatibility === 'Compatible' ? 'text-green-300' :
                    schema.compatibility === 'Conditional' ? 'text-yellow-300' :
                    'text-red-300'}">
          {schema.compatibility_reason}
        </p>
      {/if}
    </div>
  </div>
  
  <!-- Error Messages -->
  {#if schema.errors.length > 0}
    <div class="space-y-2">
      {#each schema.errors as error}
        <div class="flex items-start gap-2 p-3 bg-red-900/20 border border-red-600 rounded-lg">
          <iconify-icon icon="mdi:alert-circle" class="text-red-400 mt-0.5"></iconify-icon>
          <div class="flex-1">
            <span class="text-sm text-red-200">{error.message}</span>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>