import { useEffect, useRef } from 'react';
import type { ReactNode } from 'react';
import { useEngineStore, type EngineTelemetryFrame } from '../store/useEngineStore';

const reconnectDelayMs = 1000;
const configuredTelemetryUrl = import.meta.env.VITE_TELEMETRY_WS_URL;
const telemetryUrl =
  configuredTelemetryUrl || (import.meta.env.DEV ? 'ws://localhost:3000/ws/book' : undefined);

function isTelemetryFrame(value: unknown): value is EngineTelemetryFrame {
  if (typeof value !== 'object' || value === null) return false;

  const frame = value as Record<string, unknown>;
  if (frame.event_type !== 'BOOK_SNAPSHOT') return false;
  if (typeof frame.snapshot !== 'object' || frame.snapshot === null) return false;

  const snapshot = frame.snapshot as Record<string, unknown>;
  return (
    typeof frame.sequence_num === 'number' &&
    Number.isFinite(frame.sequence_num) &&
    typeof frame.latency_micros === 'number' &&
    Number.isFinite(frame.latency_micros) &&
    ['best_bid', 'best_ask', 'bid_volume', 'ask_volume', 'spread', 'timestamp_ns'].every(
      (key) => typeof snapshot[key] === 'number' && Number.isFinite(snapshot[key]),
    )
  );
}

export function WebSocketProvider({ children }: { children: ReactNode }) {
  const wsRef = useRef<WebSocket | null>(null);
  const updateTelemetry = useEngineStore(state => state.updateTelemetry);
  const setConnectionStatus = useEngineStore(state => state.setConnectionStatus);

  useEffect(() => {
    if (!telemetryUrl) {
      setConnectionStatus(false);
      return;
    }

    let reconnectTimeout: ReturnType<typeof setTimeout> | undefined;
    let shouldReconnect = true;
    const connectWs = () => {
      const ws = new WebSocket(telemetryUrl);

      ws.onopen = () => {
        setConnectionStatus(true);
      };

      ws.onmessage = (event) => {
        if (typeof event.data !== 'string') {
          console.error('Ignoring non-text telemetry frame.');
          return;
        }

        let parsed: unknown;
        try {
          parsed = JSON.parse(event.data);
        } catch (error) {
          console.error('Telemetry frame JSON parsing failed.', error);
          return;
        }

        if (!isTelemetryFrame(parsed)) {
          console.error('Ignoring telemetry frame with an invalid shape.');
          return;
        }

        updateTelemetry(parsed);
      };

      ws.onclose = () => {
        setConnectionStatus(false);
        wsRef.current = null;
        if (shouldReconnect) {
          reconnectTimeout = setTimeout(connectWs, reconnectDelayMs);
        }
      };

      wsRef.current = ws;
    };

    connectWs();

    return () => {
      shouldReconnect = false;
      if (reconnectTimeout !== undefined) clearTimeout(reconnectTimeout);
      const ws = wsRef.current;
      wsRef.current = null;
      if (ws) {
        ws.onclose = null;
        ws.close();
      }
    };
  }, [updateTelemetry, setConnectionStatus]);

  return <>{children}</>;
}