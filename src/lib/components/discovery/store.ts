import { writable, get } from 'svelte/store';
import { api } from '../../api/client';

export const session_id = writable<string | null>(null);
export const discoveryStatus = writable<string | null>(null);
export const loading = writable(false);
export const error = writable<string | null>(null);

// Use a regular variable to store the interval ID
let pollerIntervalId: ReturnType<typeof setInterval> | null = null;

async function pollDiscoveryStatus(sessionId: string) {
  try {
    const response = await api.discoveryStatus(sessionId);
    if (response.success && response.data) {
      discoveryStatus.set(response.data.phase);
      
      // Stop polling if discovery is finished
      const status = response.data.phase;
      if (['Completed', 'Failed', 'Cancelled'].includes(status)) {
        stopPolling();
      }
    } else {
      error.set(response.error || 'Failed to get discovery status');
      stopPolling();
    }
  } catch (err) {
    error.set('Network error while polling');
    console.error('Failed to poll discovery status:', err);
    stopPolling();
  }
}

function startPolling(sessionId: string) {
  // Stop any existing polling first
  stopPolling();
  
  // Start polling every 5 seconds
  pollerIntervalId = setInterval(() => {
    pollDiscoveryStatus(sessionId);
  }, 5000);
  
  // Do an initial poll immediately
  pollDiscoveryStatus(sessionId);
}

function stopPolling() {
  if (pollerIntervalId) {
    clearInterval(pollerIntervalId);
    pollerIntervalId = null;
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
        session_id.set(newSessionId);
        startPolling(newSessionId);
      } else {
        error.set(response.error || 'Failed to initiate discovery');
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to initiate discovery:', err);
    } finally {
      loading.set(false);
    }
  },

  async cancelDiscovery(id: string) {
    loading.set(true);
    error.set(null);
    
    try {
      const response = await api.cancelDiscovery(id);
      if (response.success) {
        // Don't clear session_id and status immediately - let polling handle the status update
        // The server will mark it as cancelled and polling will pick up the change
        // stopPolling(); // Don't stop polling yet - wait for cancelled status
      } else {
        error.set(response.error || 'Failed to cancel discovery');
      }
    } catch (err) {
      error.set('Network error');
      console.error('Failed to cancel discovery:', err);
    } finally {
      loading.set(false);
    }
  },

  // Reset all state and stop polling
  reset() {
    stopPolling();
    session_id.set(null);
    discoveryStatus.set(null);
    error.set(null);
    loading.set(false);
  },

  clearError() {
    error.set(null);
  }
};

// Cleanup function to be called when component unmounts
export function cleanupDiscoveryPolling() {
  stopPolling();
}