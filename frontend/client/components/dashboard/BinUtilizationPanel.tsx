import { useMemo } from 'react';
import { useEngineStore } from '../../store/useEngineStore';
import {
  BarChart,
  Bar,
  Cell,
  XAxis,
  Tooltip,
  ResponsiveContainer,
} from 'recharts';

// Helper to simulate DLMM bin data surrounding the current mid-price
const generateBinData = (bestBid: number, bestAsk: number) => {
  const midPrice = (bestBid + bestAsk) / 2;
  const binStep = 5; // Price tick per bin
  const numBins = 15; // Bins on each side of the active bin
  const bins = [];

  // Generate Bid Bins (Quote token reserves - e.g., USDC)
  for (let i = numBins; i > 0; i--) {
    bins.push({
      binId: 1000 - i, // Arbitrary active bin ID base
      price: midPrice - (i * binStep),
      liquidity: Math.floor(10000 / i + Math.random() * 1000),
      type: 'bid',
    });
  }

  // Active Bin (Contains both Base and Quote tokens)
  bins.push({
    binId: 1000,
    price: midPrice,
    liquidity: 15000 + Math.random() * 2000,
    type: 'active',
  });

  // Generate Ask Bins (Base token reserves - e.g., SOL)
  for (let i = 1; i <= numBins; i++) {
    bins.push({
      binId: 1000 + i,
      price: midPrice + (i * binStep),
      liquidity: Math.floor(10000 / i + Math.random() * 1000),
      type: 'ask',
    });
  }

  return bins;
};

const CustomTooltip = ({ active, payload }: any) => {
  if (active && payload && payload.length) {
    const data = payload[0].payload;
    const typeLabel = data.type === 'active' 
      ? 'Active Bin (Mixed)' 
      : data.type === 'bid' ? 'Quote Reserves (Bids)' : 'Base Reserves (Asks)';
      
    return (
      <div className="bg-gray-900 border border-gray-700 p-3 rounded shadow-xl font-mono text-xs">
        <div className="text-gray-400 mb-1">{typeLabel}</div>
        <div className="text-white">Bin ID: {data.binId}</div>
        <div className="text-white">Price: {data.price.toFixed(2)}</div>
        <div className="text-blue-400 mt-1">
          Liquidity: {data.liquidity.toLocaleString(undefined, { maximumFractionDigits: 0 })}
        </div>
      </div>
    );
  }
  return null;
};

export function BinUtilizationPanel() {
  const snapshot = useEngineStore((state) => state.snapshot);
  const isConnected = useEngineStore((state) => state.isConnected);

  const binData = useMemo(() => {
    if (!snapshot) return [];
    return generateBinData(snapshot.best_bid, snapshot.best_ask);
  }, [snapshot]);

  if (!isConnected || !snapshot) {
    return (
      <div className="flex h-64 w-full items-center justify-center rounded-lg border border-gray-800 bg-gray-900">
        <span className="font-mono text-gray-500 animate-pulse">Awaiting DLMM State...</span>
      </div>
    );
  }

  // Derived metrics for the top header
  const activeBin = binData.find(b => b.type === 'active');
  const feeApy = (45.2 + Math.random() * 2).toFixed(2); // Simulated dynamic APY

  return (
    <div className="flex flex-col w-full h-80 rounded-lg border border-gray-800 bg-gray-900 p-4 shadow-lg">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="font-mono text-sm uppercase tracking-wider text-gray-400">
            Meteora DLMM Utilization
          </h3>
          <div className="font-mono text-xs text-gray-500 mt-1">
            Active Bin ID: <span className="text-white">{activeBin?.binId}</span>
          </div>
        </div>
        <div className="text-right font-mono">
          <div className="text-xs text-gray-400">Dynamic Fee APY</div>
          <div className="text-green-400 font-bold">{feeApy}%</div>
        </div>
      </div>

      <div className="h-full w-full">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart data={binData} margin={{ top: 10, right: 0, left: 0, bottom: 0 }}>
            <XAxis 
              dataKey="price" 
              tickFormatter={(val) => val.toFixed(0)}
              tick={{ fill: '#6b7280', fontSize: 10, fontFamily: 'monospace' }}
              stroke="#374151"
              interval="preserveStartEnd"
            />
            <Tooltip content={<CustomTooltip />} cursor={{ fill: '#1f2937', opacity: 0.4 }} />
            <Bar dataKey="liquidity" isAnimationActive={false}>
              {binData.map((entry, index) => {
                let color = '#3b82f6'; // Default blue
                if (entry.type === 'bid') color = '#22c55e'; // Green for bids
                if (entry.type === 'ask') color = '#ef4444'; // Red for asks
                if (entry.type === 'active') color = '#eab308'; // Yellow for active bin
                
                return <Cell key={`cell-${index}`} fill={color} opacity={entry.type === 'active' ? 1 : 0.6} />;
              })}
            </Bar>
          </BarChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}