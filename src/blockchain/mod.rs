use crate::core::App;
use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    providers::{Http, Provider, Ws},
    types::{Address, Transaction, U256},
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction as SolanaTransaction,
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct BlockchainService {
    app: Arc<App>,
    ethereum_provider: Arc<RwLock<Provider<Http>>>,
    solana_provider: Arc<RwLock<solana_client::rpc_client::RpcClient>>,
}

#[derive(Debug, Clone)]
pub struct TransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub chain_type: ChainType,
    pub gas_limit: Option<u64>,
    pub gas_price: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum ChainType {
    Ethereum,
    Solana,
}

impl BlockchainService {
    pub async fn new(app: Arc<App>) -> Result<Self> {
        let config = app.get_config().await;
        
        let ethereum_provider = Provider::<Http>::try_from(&config.blockchain.ethereum_rpc_url)?;
        let solana_provider = solana_client::rpc_client::RpcClient::new(config.blockchain.solana_rpc_url);

        Ok(Self {
            app,
            ethereum_provider: Arc::new(RwLock::new(ethereum_provider)),
            solana_provider: Arc::new(RwLock::new(solana_provider)),
        })
    }

    pub async fn get_balance(&self, address: &str, chain_type: ChainType) -> Result<f64> {
        match chain_type {
            ChainType::Ethereum => {
                let provider = self.ethereum_provider.read().await;
                let address = address.parse::<Address>()?;
                let balance = provider.get_balance(address, None).await?;
                Ok(ethers::utils::format_units(balance, "ether")?.parse::<f64>()?)
            }
            ChainType::Solana => {
                let provider = self.solana_provider.read().await;
                let pubkey = address.parse::<Pubkey>()?;
                let balance = provider.get_balance(&pubkey)?;
                Ok(balance as f64 / 1e9) // Convert lamports to SOL
            }
        }
    }

    pub async fn send_transaction(&self, request: TransactionRequest) -> Result<String> {
        match request.chain_type {
            ChainType::Ethereum => {
                let provider = self.ethereum_provider.read().await;
                let from = request.from.parse::<Address>()?;
                let to = request.to.parse::<Address>()?;
                let amount = ethers::utils::parse_units(request.amount.to_string(), "ether")?;

                let tx = Transaction::builder()
                    .from(from)
                    .to(to)
                    .value(amount)
                    .gas(request.gas_limit.unwrap_or(21000))
                    .gas_price(request.gas_price.unwrap_or(1))
                    .build();

                let tx_hash = provider.send_transaction(tx, None).await?;
                Ok(format!("0x{:x}", tx_hash))
            }
            ChainType::Solana => {
                let provider = self.solana_provider.read().await;
                let from = request.from.parse::<Pubkey>()?;
                let to = request.to.parse::<Pubkey>()?;
                let amount = (request.amount * 1e9) as u64; // Convert SOL to lamports

                let recent_blockhash = provider.get_latest_blockhash()?;
                let transaction = SolanaTransaction::new_signed_with_payer(
                    &[solana_sdk::system_instruction::transfer(
                        &from,
                        &to,
                        amount,
                    )],
                    Some(&from),
                    &[&Keypair::new()], // This should be the actual keypair
                    recent_blockhash,
                );

                let signature = provider.send_and_confirm_transaction(&transaction)?;
                Ok(signature.to_string())
            }
        }
    }

    pub async fn get_transaction_status(&self, tx_hash: &str, chain_type: ChainType) -> Result<TransactionStatus> {
        match chain_type {
            ChainType::Ethereum => {
                let provider = self.ethereum_provider.read().await;
                let tx_hash = tx_hash.parse::<ethers::types::H256>()?;
                let receipt = provider.get_transaction_receipt(tx_hash).await?;
                
                Ok(match receipt {
                    Some(receipt) => {
                        if receipt.status.unwrap_or(U256::zero()) == U256::one() {
                            TransactionStatus::Confirmed
                        } else {
                            TransactionStatus::Failed
                        }
                    }
                    None => TransactionStatus::Pending,
                })
            }
            ChainType::Solana => {
                let provider = self.solana_provider.read().await;
                let signature = tx_hash.parse::<solana_sdk::signature::Signature>()?;
                let status = provider.get_signature_status(&signature)?;
                
                Ok(match status {
                    Some(Ok(_)) => TransactionStatus::Confirmed,
                    Some(Err(_)) => TransactionStatus::Failed,
                    None => TransactionStatus::Pending,
                })
            }
        }
    }

    pub async fn run(&self) -> Result<()> {
        // Implement blockchain service main loop
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blockchain_service_initialization() {
        let app = Arc::new(App::new().await.unwrap());
        let service = BlockchainService::new(app).await.unwrap();
        assert!(service.ethereum_provider.read().await.as_ref().is_some());
    }
} 