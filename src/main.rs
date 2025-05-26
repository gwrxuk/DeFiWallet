mod core;
mod network;
mod wallet;
mod blockchain;
mod defi;
mod utils;

use anyhow::Result;
use log::{info, error};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting DeFi Wallet P2P Application...");

    // Initialize the application components
    let app = Arc::new(core::App::new().await?);
    
    // Start the P2P network
    let network = network::Network::new(app.clone()).await?;
    
    // Start the wallet service
    let wallet_service = wallet::WalletService::new(app.clone()).await?;
    
    // Start the blockchain service
    let blockchain_service = blockchain::BlockchainService::new(app.clone()).await?;
    
    // Start the DeFi service
    let defi_service = defi::DeFiService::new(app.clone()).await?;

    // Keep the application running
    tokio::select! {
        _ = network.run() => {
            error!("Network service stopped unexpectedly");
        }
        _ = wallet_service.run() => {
            error!("Wallet service stopped unexpectedly");
        }
        _ = blockchain_service.run() => {
            error!("Blockchain service stopped unexpectedly");
        }
        _ = defi_service.run() => {
            error!("DeFi service stopped unexpectedly");
        }
    }

    Ok(())
}
