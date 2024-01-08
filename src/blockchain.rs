use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new("Genesis block".to_string(), String::new(), 1);

        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = if let Some(last_block) = self.chain.last() {
            last_block.hash.clone()
        } else {
            String::new()
        };

        let new_block = Block::new(data, previous_hash, 1);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash
                != Block::calculate_hash(
                    &current_block.timestamp,
                    &current_block.data,
                    &current_block.nonce,
                    &current_block.previous_hash,
                )
                || current_block.previous_hash != previous_block.hash
            {
                return false;
            }
        }
        true
    }
}
