use ring::{rand, signature::{self, KeyPair}};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIdentity {
	pub id: String,
	pub public_key: Vec<u8>,
	#[serde(skip_serializing)]
	private_key: Vec<u8>,
	pub name: Option<String>,
	pub known_peers: Vec<String>,
}

pub struct IdentityManager {
	identity: Option<UserIdentity>,
	key_path: PathBuf,
}

impl IdentityManager {
	pub fn new() -> Self {
		Self {
			identity: None,
			key_path: PathBuf::from("user_identity.json"),
		}
	}

	pub fn generate_new_identity(&mut self, name: Option<String>) -> Result<(), Box<dyn Error>> {
		let rng = rand::SystemRandom::new();
		let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)
			.map_err(|e| format!("Failed to generate key pair: {:?}", e))?;
		let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
			.map_err(|e| format!("Failed to create key pair from PKCS8: {:?}", e))?;
		
		let identity = UserIdentity {
			id: blake3::hash(key_pair.public_key().as_ref()).to_string(),
			public_key: key_pair.public_key().as_ref().to_vec(),
			private_key: pkcs8_bytes.as_ref().to_vec(),
			name,
			known_peers: Vec::new(),
		};

		self.identity = Some(identity);
		self.save_identity()
	}

	pub fn export_recovery_key(&self) -> Result<String, Box<dyn Error>> {
		match &self.identity {
			Some(identity) => {
				Ok(BASE64.encode(&identity.private_key))
			}
			None => Err("No identity loaded".into()),
		}
	}

	pub fn import_identity(&mut self, recovery_key: &str) -> Result<(), Box<dyn Error>> {
		let pkcs8_bytes = BASE64.decode(recovery_key)
			.map_err(|e| format!("Failed to decode recovery key: {}", e))?;
		let key_pair = signature::Ed25519KeyPair::from_pkcs8(&pkcs8_bytes)
			.map_err(|e| format!("Failed to create key pair from recovery key: {:?}", e))?;
		
		let identity = UserIdentity {
			id: blake3::hash(key_pair.public_key().as_ref()).to_string(),
			public_key: key_pair.public_key().as_ref().to_vec(),
			private_key: pkcs8_bytes,
			name: None,
			known_peers: Vec::new(),
		};

		self.identity = Some(identity);
		self.save_identity()
	}

	fn save_identity(&self) -> Result<(), Box<dyn Error>> {
		if let Some(identity) = &self.identity {
			let json = serde_json::to_string_pretty(identity)?;
			fs::write(&self.key_path, json)?;
			Ok(())
		} else {
			Err("No identity to save".into())
		}
	}
}