mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("block 1 data".to_string());
    blockchain.add_block("Block 2 Data".to_string());
    blockchain.add_block("Block 3 Data".to_string());

    // Validate and display the blockchain
    println!("Blockchain is valid: {}", blockchain.is_valid());
    for block in blockchain.chain.iter() {
        println!("___________________________");
        println!("Timestamp: {}", block.timestamp);
        println!("Data: {}", block.data);
        println!("Nonce: {}", block.nonce);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
    }
}
