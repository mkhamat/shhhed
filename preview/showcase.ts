/** Bidirectional WebSocket channel with automatic reconnection. */
interface ChannelConfig {
  url: string;
  maxRetries: number;
  heartbeatMs: number;
  onMessage: (data: unknown) => void;
}
type ConnectionState = "idle" | "connecting" | "open" | "closed";
class RealtimeChannel<T extends Record<string, unknown>> {
  private ws: WebSocket | null = null;
  private retryCount = 0;
  private heartbeatTimer?: ReturnType<typeof setInterval>;
  readonly #listeners = new Map<string, Set<(payload: T) => void>>();
  constructor(private config: ChannelConfig) {}
  async connect(): Promise<boolean> {
    if (this.ws?.readyState === WebSocket.OPEN) return true;

    return new Promise((resolve, reject) => {
      this.ws = new WebSocket(this.config.url);

      this.ws.onopen = () => {
        this.retryCount = 0;
        this.startHeartbeat();
        resolve(true);
      };

      this.ws.onmessage = (event) => {
        const parsed = JSON.parse(event.data) as { topic: string; payload: T };
        const handlers = this.#listeners.get(parsed.topic);
        handlers?.forEach((fn) => fn(parsed.payload));
      };

      this.ws.onclose = (event) => {
        clearInterval(this.heartbeatTimer);
        if (event.code !== 1000 && this.retryCount < this.config.maxRetries) {
          const delay = Math.min(1000 * 2 ** this.retryCount++, 30_000);
          setTimeout(() => this.connect(), delay);
        }
      };
    });
  }

  private startHeartbeat(): void {
    this.heartbeatTimer = setInterval(() => {
      this.ws?.send(JSON.stringify({ type: "ping", ts: Date.now() }));
    }, this.config.heartbeatMs);
  }
}
