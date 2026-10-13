import { useMemo } from 'react';
import { useEngineStore } from '../../store/useEngineStore';
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
} from 'recharts';

// Helper to extrapolate visual depth curves from top-of-book telemetry
const generateDepthData = (bestBid: number, bestAsk: number, topBidVol: number, topAskVol: number) => {
  const data = [];
  const tickSpacing = 5;
  const levels = 20;

  // 1. Generate Bids (from deep book up to the best bid)
  let cumulativeBid = topBidVol * 8; // Simulated deep liquidity
  for (let i = levels; i > 0; i--) {
    data.push({
      price: bestBid - (i * tickSpacing),
      bidVolume: cumulativeBid,
    });
    // Volume tapers off as it gets closer to the spread
    cumulativeBid -= (topBidVol * 0.3 * Math.random()); 
  }
  
  // Top of book bid
  data.push({ price: bestBid, bidVolume: Math.max(topBidVol, cumulativeBid) });

  // 2. The Spread (Empty zone)
  const midPrice = (bestBid + bestAsk) / 2;
  data.push({ price: midPrice }); // Nulls for both bid/ask will break the line

  // 3. Top of book ask
  let cumulativeAsk = topAskVol;
  data.push({ price: bestAsk, askVolume: cumulativeAsk });

  // 4. Generate Asks (from best ask out to deep book)
  for (let i = 1; i <= levels; i++) {
    cumulativeAsk += (topAskVol * 0.3 * Math.random());
    data.push({
      price: bestAsk + (i * tickSpacing),
      askVolume: cumulativeAsk,
    });
  }

  return data;
};

export function OrderBookDepthChart() {
  const snapshot = useEngineStore((state) => state.snapshot);
  const isConnected = useEngineStore((state) => state.isConnected);

  const chartData = useMemo(() => {
    if (!snapshot) return [];
    return generateDepthData(
      snapshot.best_bid,
      snapshot.best_ask,
      snapshot.bid_volume,
      snapshot.ask_volume
    );
  }, [snapshot]);

  if (!isConnected || !snapshot) {
    return (
      <div className="flex h-64 w-full items-center justify-center rounded-lg border border-gray-800 bg-gray-900">
        <span className="font-mono text-gray-500 animate-pulse">Awaiting Telemetry Stream...</span>
      </div>
    );
  }

  return (
    <div className="flex h-80 w-full flex-col rounded-lg border border-gray-800 bg-gray-900 p-4 shadow-lg">
      <div className="mb-4 flex items-center justify-between">
        <h3 className="font-mono text-sm uppercase tracking-wider text-gray-400">
          Market Depth (Cumulative)
        </h3>
        <div className="font-mono text-sm text-gray-500">
          Spread: <span className="text-white">{snapshot.spread}</span>
        </div>
      </div>

      <div className="h-full w-full">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={chartData} margin={{ top: 10, right: 0, left: 0, bottom: 0 }}>
            <XAxis 
              dataKey="price" 
              type="number" 
              domain={['dataMin', 'dataMax']} 
              tick={{ fill: '#6b7280', fontSize: 12, fontFamily: 'monospace' }}
              tickFormatter={(val) => val.toLocaleString()}
              stroke="#374151"
            />
            <YAxis hide domain={['dataMin', 'dataMax']} />
            <Tooltip
              contentStyle={{
                backgroundColor: '#111827',
                borderColor: '#374151',
                fontFamily: 'monospace',
                color: '#f3f4f6'
              }}
              itemStyle={{ color: '#f3f4f6' }}
              labelFormatter={(label) => label == null ? 'Price' : `Price: ${label.toLocaleString()}`}
            />
            
            {/* Bid Area - Green */}
            <Area
              type="stepBefore"
              dataKey="bidVolume"
              stroke="#22c55e"
              fillOpacity={0.2}
              fill="#22c55e"
              isAnimationActive={false} // Disable animation for high-frequency updates
              connectNulls={false}
            />
            
            {/* Ask Area - Red */}
            <Area
              type="stepAfter"
              dataKey="askVolume"
              stroke="#ef4444"
              fillOpacity={0.2}
              fill="#ef4444"
              isAnimationActive={false}
              connectNulls={false}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}