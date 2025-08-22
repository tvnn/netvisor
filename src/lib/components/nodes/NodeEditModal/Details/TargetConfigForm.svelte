<script lang="ts">
  import { type NodeTarget, ApplicationProtocol, validateTarget } from "$lib/components/nodes/types";
  
  export let target: NodeTarget;

  // Reactive validation
  let targetErrors: string[] = []

  const protocolOptions = [
    { value: ApplicationProtocol.Http, label: 'HTTP' },
    { value: ApplicationProtocol.Https, label: 'HTTPS' },
    { value: ApplicationProtocol.Ftp, label: 'FTP' }
  ];
  
  function updateConfig(field: string, value: any) {
    target.config = {
      ...target.config,
      [field]: value
    };
  }

</script>

<div class="space-y-4">
  {#if target.type === 'IpAddress'}
    <!-- IP Address Configuration -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="md:col-span-2">
        <label for="ip_address" class="block text-sm font-medium text-gray-300 mb-1">
          IP Address
        </label>
        <input
          id="ip_address"
          bind:value={target.config.ip}
          type="text"
          required
          placeholder="192.168.1.100"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          class:border-red-500={targetErrors.includes('IP address is required')}
        />
      </div>
      
      <div>
        <label for="ip_port" class="block text-sm font-medium text-gray-300 mb-1">
          Port
        </label>
        <input
          id="ip_port"
          bind:value={target.config.port}
          type="number"
          min="1"
          max="65535"
          placeholder="80"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          class:border-red-500={targetErrors.includes('Port must be between 1 and 65535')}
        />
      </div>
    </div>

  {:else if target.type === 'Hostname'}
    <!-- Hostname Configuration -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="md:col-span-2">
        <label for="hostname" class="block text-sm font-medium text-gray-300 mb-1">
          Hostname *
        </label>
        <input
          id="hostname"
          bind:value={target.config.hostname}
          type="text"
          required
          placeholder="server.local"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          class:border-red-500={targetErrors.includes('Hostname is required')}
        />
      </div>
      
      <div>
        <label for="hostname_port" class="block text-sm font-medium text-gray-300 mb-1">
          Port
        </label>
        <input
          id="hostname_port"
          bind:value={target.config.port}
          type="number"
          min="1"
          max="65535"
          placeholder="80"
          class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          class:border-red-500={targetErrors.includes('Port must be between 1 and 65535')}
        />
      </div>
    </div>

  {:else if target.type === 'Service'}
    <!-- Service URL Configuration -->
    <div class="space-y-4">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label for="service_protocol" class="block text-sm font-medium text-gray-300 mb-1">
            Protocol *
          </label>
          <select
            id="service_protocol"
            bind:value={target.config.protocol}
            required
            class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            class:border-red-500={targetErrors.includes('Protocol is required')}
          >
            {#each protocolOptions as option}
              <option value={option.value}>{option.label}</option>
            {/each}
          </select>
        </div>
        
        <div>
          <label for="service_hostname" class="block text-sm font-medium text-gray-300 mb-1">
            Hostname *
          </label>
          <input
            id="service_hostname"
            bind:value={target.config.hostname}
            type="text"
            required
            placeholder="api.example.com"
            class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            class:border-red-500={targetErrors.includes('Hostname is required')}
          />
        </div>
      </div>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label for="service_port" class="block text-sm font-medium text-gray-300 mb-1">
            Port
          </label>
          <input
            id="service_port"
            bind:value={target.config.port}
            type="number"
            min="1"
            max="65535"
            placeholder="80"
            class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            class:border-red-500={targetErrors.includes('Port must be between 1 and 65535')}
          />
        </div>
        
        <div>
          <label for="service_path" class="block text-sm font-medium text-gray-300 mb-1">
            Path
          </label>
          <input
            id="service_path"
            bind:value={target.config.path}
            type="text"
            placeholder="/api/v1/health"
            class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
      </div>
    </div>
  {/if}

  <!-- Show validation errors -->
  {#if targetErrors.length > 0}
    <div class="text-red-400 text-xs space-y-1">
      {#each targetErrors as error}
        <p>{error}</p>
      {/each}
    </div>
  {/if}
</div>