use defi_wallet::{
    core::App,
    wallet::{WalletService, ChainType},
};
use anyhow::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Initialize the application
    let app = Arc::new(App::new().await?);
    
    // Create wallet service
    let wallet_service = WalletService::new(app.clone()).await?;
    
    // Create Ethereum wallet
    let eth_wallet = wallet_service.create_wallet(ChainType::Ethereum).await?;
    println!("Created Ethereum wallet: {}", eth_wallet.address);
    
    // Create Solana wallet
    let sol_wallet = wallet_service.create_wallet(ChainType::Solana).await?;
    println!("Created Solana wallet: {}", sol_wallet.address);
    
    // List all wallets
    let wallets = wallet_service.list_wallets().await;
    println!("\nAll wallets:");
    for wallet in wallets {
        println!("- {} ({:?})", wallet.address, wallet.chain_type);
    }
    
    // Get specific wallet
    if let Some(wallet) = wallet_service.get_wallet(&eth_wallet.address).await? {
        println!("\nRetrieved wallet: {}", wallet.address);
    }
    
    Ok(())
} 