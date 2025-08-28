export interface InitiateDiscoveryRequest {
    daemon_id: string
}

export interface InitiateDiscoveryResponse {
    session_id: string
}

export interface DiscoverySessionRequest {
    session_id: string
}

export interface CancelDiscoveryResponse {
    session_id: string
}

export interface DiscoveryStatusResponse {
    session: {
        session_id: string,
        daemon_id: string,
        status: string,
        progress?: {
            session_id: string,
            phase: string,
            completed: number,
            total: number,
            discovered_count: number,
        },
        error_message?: string,
        started_at?: string,
        completed_at?: string,
    }
}