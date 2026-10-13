import { useEngineStore } from '../../store/useEngineStore';

export function LatencyGauge() {
  const latencyMicros = useEngineStore(state => state.latencyMicros);
  const isConnected = useEngineStore(state => state.isConnected);
  
  // Turn gauge yellow if latency exceeds 5 microseconds, red if over 10
  const colorClass = !isConnected 
    ? "text-gray-600" 
    : latencyMicros > 10 
      ? "text-red-500" 
      : latencyMicros > 5 
        ? "text-yellow-400" 
        : "text-green-500";

  return (
    <div className="p-4 bg-gray-900 border border-gray-800 rounded-lg shadow-lg min-w-[200px]">
      <div className="flex justify-between items-center mb-2">
        <h3 className="text-gray-400 text-sm font-mono uppercase tracking-wider">Engine Latency</h3>
        <span className={`h-2 w-2 rounded-full ${isConnected ? 'bg-green-500 animate-pulse' : 'bg-red-500'}`} />
      </div>
      <div className="flex items-baseline gap-2">
        <span className={`text-5xl font-bold font-mono tabular-nums ${colorClass}`}>
          {latencyMicros.toFixed(2)}
        </span>
        <span className="text-gray-500 font-mono text-lg">μs</span>
      </div>
    </div>
  );
}