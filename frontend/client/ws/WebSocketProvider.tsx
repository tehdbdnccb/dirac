import { useEffect, useRef } from 'react';
import { useEngineStore } from '../store/useEngineStore';

export function WebSocketProvider({ children }: { children: React.ReactNode }) {
  const wsRef = useRef<WebSocket | null>(null);
  
  const updateTelemetry = useEngineStore(state => state.updateTelemetry);
  const setConnectionStatus = useEngineStore(state => state.setConnectionStatus);

  useEffect(() => {
    let reconnectTimeout: NodeJS.Timeout;

    const connectWs = () => {
      // Point this to your Axum server port
      const ws = new WebSocket('ws://localhost:3000/ws/book'); 
      
      ws.onopen = () => {
        console.log('Connected to Aegis Telemetry Stream');
        setConnectionStatus(true);
      };
      
      ws.onmessage = (event) => {
        try {
          const frame = JSON.parse(event.data);
          if (frame.event_type === 'BOOK_SNAPSHOT') {
            updateTelemetry(frame);
          }
        } catch (e) {
          console.error('Frame parsing error', e);
        }
      };

      ws.onclose = () => {
        setConnectionStatus(false);
        console.log('WS disconnected. Reconnecting in 1s...');
        reconnectTimeout = setTimeout(connectWs, 1000);
      };

      wsRef.current = ws;
    };

    connectWs();

    return () => {
      clearTimeout(reconnectTimeout);
      wsRef.current?.close();
    };
  }, [updateTelemetry, setConnectionStatus]);

  return <>{children}</>;
}