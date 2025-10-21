export interface SSEConfig<T> {
	url: string;
	onMessage: (data: T) => void;
	onError?: (error: Event) => void;
	onOpen?: () => void;
}

export class SSEClient<T> {
	private eventSource: EventSource | null = null;
	private config: SSEConfig<T>;
	private reconnectAttempts = 0;
	private maxReconnectAttempts = 5;
	private reconnectDelay = 1000; // Start with 1 second

	constructor(config: SSEConfig<T>) {
		this.config = config;
	}

	connect(): void {
		if (this.eventSource) {
			this.disconnect();
		}

		this.eventSource = new EventSource(this.config.url);

		this.eventSource.onopen = () => {
			console.log('SSE connection opened');
			this.reconnectAttempts = 0;
			this.reconnectDelay = 1000;
			this.config.onOpen?.();
		};

		this.eventSource.onmessage = (event) => {
			try {
				const data = JSON.parse(event.data) as T;
				this.config.onMessage(data);
			} catch (error) {
				console.error('Failed to parse SSE message:', error);
			}
		};

		this.eventSource.onerror = (error) => {
			console.error('SSE error:', error);
			this.config.onError?.(error);

			// Attempt to reconnect with exponential backoff
			if (this.reconnectAttempts < this.maxReconnectAttempts) {
				this.reconnectAttempts++;
				const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);
				console.log(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`);

				setTimeout(() => {
					this.connect();
				}, delay);
			} else {
				console.error('Max reconnection attempts reached');
				this.disconnect();
			}
		};
	}

	disconnect(): void {
		if (this.eventSource) {
			this.eventSource.close();
			this.eventSource = null;
		}
	}

	isConnected(): boolean {
		return this.eventSource?.readyState === EventSource.OPEN;
	}
}
