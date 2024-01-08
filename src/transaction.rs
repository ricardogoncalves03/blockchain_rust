use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

impl Transaction {
    pub fn new (sender: String, receiver: String, amount: f64) -> Self {
        Self {
            sender,
            receiver,
            amount,
        }
    }
}
