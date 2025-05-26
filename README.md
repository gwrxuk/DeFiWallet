# DeFi Wallet P2P Application

A P2P application for building and integrating crypto wallets for DeFi and blockchain applications. This application provides a robust foundation for managing multiple blockchain wallets and interacting with various DeFi protocols.

## Features

- Multi-chain wallet support (Ethereum, Solana)
- P2P networking for wallet synchronization
- DeFi protocol integration (Uniswap V2/V3, SushiSwap, Curve)
- Secure key management and encryption
- Real-time transaction monitoring
- Cross-chain token swaps
- Configurable network settings

## Prerequisites

- Rust 1.70 or later
- Cargo package manager
- PostgreSQL (for wallet storage)
- Access to Ethereum and Solana RPC nodes

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/defi-wallet.git
cd defi-wallet
```

2. Configure the application:
   - Copy `config/default.toml` to `config/local.toml`
   - Update the configuration values in `config/local.toml`

3. Build the project:
```bash
cargo build --release
```

## Configuration

The application is configured through TOML files in the `config` directory:

- `default.toml`: Default configuration values
- `local.toml`: Local overrides (not tracked in git)

Key configuration sections:
- Network settings (P2P, RPC endpoints)
- Wallet storage and encryption
- Supported DeFi protocols
- Chain-specific settings

## Usage

1. Start the application:
```bash
cargo run --release
```

2. Create a new wallet:
```rust
let wallet_service = WalletService::new(app).await?;
let wallet = wallet_service.create_wallet(ChainType::Ethereum).await?;
```

3. Perform a token swap:
```rust
let defi_service = DeFiService::new(app).await?;
let swap_request = SwapRequest {
    from_token: token_info,
    to_token: target_token_info,
    amount: 1.0,
    slippage: 0.5,
    protocol: DeFiProtocol::UniswapV2,
};
let quote = defi_service.get_swap_quote(&swap_request).await?;
let tx_hash = defi_service.execute_swap(swap_request).await?;
```

## Tutorial

### 1. Setting Up Your Development Environment

First, ensure you have all the prerequisites installed:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL
# For Ubuntu/Debian:
sudo apt-get install postgresql
# For macOS:
brew install postgresql
```

### 2. Creating Your First Wallet

Here's a complete example of creating and managing an Ethereum wallet:

```rust
use defi_wallet::wallet::{WalletService, ChainType};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the application
    let app = Arc::new(App::new().await?);
    
    // Create wallet service
    let wallet_service = WalletService::new(app.clone()).await?;
    
    // Create a new Ethereum wallet
    let wallet = wallet_service.create_wallet(ChainType::Ethereum).await?;
    println!("Created wallet with address: {}", wallet.address);
    
    // List all wallets
    let wallets = wallet_service.list_wallets().await;
    println!("Total wallets: {}", wallets.len());
    
    Ok(())
}
```

### 3. Performing a Token Swap

Here's how to perform a token swap using Uniswap V2:

```rust
use defi_wallet::defi::{DeFiService, TokenInfo, SwapRequest, DeFiProtocol, ChainType};

async fn perform_swap() -> Result<()> {
    let app = Arc::new(App::new().await?);
    let defi_service = DeFiService::new(app).await?;
    
    // Define tokens
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
    
    // Create swap request
    let swap_request = SwapRequest {
        from_token: eth_token,
        to_token: usdc_token,
        amount: 1.0, // 1 ETH
        slippage: 0.5, // 0.5%
        protocol: DeFiProtocol::UniswapV2,
    };
    
    // Get quote
    let quote = defi_service.get_swap_quote(&swap_request).await?;
    println!("Expected output: {} USDC", quote.expected_output);
    println!("Price impact: {}%", quote.price_impact);
    
    // Execute swap
    let tx_hash = defi_service.execute_swap(swap_request).await?;
    println!("Swap transaction hash: {}", tx_hash);
    
    Ok(())
}
```

### 4. Setting Up P2P Networking

To enable P2P networking and wallet synchronization:

```rust
use defi_wallet::network::Network;

async fn setup_network() -> Result<()> {
    let app = Arc::new(App::new().await?);
    
    // Initialize network
    let mut network = Network::new(app.clone()).await?;
    
    // Start network service
    tokio::spawn(async move {
        if let Err(e) = network.run().await {
            eprintln!("Network error: {}", e);
        }
    });
    
    Ok(())
}
```

### 5. Monitoring Transactions

Here's how to monitor transaction status:

```rust
use defi_wallet::blockchain::{BlockchainService, ChainType, TransactionStatus};

async fn monitor_transaction(tx_hash: &str) -> Result<()> {
    let app = Arc::new(App::new().await?);
    let blockchain_service = BlockchainService::new(app).await?;
    
    // Check transaction status
    let status = blockchain_service.get_transaction_status(tx_hash, ChainType::Ethereum).await?;
    
    match status {
        TransactionStatus::Pending => println!("Transaction is pending"),
        TransactionStatus::Confirmed => println!("Transaction confirmed!"),
        TransactionStatus::Failed => println!("Transaction failed"),
    }
    
    Ok(())
}
```

### 6. Best Practices

1. **Security**:
   - Always use environment variables for sensitive data
   - Never commit private keys or API keys to version control
   - Use hardware wallets for large transactions

2. **Error Handling**:
   - Always handle errors appropriately
   - Implement retry mechanisms for network operations
   - Log important events and errors

3. **Testing**:
   - Write unit tests for critical components
   - Use test networks (e.g., Ethereum Goerli, Solana Devnet)
   - Implement integration tests for DeFi interactions

4. **Performance**:
   - Use connection pooling for database operations
   - Implement caching for frequently accessed data
   - Monitor memory usage and implement proper cleanup

## Examples

The project includes several example applications in the `examples` directory that demonstrate different features:

1. **Basic Wallet Management** (`examples/basic_wallet.rs`):
   - Creating wallets for different chains
   - Listing and retrieving wallet information
   ```bash
   cargo run --example basic_wallet
   ```

2. **Token Swaps** (`examples/token_swap.rs`):
   - Comparing quotes from different DEXes
   - Executing token swaps
   - Handling slippage and fees
   ```bash
   cargo run --example token_swap
   ```

3. **P2P Networking** (`examples/p2p_network.rs`):
   - Setting up P2P network
   - Broadcasting wallet updates
   - Handling peer discovery
   ```bash
   cargo run --example p2p_network
   ```

4. **Transaction Monitoring** (`examples/transaction_monitor.rs`):
   - Sending transactions
   - Monitoring transaction status
   - Checking wallet balances
   ```bash
   cargo run --example transaction_monitor
   ```

To run any example:
1. Make sure you have configured the application properly
2. Use the `cargo run --example <example_name>` command
3. Check the console output for results

Note: Some examples require proper configuration of RPC endpoints and API keys in your `config/local.toml` file.

## Architecture

The application is built with a modular architecture:

- `core`: Application core and configuration
- `wallet`: Wallet management and operations
- `network`: P2P networking and synchronization
- `blockchain`: Blockchain interactions
- `defi`: DeFi protocol integrations

## Security

- Private keys are encrypted at rest
- Secure P2P communication using libp2p
- Transaction signing with hardware wallet support
- Rate limiting and anti-spam measures

## Testing

Run the test suite:
```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- libp2p for P2P networking
- ethers-rs for Ethereum integration
- solana-sdk for Solana integration
- All DeFi protocol teams for their excellent documentation 