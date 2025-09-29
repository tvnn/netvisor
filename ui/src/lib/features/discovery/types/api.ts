export interface InitiateDiscoveryRequest {
	daemon_id: string;
}

export interface DiscoverySessionRequest {
	session_id: string;
}

export interface DaemonDiscoveryUpdate {
	session_id: string;
	daemon_id: string;
	phase?: string;
	completed?: number;
	total?: number;
	discovered_count?: number;
	error?: string;
	started_at?: string;
	finished_at?: string;
}
