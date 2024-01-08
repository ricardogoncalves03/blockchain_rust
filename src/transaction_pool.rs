use crate::transaction::{Transaction, self};

#[derive(Debug)]
pub struct TransactionPool {
    transactions: Vec<Transaction>,
}

impl TransactionPool {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn get_transactions_for_block(&mut self) -> Vec<Transaction> {
        let transactions = self.transactions.clone();
        self.transactions.clear();
        transactions
    }
}