use std::{cmp::Ordering, time::{SystemTime, UNIX_EPOCH}};

use crate::models::{address::Address, transaction::Transaction};


#[derive(Clone)]
pub struct MempoolEntry {
    pub tx: Transaction,
    pub received_at: u64,

    pub effective_gas_price: u64,
    pub is_executable: bool,

}

impl MempoolEntry {
    /// Create a new entry with computed fee + initial executability
    pub fn new(tx: Transaction, base_fee: u64, current_nonce: u64, now: u64) -> Self {
        let effective_gas_price = tx.effective_gas_price(base_fee);

        let is_executable = tx.nonce == current_nonce;

        Self {
            tx,
            received_at: now,
            effective_gas_price,
            is_executable,
        }
    }



    /// Update cached fee when base fee changes (after each block)
    pub fn update_effective_fee(&mut self, base_fee: u64) {
        self.effective_gas_price = self.tx.effective_gas_price(base_fee);
    }

    /// Re-check executability based on current account nonce
    pub fn is_executable(&self, current_nonce: u64) -> bool {
        self.tx.nonce == current_nonce
    }

    /// Optional: update cached executability (useful after block inclusion)
    pub fn refresh_executability(&mut self, current_nonce: u64) {
        self.is_executable = self.is_executable(current_nonce);
    }
}


impl Eq for MempoolEntry {}

impl PartialEq for MempoolEntry {
    fn eq(&self, other: &Self) -> bool {
        self.tx.hash == other.tx.hash
    }
}

impl Ord for MempoolEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Max-heap → higher fee = higher priority
        self.effective_gas_price
            .cmp(&other.effective_gas_price)
            // Tie-breaker: earlier tx wins (lower timestamp = higher priority)
            .then_with(|| other.received_at.cmp(&self.received_at))
    }
}

impl PartialOrd for MempoolEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

