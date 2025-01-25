use libp2p::{
	core::upgrade,
	identity,
	PeerId,
	Transport,
};
use async_trait::async_trait;
use std::error::Error;

pub struct NetworkManager {
	peer_id: PeerId,
	// Will hold network state
}

#[async_trait]
pub trait PeerConnection {
	async fn connect(&mut self, peer: PeerId) -> Result<(), Box<dyn Error>>;
	async fn disconnect(&mut self, peer: PeerId) -> Result<(), Box<dyn Error>>;
	async fn broadcast_data(&self, data: Vec<u8>) -> Result<(), Box<dyn Error>>;
}

impl NetworkManager {
	pub fn new() -> Self {
		let keypair = identity::Keypair::generate_ed25519();
		let peer_id = PeerId::from(keypair.public());
		
		Self {
			peer_id,
		}
	}
}