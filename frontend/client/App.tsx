import { BinUtilizationPanel } from './components/dashboard/BinUtilizationPanel';
import { LatencyGauge } from './components/dashboard/LatencyGauge';
import { OrderBookDepthChart } from './components/dashboard/OrderBookDepthChart';
import { useEngineStore } from './store/useEngineStore';
import { WebSocketProvider } from './ws/WebSocketProvider';

function Dashboard() {
  const isConnected = useEngineStore((state) => state.isConnected);
  const sequenceNum = useEngineStore((state) => state.sequenceNum);

  return (
    <main className="dashboard-shell">
      <header className="dashboard-header">
        <a className="brand" href="/" aria-label="Meteora dashboard home">
          <span className="brand-mark" aria-hidden="true">M</span>
          <span className="brand-name">METEORA<span className="brand-separator">/</span>ENGINE</span>
        </a>
        <div className="stream-status" role="status" aria-live="polite">
          <span className={`status-dot${isConnected ? ' is-connected' : ''}`} />
          <span>{isConnected ? 'LIVE TELEMETRY' : 'STREAM DISCONNECTED'}</span>
          {isConnected && <span className="sequence-number">SEQ {sequenceNum}</span>}
        </div>
      </header>

      <section className="dashboard-intro" aria-labelledby="dashboard-title">
        <p className="eyebrow">SOLANA · MARKET INFRASTRUCTURE</p>
        <h1 id="dashboard-title">Engine dashboard</h1>
        <p className="dashboard-description">
          Live order book depth, engine latency, and Meteora DLMM bin utilization.
        </p>
      </section>

      <section className="dashboard-grid" aria-label="Engine telemetry">
        <div className="latency-panel">
          <LatencyGauge />
        </div>
        <div className="chart-panel">
          <OrderBookDepthChart />
        </div>
        <div className="chart-panel">
          <BinUtilizationPanel />
        </div>
      </section>

      {!isConnected && (
        <p className="connection-note">
          Connect the telemetry service to stream live market data. The dashboard is ready
          and will reconnect automatically when the service is available.
        </p>
      )}

      <footer className="dashboard-footer">
        <span>AEGIS QUANTITATIVE ENGINE</span>
        <span>TELEMETRY {isConnected ? 'ACTIVE' : 'OFFLINE'}</span>
      </footer>
    </main>
  );
}

export function App() {
  return (
    <WebSocketProvider>
      <Dashboard />
    </WebSocketProvider>
  );
}
