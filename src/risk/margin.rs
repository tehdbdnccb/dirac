use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PositionRisk {
    pub account_id: [u8; 32],
    pub asset_id: u32,
    pub size: Decimal,           // Positive for Long, Negative for Short
    pub average_entry: Decimal,
    pub unrealized_pnl: Decimal,
}

pub struct MarginEngine {
    /// Maintenance margin fraction (e.g., 0.05 for 5%)
    pub maintenance_margin_rate: Decimal,
    /// Initial margin fraction (e.g., 0.10 for 10%)
    pub initial_margin_rate: Decimal,
}

impl MarginEngine {
    pub fn new(initial_margin_rate: Decimal, maintenance_margin_rate: Decimal) -> Self {
        Self {
            initial_margin_rate,
            maintenance_margin_rate,
        }
    }

    #[inline]
    pub fn calculate_required_margin(&self, size: Decimal, mark_price: Decimal, is_initial: bool) -> Decimal {
        let notional_value = (size * mark_price).abs();
        let rate = if is_initial {
            self.initial_margin_rate
        } else {
            self.maintenance_margin_rate
        };
        notional_value * rate
    }

    /// Evaluates if an account passes the maintenance margin requirement
    pub fn is_margin_safe(&self, collateral: Decimal, position: &PositionRisk, mark_price: Decimal) -> bool {
        let maintenance_requirement = self.calculate_required_margin(position.size, mark_price, false);
        let account_value = collateral + position.unrealized_pnl;
        
        account_value >= maintenance_requirement
    }

    pub fn compute_unrealized_pnl(position: &PositionRisk, mark_price: Decimal) -> Decimal {
        if position.size.is_sign_positive() {
            (mark_price - position.average_entry) * position.size
        } else {
            (position.average_entry - mark_price) * position.size.abs()
        }
    }
}