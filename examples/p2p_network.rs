use defi_wallet::{
    core::App,
    network::{Network, NetworkMessage},
};
use anyhow::Result;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Initialize the application
    let app = Arc::new(App::new().await?);
    
    // Create and start the network
    let mut network = Network::new(app.clone()).await?;
    
    println!("Starting P2P network...");
    
    // Spawn network service
    let network_handle = tokio::spawn(async move {
        if let Err(e) = network.run().await {
            eprintln!("Network error: {}", e);
        }
    });
    
    // Wait for network to initialize
    sleep(Duration::from_secs(2)).await;
    
    // Example: Broadcast wallet update
    let wallet_update = NetworkMessage::WalletUpdate {
        address: "0x123...".to_string(),
        balance: 1.5,
    };
    
    println!("Broadcasting wallet update...");
    // Note: In a real application, you would use the network's broadcast method
    // network.broadcast(wallet_update).await?;
    
    // Example: Send transaction
    let transaction = NetworkMessage::Transaction {
        from: "0x123...".to_string(),
        to: "0x456...".to_string(),
        amount: 0.1,
        chain_type: "Ethereum".to_string(),
    };
    
    println!("Broadcasting transaction...");
    // network.broadcast(transaction).await?;
    
    // Keep the application running for a while
    println!("Network running. Press Ctrl+C to exit.");
    sleep(Duration::from_secs(30)).await;
    
    // Clean shutdown
    network_handle.abort();
    
    Ok(())
} 