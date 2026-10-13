use rust_decimal::Decimal;
use crossbeam::queue::ArrayQueue;
use super::margin::{MarginEngine, PositionRisk};

pub struct LiquidationEvent {
    pub account_id: [u8; 32],
    pub asset_id: u32,
    pub size_to_liquidate: Decimal,
    pub mark_price: Decimal,
}

pub struct LiquidationEngine {
    margin_engine: MarginEngine,
    // Lock-free queue for high-frequency dispatch of liquidation orders to the matching engine
    liquidation_queue: ArrayQueue<LiquidationEvent>,
}

impl LiquidationEngine {
    pub fn new(margin_engine: MarginEngine, queue_capacity: usize) -> Self {
        Self {
            margin_engine,
            liquidation_queue: ArrayQueue::new(queue_capacity),
        }
    }

    /// Continuously sweeps accounts against the latest oracle/mark price.
    /// In an HFT context, this runs pinned to an isolated thread.
    #[inline]
    pub fn evaluate_account(
        &self,
        collateral: Decimal,
        position: &mut PositionRisk,
        mark_price: Decimal,
    ) {
        position.unrealized_pnl = MarginEngine::compute_unrealized_pnl(position, mark_price);

        if !self.margin_engine.is_margin_safe(collateral, position, mark_price) {
            let event = LiquidationEvent {
                account_id: position.account_id,
                asset_id: position.asset_id,
                size_to_liquidate: position.size, // Full liquidation for ultra-low latency; partial logic can be added
                mark_price,
            };

            // Non-blocking dispatch to the order matching engine
            let _ = self.liquidation_queue.push(event);
        }
    }
    
    pub fn poll_liquidations(&self) -> Option<LiquidationEvent> {
        self.liquidation_queue.pop()
    }
}