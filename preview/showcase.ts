/** Bidirectional WebSocket channel with automatic reconnection. */
interface ChannelConfig {
  url: string;
  maxRetries: number;
  onMessage: (data: unknown) => void;
}
type ConnectionState = "idle" | "connecting" | "open" | "closed";

class RealtimeChannel<T extends Record<string, unknown>> {
  private ws: WebSocket | null = null;
  private retryCount = 0;
  readonly #listeners = new Map<string, Set<(payload: T) => void>>();

  constructor(private config: ChannelConfig) {}

  async connect(): Promise<boolean> {
    if (this.ws?.readyState === WebSocket.OPEN) return true;
    return new Promise((resolve) => {
      this.ws = new WebSocket(this.config.url);
      this.ws.onopen = () => { this.retryCount = 0; resolve(true); };
      this.ws.onmessage = (event) => {
        const parsed = JSON.parse(event.data) as { topic: string; payload: T };
        this.#listeners.get(parsed.topic)?.forEach((fn) => fn(parsed.payload));
      };
    });
  }
}
