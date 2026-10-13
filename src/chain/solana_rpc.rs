use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::env;
use std::sync::Arc;
use tracing::info;

pub struct AegisRpcClient {
    pub client: Arc<RpcClient>,
}

impl AegisRpcClient {
    pub fn new() -> Self {
        let rpc_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        
        info!("Initializing high-speed Solana RPC Client targeting: {}", rpc_url);
        
        let client = RpcClient::new_with_commitment(
            rpc_url,
            CommitmentConfig::confirmed(),
        );

        Self {
            client: Arc::new(client),
        }
    }
}