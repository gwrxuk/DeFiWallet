use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use config::{Config, ConfigError, File};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub network: NetworkConfig,
    pub wallet: WalletConfig,
    pub blockchain: BlockchainConfig,
    pub defi: DeFiConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub listen_addr: String,
    pub bootstrap_peers: Vec<String>,
    pub max_peers: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletConfig {
    pub storage_path: String,
    pub encryption_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockchainConfig {
    pub ethereum_rpc_url: String,
    pub solana_rpc_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeFiConfig {
    pub supported_protocols: Vec<String>,
    pub default_slippage: f64,
}

pub struct App {
    config: Arc<RwLock<AppConfig>>,
    state: Arc<RwLock<AppState>>,
}

#[derive(Default)]
pub struct AppState {
    pub connected_peers: usize,
    pub active_wallets: usize,
    pub pending_transactions: usize,
}

impl App {
    pub async fn new() -> Result<Self> {
        let config = Self::load_config()?;
        
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            state: Arc::new(RwLock::new(AppState::default())),
        })
    }

    fn load_config() -> Result<AppConfig, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .build()?;

        config.try_deserialize()
    }

    pub async fn get_config(&self) -> AppConfig {
        self.config.read().await.clone()
    }

    pub async fn update_state<F>(&self, f: F)
    where
        F: FnOnce(&mut AppState),
    {
        let mut state = self.state.write().await;
        f(&mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_initialization() {
        let app = App::new().await.unwrap();
        let config = app.get_config().await;
        assert!(config.network.max_peers > 0);
    }
} 