export interface PollerConfig {
	intervalMs: number;
	onPoll: () => Promise<void>;
	onError?: (error: any) => void;
	name?: string; // For debugging
}

export class Poller {
	private intervalId: ReturnType<typeof setInterval> | null = null;
	private config: PollerConfig;
	private isRunning = false;

	constructor(config: PollerConfig) {
		this.config = config;
	}

	start(): void {
		// Stop any existing polling first
		this.stop();

		this.isRunning = true;

		// Start polling at specified interval
		this.intervalId = setInterval(async () => {
			if (!this.isRunning) return; // Safety check

			try {
				await this.config.onPoll();
			} catch (error) {
				if (this.config.onError) {
					this.config.onError(error);
				} else {
					console.error(
						`Polling error${this.config.name ? ` in ${this.config.name}` : ''}:`,
						error
					);
				}
			}
		}, this.config.intervalMs);

		// Do an initial poll immediately (optional - can be made configurable)
		this.pollOnce();
	}

	stop(): void {
		this.isRunning = false;

		if (this.intervalId) {
			clearInterval(this.intervalId);
			this.intervalId = null;
		}
	}

	async pollOnce(): Promise<void> {
		try {
			await this.config.onPoll();
		} catch (error) {
			if (this.config.onError) {
				this.config.onError(error);
			} else {
				console.error(`Polling error${this.config.name ? ` in ${this.config.name}` : ''}:`, error);
			}
		}
	}

	updateInterval(newIntervalMs: number): void {
		const wasRunning = this.isRunning;
		this.config.intervalMs = newIntervalMs;

		if (wasRunning) {
			this.stop();
			this.start();
		}
	}

	getIsRunning(): boolean {
		return this.isRunning;
	}
}

// Factory function for common polling patterns
export function createPoller(config: PollerConfig): Poller {
	return new Poller(config);
}
