use crate::models::transaction::Transaction;

pub struct TxMeta {
    pub tx: Transaction,

    pub effective_gas_price: u64, // base_fee + priority_fee
    pub is_executable: bool,      // nonce satisfied
}

