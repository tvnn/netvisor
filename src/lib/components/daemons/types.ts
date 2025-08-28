export interface Daemon {
    id: string;
    registered_at: string;
    last_seen: string;
    node_id: string;
}

export interface DaemonListResponse {
  daemons: Daemon[];
  total: number;
}
