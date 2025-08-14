<!-- src/lib/components/modals/NodeEditor/CapabilitiesForm.svelte -->
<script lang="ts">
  import type { NodeCapability, NodeType } from "$lib/types/nodes";
  
  export let capabilities: NodeCapability[];
  export let nodeType: NodeType | string;
  
  const nodeCapabilities: NodeCapability[] = [
    'SshAccess', 'RdpAccess', 'VncAccess',
    'HttpService', 'HttpsService', 'DatabaseService',
    'DnsService', 'EmailService', 'FtpService'
  ];
  
  function getCapabilityDisplayName(capability: NodeCapability): string {
    const names: Record<NodeCapability, string> = {
      SshAccess: 'SSH Access',
      RdpAccess: 'RDP Access', 
      VncAccess: 'VNC Access',
      HttpService: 'HTTP Service',
      HttpsService: 'HTTPS Service',
      DatabaseService: 'Database Service',
      DnsService: 'DNS Service',
      EmailService: 'Email Service',
      FtpService: 'FTP Service'
    };
    return names[capability] || capability;
  }
  
  function getSuggestedCapabilities(nodeType: NodeType | string): NodeCapability[] {
    // This should match the backend logic for suggested capabilities
    switch (nodeType) {
      case 'WebServer':
        return ['HttpService', 'HttpsService', 'SshAccess'];
      case 'DatabaseServer':
        return ['DatabaseService', 'SshAccess'];
      case 'DnsServer':
        return ['DnsService', 'SshAccess'];
      case 'VpnServer':
        return ['SshAccess'];
      case 'NasDevice':
        return ['SshAccess', 'HttpService'];
      case 'MediaServer':
        return ['HttpService', 'SshAccess'];
      case 'Router':
      case 'Switch':
      case 'AccessPoint':
        return ['HttpService', 'SshAccess'];
      case 'Firewall':
        return ['HttpService', 'SshAccess'];
      case 'Workstation':
        return ['SshAccess', 'RdpAccess', 'VncAccess'];
      case 'Printer':
        return ['HttpService'];
      case 'Camera':
        return ['HttpService'];
      case 'IotDevice':
      case 'UnknownDevice':
      default:
        return [];
    }
  }
  
  $: suggestedCapabilities = getSuggestedCapabilities(nodeType);
  
  function handleCapabilityToggle(capability: NodeCapability) {
    if (capabilities.includes(capability)) {
      capabilities = capabilities.filter(c => c !== capability);
    } else {
      capabilities = [...capabilities, capability];
    }
  }
  
  function applySuggested() {
    capabilities = [...suggestedCapabilities];
  }
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <h3 class="text-lg font-medium text-white">Capabilities</h3>
    {#if suggestedCapabilities.length > 0}
      <button
        type="button"
        on:click={applySuggested}
        class="text-sm text-blue-400 hover:text-blue-300 underline"
      >
        Apply Suggested ({suggestedCapabilities.length})
      </button>
    {/if}
  </div>
  
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
    {#each nodeCapabilities as capability}
      <label class="flex items-center space-x-2 cursor-pointer">
        <input
          type="checkbox"
          checked={capabilities.includes(capability)}
          on:change={() => handleCapabilityToggle(capability)}
          class="rounded bg-gray-700 border-gray-600 text-blue-600 focus:ring-blue-500"
        />
        <span 
          class="text-sm text-gray-300 select-none"
          class:text-blue-300={suggestedCapabilities.includes(capability)}
          class:font-medium={suggestedCapabilities.includes(capability)}
        >
          {getCapabilityDisplayName(capability)}
          {#if suggestedCapabilities.includes(capability)}
            <span class="text-xs text-blue-400">(suggested)</span>
          {/if}
        </span>
      </label>
    {/each}
  </div>
  
  {#if capabilities.length === 0}
    <p class="text-sm text-gray-400">
      No capabilities selected. Capabilities help determine which tests are compatible with this node.
    </p>
  {:else}
    <p class="text-sm text-gray-400">
      {capabilities.length} capabilit{capabilities.length === 1 ? 'y' : 'ies'} selected
    </p>
  {/if}
</div>