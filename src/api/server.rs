use crate::api::{handlers, websockets};
use crate::engine::memory::MemoryManager;
use crate::types::{BookSnapshotDto, EngineTelemetryFrame};
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub async fn run_server(addr: SocketAddr, ipc_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut mem_mgr = MemoryManager::new(ipc_path);
    let ring_ptr = mem_mgr.ring_buffer() as *mut _;

    let (tx, _) = broadcast::channel::<EngineTelemetryFrame>(10000);
    let tx_arc = Arc::new(tx);

    let app_state = Arc::new(handlers::AppState {
        ring_buffer: Arc::new(Mutex::new(ring_ptr)),
        sequence_counter: AtomicU64::new(0),
    });

    // Background task polling order book updates and broadcasting to WebSockets
    let tx_clone = tx_arc.clone();
    tokio::spawn(async move {
        let mut seq = 0u64;
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(50));
        
        loop {
            interval.tick().await;
            seq += 1;

            let frame = EngineTelemetryFrame {
                event_type: "BOOK_SNAPSHOT".to_string(),
                sequence_num: seq,
                snapshot: BookSnapshotDto {
                    best_bid: 100_000,
                    best_ask: 100_050,
                    bid_volume: 500,
                    ask_volume: 450,
                    spread: 50,
                    timestamp_ns: crate::api::telemetry::current_time_nanos(),
                },
                latency_micros: 1.25,
            };

            let _ = tx_clone.send(frame);
        }
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/api/order", post(handlers::submit_order))
        .route("/ws/book", get(websockets::ws_handler))
        .layer(cors)
        .with_state(app_state)
        .with_state(tx_arc);

    info!("Aegis Telemetry API listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}