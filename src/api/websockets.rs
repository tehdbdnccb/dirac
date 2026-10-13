use crate::types::EngineTelemetryFrame;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(tx): State<Arc<broadcast::Sender<EngineTelemetryFrame>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, tx))
}

async fn handle_socket(
    socket: WebSocket,
    tx: Arc<broadcast::Sender<EngineTelemetryFrame>>,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = tx.subscribe();

    info!("Next.js dashboard client connected to WebSocket telemetry feed");

    // Spawn task to forward broadcast telemetry to WS client
    let mut send_task = tokio::spawn(async move {
        while let Ok(frame) = rx.recv().await {
            if let Ok(json_str) = serde_json::to_string(&frame) {
                if sender.send(Message::Text(json_str)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming client control packets
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Close(_) = msg {
                break;
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    info!("Next.js client disconnected from WebSocket stream");
}