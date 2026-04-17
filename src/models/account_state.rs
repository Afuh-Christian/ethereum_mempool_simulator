use std::collections::BTreeMap;

use crate::models::{address::Address, transaction::Transaction};

pub struct AccountState {
    pub address: Address,
    pub current_nonce: u64,

    // Pending transactions sorted by nonce
    pub pending: BTreeMap<u64, Transaction>,
}