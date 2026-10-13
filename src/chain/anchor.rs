use anchor_lang::InstructionData;
use solana_sdk::pubkey::Pubkey;

// Represents the Anchor instruction struct inside the Aegis clearing smart contract
#[derive(anchor_lang::prelude::AnchorSerialize, anchor_lang::prelude::AnchorDeserialize)]
pub struct ClearPositionsArgs {
    pub order_id: u64,
    pub net_settlement: i64,
}

impl InstructionData for ClearPositionsArgs {
    fn data(&self) -> Vec<u8> {
        let mut d = b"global:clear_positions".to_vec(); // Mock sighash
        d.extend_from_slice(&self.order_id.to_le_bytes());
        d.extend_from_slice(&self.net_settlement.to_le_bytes());
        d
    }
}