use crate::models::transaction::Transaction;

pub struct Block {
    pub number: u64,
    pub transactions: Vec<Transaction>,

    pub gas_used: u64,
    pub gas_limit: u64,

    pub base_fee: u64,
}