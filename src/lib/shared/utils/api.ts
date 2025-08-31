import type { Writable } from "svelte/store";

const API_BASE = 'http://localhost:3000/api';

interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

class ApiClient {
  async request<TResponseData, TStoreData = TResponseData>(
    endpoint: string,
    dataStore: Writable<TStoreData> | null,
    storeAction: ((data: TResponseData, current: TStoreData) => TStoreData) | null,
    errorStore: Writable<string | null>,
    loadingStore: Writable<boolean>,
    options: RequestInit = {},
    errorMessage?: string
  ): Promise<ApiResponse<TResponseData> | null> {
    const url = `${API_BASE}${endpoint}`;
    const defaultErrorMessage = errorMessage || `Failed to load from ${endpoint}`;
    
    loadingStore.set(true);
    errorStore.set(null);
    
    try {
      const response = await fetch(url, {
        headers: {
          'Content-Type': 'application/json',
          ...options.headers,
        },
        ...options,
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({ 
          success: false, 
          error: `HTTP ${response.status}: ${response.statusText}` 
        }));
        const errorMsg = errorData.error || `HTTP ${response.status}`;
        errorStore.set(errorMsg);
        return null;
      }

      const jsonResponse: ApiResponse<TResponseData> = await response.json();
      if (jsonResponse.success && jsonResponse.data) {
        if (dataStore && storeAction) {
          dataStore.update(current => storeAction(jsonResponse.data!, current));
        }
        return jsonResponse;
      } else {
        errorStore.set(jsonResponse.error || defaultErrorMessage);
        return null;
      }
    } catch (err) {
      errorStore.set('Network error');
      console.error(`${defaultErrorMessage}:`, err);
      return null;
    } finally {
      loadingStore.set(false);
    }
  }
}

export const api = new ApiClient();