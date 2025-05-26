use defi_wallet::{
    core::App,
    defi::{DeFiService, TokenInfo, SwapRequest, DeFiProtocol, ChainType},
};
use anyhow::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Initialize the application
    let app = Arc::new(App::new().await?);
    let defi_service = DeFiService::new(app).await?;
    
    // Define common tokens
    let eth_token = TokenInfo {
        address: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".to_string(),
        symbol: "ETH".to_string(),
        decimals: 18,
        chain_type: ChainType::Ethereum,
    };
    
    let usdc_token = TokenInfo {
        address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
        symbol: "USDC".to_string(),
        decimals: 6,
        chain_type: ChainType::Ethereum,
    };
    
    // Example 1: ETH to USDC on Uniswap V2
    let uniswap_v2_swap = SwapRequest {
        from_token: eth_token.clone(),
        to_token: usdc_token.clone(),
        amount: 1.0,
        slippage: 0.5,
        protocol: DeFiProtocol::UniswapV2,
    };
    
    println!("Getting Uniswap V2 quote...");
    let v2_quote = defi_service.get_swap_quote(&uniswap_v2_swap).await?;
    println!("Uniswap V2 Quote:");
    println!("Expected output: {} USDC", v2_quote.expected_output);
    println!("Price impact: {}%", v2_quote.price_impact);
    println!("Fee: {} ETH", v2_quote.fee);
    
    // Example 2: ETH to USDC on Uniswap V3
    let uniswap_v3_swap = SwapRequest {
        from_token: eth_token.clone(),
        to_token: usdc_token.clone(),
        amount: 1.0,
        slippage: 0.5,
        protocol: DeFiProtocol::UniswapV3,
    };
    
    println!("\nGetting Uniswap V3 quote...");
    let v3_quote = defi_service.get_swap_quote(&uniswap_v3_swap).await?;
    println!("Uniswap V3 Quote:");
    println!("Expected output: {} USDC", v3_quote.expected_output);
    println!("Price impact: {}%", v3_quote.price_impact);
    println!("Fee: {} ETH", v3_quote.fee);
    
    // Example 3: Compare quotes and execute best swap
    let best_quote = if v2_quote.expected_output > v3_quote.expected_output {
        println!("\nUniswap V2 offers better rate");
        v2_quote
    } else {
        println!("\nUniswap V3 offers better rate");
        v3_quote
    };
    
    // Execute the swap with the best quote
    println!("\nExecuting swap...");
    let tx_hash = defi_service.execute_swap(uniswap_v2_swap).await?;
    println!("Swap executed! Transaction hash: {}", tx_hash);
    
    Ok(())
} 