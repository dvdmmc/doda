use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use blake3;

#[derive(Debug, Serialize, Deserialize)]
pub struct OsintData {
	pub id: String,
	pub content: String,
	pub source: String,
	pub timestamp: SystemTime,
	pub verification_hash: String, // Changed from Hash to String
	pub signatures: Vec<Signature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
	pub peer_id: String,
	pub timestamp: SystemTime,
	pub signature: Vec<u8>,
}

pub trait DataVerification {
	fn verify_authenticity(&self) -> bool;
	fn add_signature(&mut self, signature: Signature);
	fn compute_hash(&self) -> String;
}

impl OsintData {
	pub fn new(content: String, source: String) -> Self {
		let id = blake3::hash(content.as_bytes()).to_string();
		Self {
			id,
			content,
			source,
			timestamp: SystemTime::now(),
			verification_hash: blake3::hash(b"").to_string(), // Store hash as String
			signatures: Vec::new(),
		}
	}
}