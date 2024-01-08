use chrono::prelude::*;
use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

pub struct Block {
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub nonce: i64,
    pub previous_hash: String,
    pub timestamp: i64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut nonce = 0;
        let hash;

        loop {
            let hash_candidate = Self::calculate_hash(&timestamp, &transactions, &nonce, &previous_hash);
            if hash_candidate.starts_with(&"0".repeat(difficulty)) {
                hash = hash_candidate;
                break;
            } else {
                nonce += 1;
            }
        }

        Self {
            timestamp,
            transactions,
            nonce,
            previous_hash,
            hash,
        }
    }

    pub fn calculate_hash(
        timestamp: &i64,
        transactions: &Vec<Transaction>,
        nonce: &i64,
        previous_hash: &String,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(timestamp.to_string().as_bytes());
        let transactions_str = serde_json::to_string(transactions).unwrap();
        hasher.update(transactions_str.as_bytes());
        hasher.update(nonce.to_string().as_bytes());
        hasher.update(previous_hash.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
