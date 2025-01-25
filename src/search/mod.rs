use crate::data::OsintData;
use std::collections::HashMap;
use std::error::Error;
use futures::Stream;

pub struct SearchEngine {
	index: HashMap<String, Vec<String>>, // keyword -> data_ids
}

#[async_trait::async_trait]
pub trait SearchOperations {
	async fn search(&self, query: &str) -> Result<Vec<OsintData>, Box<dyn Error>>;
	async fn index_data(&mut self, data: &OsintData) -> Result<(), Box<dyn Error>>;
}

impl SearchEngine {
	pub fn new() -> Self {
		Self {
			index: HashMap::new(),
		}
	}

	fn tokenize(text: &str) -> Vec<String> {
		text.split_whitespace()
			.map(|s| s.to_lowercase())
			.collect()
	}
}

pub struct SearchResult {
	pub data: OsintData,
	pub relevance_score: f64,
}