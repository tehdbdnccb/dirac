use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum OrderSide {
    Bid = 0,
    Ask = 1,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    Market,
    ImmediateOrCancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub order_id: u64,
    pub price: u64,
    pub size: u32,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub trader_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookSnapshotDto {
    pub best_bid: u64,
    pub best_ask: u64,
    pub bid_volume: u32,
    pub ask_volume: u32,
    pub spread: u64,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineTelemetryFrame {
    pub event_type: String,
    pub sequence_num: u64,
    pub snapshot: BookSnapshotDto,
    pub latency_micros: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAck {
    pub order_id: u64,
    pub status: String,
    pub timestamp: u64,
}