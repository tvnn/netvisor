import type { Writable } from "svelte/store";
import { loading, pushError } from "../stores/feedback";

const API_BASE = 'http://localhost:60072/api';

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
    options: RequestInit = {},
    isBackgroundRequest: boolean = false
  ): Promise<ApiResponse<TResponseData> | null> {
    const url = `${API_BASE}${endpoint}`;
    const baseErrorMessage = `Failed to ${options.method || 'load'} from ${endpoint}`;
    
    if (!isBackgroundRequest) loading.set(true);
    
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
        pushError(errorMsg);
        return null;
      }

      const jsonResponse: ApiResponse<TResponseData> = await response.json();
      if (jsonResponse.success) {
        if (dataStore && storeAction) {
          dataStore.update(current => storeAction(jsonResponse.data!, current));
        }
        return jsonResponse;
      } else if (jsonResponse?.error) {
        pushError(`${baseErrorMessage}: ${jsonResponse.error}`);
        return null;
      } else {
        pushError(`${baseErrorMessage}: Unknown error`);
        return null;
      }
    } catch (err) {
      pushError(`${baseErrorMessage}: ${err}`);
      return null;
    } finally {
      if (!isBackgroundRequest) loading.set(false);
    }
  }
}

export const api = new ApiClient();