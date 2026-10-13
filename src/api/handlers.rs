use crate::engine::memory::{FfiOrder, SpscRingBuffer};
use crate::types::{OrderAck, OrderRequest, OrderSide};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub ring_buffer: Arc<Mutex<*mut SpscRingBuffer>>,
    pub sequence_counter: AtomicU64,
}

// Safety: The raw pointer points to mmap memory designed for multi-thread atomic access
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "status": "online",
        "engine": "active",
        "timestamp": crate::api::telemetry::current_time_nanos()
    })))
}

pub async fn submit_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OrderRequest>,
) -> impl IntoResponse {
    let ring_ptr = *state.ring_buffer.lock().unwrap();
    let ring = unsafe { &mut *ring_ptr };

    let head = ring.head.load(Ordering::Acquire);
    let next_head = (head + 1) % crate::engine::memory::RING_BUFFER_SIZE;

    // Zero-copy direct slot write into shared memory
    let ffi_order = FfiOrder {
        order_id: payload.order_id,
        price: payload.price,
        size: payload.size,
        side: match payload.side {
            OrderSide::Bid => 0,
            OrderSide::Ask => 1,
        },
        _padding: [0; 3],
    };

    ring.buffer[head] = ffi_order;
    ring.head.store(next_head, Ordering::Release);

    let ack = OrderAck {
        order_id: payload.order_id,
        status: "QUEUED_SPSC".to_string(),
        timestamp: crate::api::telemetry::current_time_nanos(),
    };

    (StatusCode::ACCEPTED, Json(ack))
}