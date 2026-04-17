use crate::models::address::Address;



pub struct Transaction {
    pub hash: u64,

    pub sender: Address,
    pub nonce: u64,

    // EIP-1559 fields
    pub max_fee_per_gas: u64,
    pub max_priority_fee_per_gas: u64,

    pub gas_limit: u64,
    pub size: usize,

    pub timestamp: u64,
}