use crate::core::App;
use anyhow::Result;
use async_trait::async_trait;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub public_key: PublicKey,
    pub encrypted_private_key: Vec<u8>,
    pub chain_type: ChainType,
    pub balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainType {
    Ethereum,
    Solana,
    Bitcoin,
}

pub struct WalletService {
    app: Arc<App>,
    wallets: Arc<RwLock<Vec<Wallet>>>,
}

impl WalletService {
    pub async fn new(app: Arc<App>) -> Result<Self> {
        Ok(Self {
            app,
            wallets: Arc::new(RwLock::new(Vec::new())),
        })
    }

    pub async fn create_wallet(&self, chain_type: ChainType) -> Result<Wallet> {
        let keypair = Keypair::generate(&mut rand::thread_rng());
        let config = self.app.get_config().await;
        
        // Encrypt private key
        let encrypted_private_key = self.encrypt_private_key(
            keypair.secret.as_bytes(),
            &config.wallet.encryption_key,
        )?;

        let wallet = Wallet {
            address: self.generate_address(&keypair.public, &chain_type),
            public_key: keypair.public,
            encrypted_private_key,
            chain_type,
            balance: 0.0,
        };

        let mut wallets = self.wallets.write().await;
        wallets.push(wallet.clone());
        
        self.app.update_state(|state| {
            state.active_wallets += 1;
        }).await;

        Ok(wallet)
    }

    pub async fn get_wallet(&self, address: &str) -> Result<Option<Wallet>> {
        let wallets = self.wallets.read().await;
        Ok(wallets.iter().find(|w| w.address == address).cloned())
    }

    pub async fn list_wallets(&self) -> Vec<Wallet> {
        self.wallets.read().await.clone()
    }

    fn encrypt_private_key(&self, private_key: &[u8], encryption_key: &str) -> Result<Vec<u8>> {
        // Implement proper encryption here
        // This is a placeholder implementation
        Ok(private_key.to_vec())
    }

    fn generate_address(&self, public_key: &PublicKey, chain_type: &ChainType) -> String {
        match chain_type {
            ChainType::Ethereum => format!("0x{}", hex::encode(&public_key.to_bytes()[..20])),
            ChainType::Solana => bs58::encode(public_key.to_bytes()).into_string(),
            ChainType::Bitcoin => {
                // Implement Bitcoin address generation
                "btc_address".to_string()
            }
        }
    }

    pub async fn run(&self) -> Result<()> {
        // Implement wallet service main loop
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wallet_creation() {
        let app = Arc::new(App::new().await.unwrap());
        let wallet_service = WalletService::new(app).await.unwrap();
        
        let wallet = wallet_service.create_wallet(ChainType::Ethereum).await.unwrap();
        assert!(!wallet.address.is_empty());
        assert_eq!(wallet.chain_type, ChainType::Ethereum);
    }
} 