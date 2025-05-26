use defi_wallet::{
    core::App,
    blockchain::{BlockchainService, ChainType, TransactionStatus, TransactionRequest},
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
    let blockchain_service = BlockchainService::new(app).await?;
    
    // Example: Send a transaction
    let tx_request = TransactionRequest {
        from: "0x123...".to_string(),
        to: "0x456...".to_string(),
        amount: 0.1,
        chain_type: ChainType::Ethereum,
        gas_limit: Some(21000),
        gas_price: Some(20),
    };
    
    println!("Sending transaction...");
    let tx_hash = blockchain_service.send_transaction(tx_request).await?;
    println!("Transaction sent! Hash: {}", tx_hash);
    
    // Monitor transaction status
    println!("\nMonitoring transaction status...");
    let mut attempts = 0;
    let max_attempts = 10;
    
    while attempts < max_attempts {
        let status = blockchain_service.get_transaction_status(&tx_hash, ChainType::Ethereum).await?;
        
        match status {
            TransactionStatus::Pending => {
                println!("Transaction is pending... (Attempt {}/{})", attempts + 1, max_attempts);
            }
            TransactionStatus::Confirmed => {
                println!("Transaction confirmed!");
                break;
            }
            TransactionStatus::Failed => {
                println!("Transaction failed!");
                break;
            }
        }
        
        attempts += 1;
        sleep(Duration::from_secs(3)).await;
    }
    
    // Example: Check balance
    let address = "0x123...".to_string();
    println!("\nChecking balance for address: {}", address);
    let balance = blockchain_service.get_balance(&address, ChainType::Ethereum).await?;
    println!("Balance: {} ETH", balance);
    
    Ok(())
} 