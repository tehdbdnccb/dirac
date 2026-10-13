use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::str::FromStr;

pub struct MeteoraRouter {
    dlmm_program_id: Pubkey,
}

impl MeteoraRouter {
    pub fn new() -> Self {
        Self {
            dlmm_program_id: Pubkey::from_str("LBUZKhRxPF3XUpBCjp4kVf7L9uA34nQhC8j8H7Nf2v").unwrap(),
        }
    }

    pub fn build_dlmm_swap_instruction(
        &self,
        pool_address: Pubkey,
        user_wallet: Pubkey,
        amount_in: u64,
        min_amount_out: u64,
        active_bin_array: Pubkey,
    ) -> Instruction {
        // DLMM Swap instruction sighash (Anchor 8-byte discriminator)
        // sha256("global:swap")[..8]
        let mut data = vec![248, 198, 158, 145, 225, 117, 135, 200];
        data.extend_from_slice(&amount_in.to_le_bytes());
        data.extend_from_slice(&min_amount_out.to_le_bytes());

        Instruction {
            program_id: self.dlmm_program_id,
            accounts: vec![
                AccountMeta::new_readonly(pool_address, false),
                AccountMeta::new(user_wallet, true),
                AccountMeta::new(active_bin_array, false),
                // Additional account metas required by Meteora (vaults, token program, etc.) would be appended here
            ],
            data,
        }
    }
}