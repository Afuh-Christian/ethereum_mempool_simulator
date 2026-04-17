use std::collections::BTreeMap;

use crate::models::{address::Address, transaction::Transaction};

pub struct AccountState {
    pub address: Address,
    pub current_nonce: u64,

    // Pending transactions sorted by nonce
    pub pending: BTreeMap<u64, Transaction>,
}



impl AccountState {
    pub fn insert_tx(&mut self, tx: Transaction) -> Result<(), &'static str> {
        let nonce = tx.nonce;

        // Reject old nonce
        if nonce < self.current_nonce {
            return Err("nonce too low");
        }

        // If same nonce exists → replacement case
        if self.pending.contains_key(&nonce) {
            if self.replace_tx(tx) {
                return Ok(());
            } else {
                return Err("replacement underpriced");
            }
        }

        // Insert normally
        self.pending.insert(nonce, tx);
        Ok(())
    }






        pub fn replace_tx(&mut self, tx: Transaction) -> bool {
        let nonce = tx.nonce;

        if let Some(existing) = self.pending.get(&nonce) {
            // Compare effective fees (you may pass base_fee if needed)
            let old_fee = existing.max_fee_per_gas;
            let new_fee = tx.max_fee_per_gas;

            // 10% bump rule
            if new_fee >= old_fee + (old_fee / 10) {
                self.pending.insert(nonce, tx);
                return true;
            }
        }

        false
    }




        pub fn get_next_executable(&self) -> Option<&Transaction> {
        self.pending.get(&self.current_nonce)
    }





        pub fn pop_next_executable(&mut self) -> Option<Transaction> {
        if let Some(tx) = self.pending.remove(&self.current_nonce) {
            self.current_nonce += 1;
            return Some(tx);
        }
        None
    }

        pub fn remove_tx(&mut self, nonce: u64) {
        self.pending.remove(&nonce);
    }

        pub fn has_nonce(&self, nonce: u64) -> bool {
        self.pending.contains_key(&nonce)
    }

}
