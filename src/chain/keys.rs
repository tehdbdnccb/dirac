use solana_sdk::signature::{read_keypair_file, Keypair};
use std::env;

pub struct KeyManager {
    pub clearing_wallet: Keypair,
}

impl KeyManager {
    pub fn new() -> Self {
        // Load the highly-permissioned clearing wallet used for MEV tips and settlement
        let key_path = env::var("SOLANA_WALLET_PATH")
            .unwrap_or_else(|_| "~/.config/solana/clearing_key.json".to_string());
        
        let clearing_wallet = read_keypair_file(&key_path)
            .expect("Failed to load Solana clearing keypair from file");

        Self { clearing_wallet }
    }
}