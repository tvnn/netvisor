import { writable, get } from 'svelte/store';
import { api } from '../../api/client';
import { daemons } from '../daemons/store';
import { nodeActions, nodes } from '../nodes/store';
import { createPoller, type Poller } from '../../utils/polling';

// Existing stores for the currently active/displayed session
export const session_id = writable<string | null>(null);
export const discoveryStatus = writable<string | null>(null);
export const loading = writable(false);
export const error = writable<string | null>(null);

// Current session details (for the session being actively displayed/polled)
export const completed = writable<number>(0);
export const total = writable<number>(0);
export const discovered_count = writable<number>(0);

// Per-daemon session tracking - this is the key store for multi-daemon support
export const daemonSessions = writable<Map<string, string>>(new Map()); // daemon_id -> session_id

// Discovery status poller instance
let discoveryPoller: Poller | null = null;

async function pollDiscoveryStatus(sessionId: string) {
  const response = await api.discoveryStatus(sessionId);
  
  if (response.success && response.data) {
    // Only update the current session stores if this is the actively displayed session
    if (get(session_id) === sessionId) {

      if (get(discovered_count) < response.data.discovered_count) {
        nodeActions.loadNodes(false);
      }

      discoveryStatus.set(response.data.phase);
      completed.set(response.data.completed);
      total.set(response.data.total);
      discovered_count.set(response.data.discovered_count);
      
      // Update error state
      if (response.data.error) {
        error.set(response.data.error);
      } else {
        error.set(null);
      }
    }
    
    // Stop polling if discovery is finished
    const status = response.data.phase;
    if (['Completed', 'Failed', 'Cancelled'].includes(status)) {
      stopPolling();
      loading.set(false);
      
      // Clean up daemon session mapping when completed
      daemonSessions.update(sessions => {
        for (const [daemonId, sessId] of sessions.entries()) {
          if (sessId === sessionId) {
            sessions.delete(daemonId);
            break;
          }
        }
        return sessions;
      });
      
      // Reset current session if this was the active one
      if (get(session_id) === sessionId) {
        session_id.set(null);
        completed.set(0);
        total.set(0);
        discovered_count.set(0);
      }
    }
  } else {
    throw new Error(response.error || 'Failed to get discovery status');
  }
}

function startPolling(sessionId: string) {
  // Stop any existing polling first
  stopPolling();
  
  // Create and start new discovery poller
  discoveryPoller = createPoller({
    intervalMs: 5000, // 5 seconds
    onPoll: async () => {
      await pollDiscoveryStatus(sessionId);
    },
    onError: (pollingError) => {
      error.set('Network error while polling');
      console.error('Failed to poll discovery status:', pollingError);
      stopPolling();
    },
    name: 'DiscoveryPoller'
  });
  
  discoveryPoller.start();
}

function stopPolling() {
  if (discoveryPoller) {
    discoveryPoller.stop();
    discoveryPoller = null;
  }
}

export const discoveryActions = {
  async initiateDiscovery(daemon_id: string) {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.initiateDiscovery({ daemon_id });
      
      if (response.success && response.data) {
        const newSessionId = response.data.session_id;
        
        // Update daemon-specific session mapping
        daemonSessions.update(sessions => {
          sessions.set(daemon_id, newSessionId);
          return sessions;
        });
        
        // Set this as the current active session for display/polling
        session_id.set(newSessionId);
        completed.set(0);
        total.set(0);
        discovered_count.set(0);
        
        startPolling(newSessionId);
      } else {
        error.set(response.error || 'Failed to initiate discovery');
      }
    } catch (err) {
      error.set('Network error');
    } finally {
      loading.set(false);
    }
  },

  async cancelDiscovery(sessionId: string) {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.cancelDiscovery(sessionId);
      if (response.success) {
        // Don't clear session_id and status immediately - let polling handle the status update
        // The server will mark it as cancelled and polling will pick up the change
      } else {
        error.set(response.error || 'Failed to cancel discovery');
      }
    } catch (err) {
      error.set('Network error');
      loading.set(false);
      console.error('Failed to cancel discovery:', err);
    }
  },

  async checkActiveDiscoverySessions() {
    try {
      const response = await api.getActiveDiscoverySessions();
      if (response.success && response.data && response.data.length > 0) {
        // Update daemon session mappings for all active sessions
        const sessions = new Map<string, string>();
        
        for (const session of response.data) {
          sessions.set(session.daemon_id, session.session_id);
        }
        
        daemonSessions.set(sessions);
        
        // If there are active sessions but no current session, pick the first one to display
        if (response.data.length > 0 && get(session_id) === null) {
          const firstSession = response.data[0];
          
          session_id.set(firstSession.session_id);
          discoveryStatus.set(firstSession.phase);
          completed.set(firstSession.completed);
          total.set(firstSession.total);
          discovered_count.set(firstSession.discovered_count);
          
          if (firstSession.error) {
            error.set(firstSession.error);
          }
          
          startPolling(firstSession.session_id);
        }
      }
    } catch (error) {
      console.error('Failed to check for active discovery sessions:', error);
    }
  },

  // Get session for specific daemon
  getDaemonSessionId(daemon_id: string): string | null {
    return get(daemonSessions).get(daemon_id) || null;
  },

  // Switch to monitoring a different daemon's session
  async switchToDaemonSession(daemon_id: string) {
    const sessionId = this.getDaemonSessionId(daemon_id);
    if (sessionId) {
      session_id.set(sessionId);
      
      // Get current status for this session
      try {
        const response = await api.discoveryStatus(sessionId);
        if (response.success && response.data) {
          discoveryStatus.set(response.data.phase);
          completed.set(response.data.completed);
          total.set(response.data.total);
          discovered_count.set(response.data.discovered_count);
          
          if (response.data.error) {
            error.set(response.data.error);
          } else {
            error.set(null);
          }
        }
      } catch (err) {
        console.error('Failed to get discovery status:', err);
        error.set('Failed to get discovery status');
      }
      
      startPolling(sessionId);
    }
  },

  // Helper function to get daemon name from daemon and nodes stores
  getDaemonDisplayName(daemon_id: string): string {
    const daemon = get(daemons).find(d => d.id === daemon_id);
    if (!daemon) return 'Unknown Daemon';
    
    const node = get(nodes).find(n => n.id === daemon.node_id);
    return node ? node.name : `Daemon ${daemon.id.substring(0, 8)}`;
  },

  // Helper function to get node name for a daemon
  getDaemonNodeName(daemon_id: string): string {
    const daemon = get(daemons).find(d => d.id === daemon_id);
    if (!daemon) return 'Unknown Node';
    
    const node = get(nodes).find(n => n.id === daemon.node_id);
    return node ? node.name : 'Unknown Node';
  },

  // Get polling status
  getDiscoveryPollingStatus(): boolean {
    return discoveryPoller?.getIsRunning() ?? false;
  },

  // Reset all state and stop polling
  reset() {
    stopPolling();
    session_id.set(null);
    discoveryStatus.set(null);
    completed.set(0);
    total.set(0);
    discovered_count.set(0);
    error.set(null);
    loading.set(false);
    daemonSessions.set(new Map());
  },

  clearError() {
    error.set(null);
  }
};

// Cleanup function to be called when component unmounts
export function cleanupDiscoveryPolling() {
  stopPolling();
}