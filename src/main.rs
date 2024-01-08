mod block;
mod blockchain;
mod transaction;
mod transaction_pool;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::new();

    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
    let tx2 = Transaction::new("Bob".to_string(), "Alice".to_string(), 5.0);

    blockchain.add_transaction(tx1);
    blockchain.add_transaction(tx2);

    blockchain.add_block();
    blockchain.add_block();
    blockchain.add_block();

    // Validate and display the blockchain
    println!("Blockchain is valid: {}", blockchain.is_valid());
    for block in blockchain.chain.iter() {
        println!("___________________________");
        println!("Timestamp: {}", block.timestamp);
        println!("Transactions: {:?}", block.transactions);
        println!("Nonce: {}", block.nonce);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
    }
}
