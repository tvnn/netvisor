export type EntitySource =
	| { type: 'Manual' }
	| { type: 'System' }
	| { type: 'Unknown' }
	| {
			type: 'Discovery';
			metadata: {
				discovery_type: DiscoveryType;
				daemon_id: string;
				details: MatchResult;
			};
	  };

export type MatchReason =
	| { type: 'reason'; data: string }
	| { type: 'container'; data: [string, MatchReason[]] };

export interface MatchResult {
	reason: MatchReason;
	confidence: 'Low' | 'Medium' | 'High' | 'Certain';
}

export type DiscoveryType =
	| { type: 'SelfReport' }
	| { type: 'Network' }
	| { type: 'Docker'; host_id: string }
	| { type: 'Proxmox'; host_id: string };
