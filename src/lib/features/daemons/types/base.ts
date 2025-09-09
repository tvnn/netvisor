export interface DaemonBase {
    host_id: string;
}

export interface Daemon extends DaemonBase {
  id: string;
  registered_at: string;
  last_seen: string;
}
