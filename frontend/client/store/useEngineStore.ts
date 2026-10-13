import { create } from 'zustand';

export interface BookSnapshot {
  best_bid: number;
  best_ask: number;
  bid_volume: number;
  ask_volume: number;
  spread: number;
  timestamp_ns: number;
}

export interface EngineTelemetryFrame {
  event_type: string;
  sequence_num: number;
  snapshot: BookSnapshot;
  latency_micros: number;
}

interface EngineState {
  snapshot: BookSnapshot | null;
  latencyMicros: number;
  sequenceNum: number;
  isConnected: boolean;
  setConnectionStatus: (status: boolean) => void;
  updateTelemetry: (frame: EngineTelemetryFrame) => void;
}

export const useEngineStore = create<EngineState>((set) => ({
  snapshot: null,
  latencyMicros: 0,
  sequenceNum: 0,
  isConnected: false,
  setConnectionStatus: (status) => set({ isConnected: status }),
  updateTelemetry: (frame) => set({
    snapshot: frame.snapshot,
    latencyMicros: frame.latency_micros,
    sequenceNum: frame.sequence_num,
  })
}));