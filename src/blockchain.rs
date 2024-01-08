use crate::block::Block;
use crate::transaction::Transaction;
use crate::transaction_pool::TransactionPool;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub transaction_pool: TransactionPool,
    difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(vec![], String::new(), 4);

        Self {
            chain: vec![genesis_block],
            transaction_pool: TransactionPool::new(),
            difficulty: 4,
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transaction_pool.add_transaction(transaction);
    }

    pub fn add_block(&mut self) {
        let transactions = self.transaction_pool.get_transactions_for_block();
        let previous_hash = if let Some(last_block) = self.chain.last() {
            last_block.hash.clone()
        } else {
            String::new()
        };
        let new_block = Block::new(transactions, previous_hash, self.difficulty);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash
                != Block::calculate_hash(
                    &current_block.timestamp,
                    &current_block.transactions,
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
