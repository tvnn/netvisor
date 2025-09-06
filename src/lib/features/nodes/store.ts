import { writable } from 'svelte/store';
import type { Node } from "./types/base";
import { api } from '../../shared/utils/api';
import type { NodeTarget } from './types/targets';
import { pushInfo, pushWarning } from '$lib/shared/stores/feedback';
import { utcTimeZoneSentinel, uuidv4Sentinel } from '$lib/shared/utils/formatting';

export const nodes = writable<Node[]>([]);
export const polling = writable(false);

// // Create node polling instance
// let nodePoller: Poller | null = null;

// export function startNodePolling() {  
//   nodePoller = createPoller({
//     intervalMs: 5000, // 5 seconds
//     onPoll: async () => {
//       await getNodes();
//     },
//     onError: (pollingError) => {
//       pushError(`Failed to poll node status: ${pollingError}`);
//       stopNodePolling();
//     },
//     name: 'NodePoller'
//   });
  
//   nodePoller.start();
// }

// export async function stopNodePolling() {
//   if (nodePoller) {
//     nodePoller.stop();
//     nodePoller = null;
//   }
// }

export async function getNodes() {
  return await api.request<Node[]>(
    '/nodes',
    nodes,
    (nodes) => nodes,
    { method: 'GET', },
    true
  )
}

export async function createNode(data: Node) {
  return await api.request<Node, Node[]>(
    '/nodes',
    nodes,
    (node, current) => [...current, node],
    { method: 'POST', body: JSON.stringify(data)},
  )
}

interface UpdateNodeResponse {
  node: Node,
  // capability_test_changes: Record<string, NodeCapabilityTestChange>,
  subnet_changes: NodeSubnetRelationshipChange
}

// interface NodeCapabilityTestChange {
//     newly_compatible: string[], 
//     incompatible: string[]
// }

interface NodeSubnetRelationshipChange {
  new_gateway: Subnet[],
  no_longer_gateway: Subnet[],
  new_dns_resolver: Subnet[],
  no_longer_dns_resolver: Subnet[]
}

export async function updateNode(data: Node) {
  return await api.request<UpdateNodeResponse, Node[]>(
    `/nodes/${data.id}`,
    nodes,
    (updatedNodeResponse, current) => {
      const updatedNode = updatedNodeResponse.node;

      // Object.keys(updatedNodeResponse.capability_test_changes).forEach(cap => {
      //   let incompatible = updatedNodeResponse.capability_test_changes[cap].incompatible.map(i => testTypes.getDisplay(i))
      //   let newly_compatible = updatedNodeResponse.capability_test_changes[cap].newly_compatible.map(n => testTypes.getDisplay(n))
      //   incompatible.length > 0 ? pushWarning(`The following tests are no longer compatible with node "${updatedNode.name}" and have been removed: ${incompatible.join(", ")}`) : null
      //   newly_compatible.length > 0 ? pushInfo(`The following tests are now compatible with node "${updatedNode.name}" and have been added: ${newly_compatible.join(", ")}`) : null
      // })

      if (updatedNodeResponse.subnet_changes.new_dns_resolver.length > 0) {
        pushInfo(`The following subnets now have node "${updatedNode.name}" set as a DNS resolver: ${
          updatedNodeResponse.subnet_changes.new_dns_resolver.map(d => `${d.name} (${d.cidr})`).join(", ")
        }`)
      }

      if (updatedNodeResponse.subnet_changes.new_gateway.length > 0) {
        pushInfo(`The following subnets now have node "${updatedNode.name}" set as a gateway: ${
          updatedNodeResponse.subnet_changes.new_gateway.map(d => `${d.name} (${d.cidr})`).join(", ")
        }`)
      }

      if (updatedNodeResponse.subnet_changes.no_longer_dns_resolver.length > 0) {
        pushWarning(`The following subnets no longer have node "${updatedNode.name}" set as a gateway: ${
          updatedNodeResponse.subnet_changes.no_longer_dns_resolver.map(d => `${d.name} (${d.cidr})`).join(", ")
        }`)
      }

      if (updatedNodeResponse.subnet_changes.no_longer_gateway.length > 0) {
        pushWarning(`The following subnets no longer have node "${updatedNode.name}" set as a gateway: ${
          updatedNodeResponse.subnet_changes.no_longer_gateway.map(d => `${d.name} (${d.cidr})`).join(", ")
        }`)
      }

      return current.map(n => n.id === data.id ? updatedNode : n)
    },
    { method: 'PUT', body: JSON.stringify(data)},
  )
}

export async function deleteNode(id: string) {
  return await api.request<void, Node[]>(
    `/nodes/${id}`,
    nodes,
    (_, current) => current.filter(g => g.id !== id),
    { method: 'DELETE'},
  )
}


export function createEmptyNodeFormData(): Node {
  return {
    id: uuidv4Sentinel,
    created_at: utcTimeZoneSentinel,
    updated_at: utcTimeZoneSentinel,
    name: '',
    description: '',
    hostname: '',
    target: {
      type: 'IpAddress',
      config: {
        ip: '',
      },
    },
    services: [],
    subnets: [],
    last_seen: utcTimeZoneSentinel,
    node_groups: [],
    discovery_status: 'Manual',
  };
}

export function getNodeTargetString(target: NodeTarget): string {
  switch (target.type) {
    case 'IpAddress':
      return target.config.ip;
    case 'Hostname':
      return target.config.hostname;
    default:
      return 'Unknown target';
  }
}
