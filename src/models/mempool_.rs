use std::collections::{BinaryHeap, HashMap};

use crate::models::{account_state::AccountState, address::Address, tx_meta::TxMeta};

pub struct Mempool {
    // Fast lookup
    pub txs: HashMap<u64, TxMeta>,

    // Priority queue (max heap)
    pub priority_queue: BinaryHeap<TxMeta>,

    // Per-account tracking
    pub accounts: HashMap<Address, AccountState>,

    pub current_size: usize,
}