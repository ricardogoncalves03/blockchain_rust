use chrono::prelude::*;
use sha2::{Digest, Sha256};

pub struct Block {
    pub timestamp: i64,
    pub data: String,
    pub nonce: i64,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(data: String, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut nonce = 0;
        let mut hash = String::new();

        loop {
            let hash_candidate = Self::calculate_hash(&timestamp, &data, &nonce, &previous_hash);
            if hash_candidate.starts_with(&"0".repeat(difficulty)) {
                hash = hash_candidate;
                break;
            } else {
                nonce += 1;
            }
        }

        Self {
            timestamp,
            data,
            nonce,
            previous_hash,
            hash,
            //calculate_hash,
        }
    }

    pub fn calculate_hash(
        timestamp: &i64,
        data: &String,
        nonce: &i64,
        previous_hash: &String,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(timestamp.to_string().as_bytes());
        hasher.update(data.as_bytes());
        hasher.update(nonce.to_string().as_bytes());
        hasher.update(previous_hash.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
