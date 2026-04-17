use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::transaction::Transaction;



pub struct Utility;


impl Utility {


       pub fn now() -> u64 {

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}


pub fn generate_address(id: u64) -> [u8; 20] {
    let mut addr = [0u8; 20];

    // put id into last 8 bytes
    addr[12..20].copy_from_slice(&id.to_be_bytes());

    addr
}


pub fn generate_tx(id: u64) -> Transaction {
    Transaction {
        hash: id,
        sender: Utility::generate_address(id % 10), // 👈 FIXED
        nonce: id / 10,

        max_fee_per_gas: 1_000 + (id % 100),
        max_priority_fee_per_gas: 10,

        gas_limit: 21_000,
        size: 100,

        timestamp: 0,
    }
}

}