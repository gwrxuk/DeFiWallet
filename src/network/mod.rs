use crate::core::App;
use anyhow::Result;
use async_trait::async_trait;
use futures::StreamExt;
use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity,
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess, Swarm},
    tcp::TokioTcpConfig,
    Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    WalletUpdate {
        address: String,
        balance: f64,
    },
    Transaction {
        from: String,
        to: String,
        amount: f64,
        chain_type: String,
    },
    PeerDiscovery {
        peers: Vec<String>,
    },
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "NetworkEvent")]
pub struct WalletBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
}

#[derive(Debug)]
pub enum NetworkEvent {
    Floodsub(FloodsubEvent),
    Mdns(MdnsEvent),
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for WalletBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        self.floodsub.inject_event(event);
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for WalletBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        self.mdns.inject_event(event);
    }
}

pub struct Network {
    app: Arc<App>,
    swarm: Swarm<WalletBehaviour>,
    event_sender: mpsc::Sender<NetworkEvent>,
    event_receiver: mpsc::Receiver<NetworkEvent>,
}

impl Network {
    pub async fn new(app: Arc<App>) -> Result<Self> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = local_key.public().into_peer_id();

        let transport = TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(libp2p::noise::NoiseAuthenticated::xx(&local_key).unwrap())
            .multiplex(libp2p::mplex::MplexConfig::new())
            .boxed();

        let mut behaviour = WalletBehaviour {
            floodsub: Floodsub::new(local_peer_id),
            mdns: Mdns::new(Default::default()).await?,
        };

        // Subscribe to topics
        let wallet_topic = Topic::new("wallet-updates");
        behaviour.floodsub.subscribe(wallet_topic);

        let (event_sender, event_receiver) = mpsc::channel(100);
        let swarm = Swarm::new(transport, behaviour, local_peer_id);

        Ok(Self {
            app,
            swarm,
            event_sender,
            event_receiver,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let config = self.app.get_config().await;
        let listen_addr = config.network.listen_addr.parse()?;
        
        self.swarm.listen_on(listen_addr)?;

        loop {
            tokio::select! {
                swarm_event = self.swarm.next() => {
                    if let Some(event) = swarm_event {
                        self.handle_swarm_event(event).await?;
                    }
                }
                Some(event) = self.event_receiver.recv() => {
                    self.handle_network_event(event).await?;
                }
            }
        }
    }

    async fn handle_swarm_event(&mut self, event: libp2p::swarm::SwarmEvent<NetworkEvent>) -> Result<()> {
        match event {
            libp2p::swarm::SwarmEvent::Behaviour(NetworkEvent::Floodsub(floodsub_event)) => {
                self.handle_floodsub_event(floodsub_event).await?;
            }
            libp2p::swarm::SwarmEvent::Behaviour(NetworkEvent::Mdns(mdns_event)) => {
                self.handle_mdns_event(mdns_event).await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_floodsub_event(&mut self, event: FloodsubEvent) -> Result<()> {
        match event {
            FloodsubEvent::Message(message) => {
                if let Ok(network_message) = serde_json::from_slice::<NetworkMessage>(&message.data) {
                    self.handle_network_message(network_message).await?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_mdns_event(&mut self, event: MdnsEvent) -> Result<()> {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, _addr) in list {
                    self.swarm.behaviour_mut().floodsub.add_node_to_partial_view(peer_id);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, _addr) in list {
                    if !self.swarm.behaviour_mut().mdns.has_node(&peer_id) {
                        self.swarm.behaviour_mut().floodsub.remove_node_from_partial_view(&peer_id);
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_network_message(&mut self, message: NetworkMessage) -> Result<()> {
        match message {
            NetworkMessage::WalletUpdate { address, balance } => {
                // Update wallet balance in the app state
                self.app.update_state(|state| {
                    // Update relevant state
                }).await;
            }
            NetworkMessage::Transaction { from, to, amount, chain_type } => {
                // Handle incoming transaction
            }
            NetworkMessage::PeerDiscovery { peers } => {
                // Handle peer discovery
            }
        }
        Ok(())
    }

    async fn handle_network_event(&mut self, event: NetworkEvent) -> Result<()> {
        match event {
            NetworkEvent::Floodsub(event) => {
                self.swarm.behaviour_mut().floodsub.inject_event(event);
            }
            NetworkEvent::Mdns(event) => {
                self.swarm.behaviour_mut().mdns.inject_event(event);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_initialization() {
        let app = Arc::new(App::new().await.unwrap());
        let network = Network::new(app).await.unwrap();
        assert!(network.event_sender.capacity() > 0);
    }
} 