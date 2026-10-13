use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SettlementTransfer {
    pub from_account: [u8; 32],
    pub to_account: [u8; 32],
    pub amount: Decimal,
    pub asset_id: u32,
    pub timestamp: u64,
}

/// Autonomous Clearing protocol 
/// Batches state updates and determines net obligations for zero-knowledge proofs or cross-chain settlement.
pub struct ClearingNetwork {
    /// Ledger mapping Account ID -> Asset ID -> Balance
    balances: HashMap<[u8; 32], HashMap<u32, Decimal>>,
    /// Pending transfers awaiting block finalization or ZK proof generation
    pending_settlements: Vec<SettlementTransfer>,
}

impl ClearingNetwork {
    pub fn new() -> Self {
        Self {
            balances: HashMap::with_capacity(100_000),
            pending_settlements: Vec::with_capacity(10_000),
        }
    }

    /// Nets a matched trade between taker and maker
    #[inline]
    pub fn clear_trade(
        &mut self,
        buyer_id: [u8; 32],
        seller_id: [u8; 32],
        base_asset: u32,
        quote_asset: u32,
        price: Decimal,
        size: Decimal,
        timestamp: u64,
    ) {
        let notional = price * size;

        // Add quote asset transfer (Buyer -> Seller)
        self.pending_settlements.push(SettlementTransfer {
            from_account: buyer_id,
            to_account: seller_id,
            amount: notional,
            asset_id: quote_asset,
            timestamp,
        });

        // Add base asset transfer (Seller -> Buyer)
        self.pending_settlements.push(SettlementTransfer {
            from_account: seller_id,
            to_account: buyer_id,
            amount: size,
            asset_id: base_asset,
            timestamp,
        });
    }

    /// Executes the netted settlements and updates local ledger balances.
    /// In a ZK architecture (e.g., Midnight Compact or Noir), this state transition is hashed.
    pub fn finalize_settlement_batch(&mut self) -> Result<(), &'static str> {
        for transfer in self.pending_settlements.drain(..) {
            // Deduct from sender
            let sender_balance = self.balances
                .entry(transfer.from_account)
                .or_default()
                .entry(transfer.asset_id)
                .or_insert(Decimal::ZERO);
                
            if *sender_balance < transfer.amount {
                // In a true margin setup, this should be prevented by the Risk Engine prior to matching.
                return Err("Insufficient collateral for settlement");
            }
            *sender_balance -= transfer.amount;

            // Credit to receiver
            let receiver_balance = self.balances
                .entry(transfer.to_account)
                .or_default()
                .entry(transfer.asset_id)
                .or_insert(Decimal::ZERO);
            *receiver_balance += transfer.amount;
        }

        Ok(())
    }
}