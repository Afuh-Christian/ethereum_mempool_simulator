use std::{collections::{BinaryHeap, HashMap}, time::{SystemTime, UNIX_EPOCH}};

use crate::models::{account_state::AccountState, address::Address, mempool_entry::MempoolEntry, transaction::Transaction, tx_meta::TxMeta, utils::Utility};

pub struct Mempool {
    pub txs: HashMap<u64, MempoolEntry>,        // hash → entry
    pub accounts: HashMap<Address, AccountState>,
    pub priority_queue: BinaryHeap<MempoolEntry>,

    pub base_fee: u64,
    pub max_size: usize,
}


impl Mempool {
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), &'static str> {
        // 1. Basic validation
        self.validate_transaction(&tx)?;

        let sender = tx.sender;
        let hash = tx.hash;

        let account = self.accounts
            .entry(sender)
            .or_insert(AccountState {
                address: sender,
                current_nonce: tx.nonce,
                pending: Default::default(),
            });

        // 2. Insert into account (handles replacement)
        account.insert_tx(tx.clone())?;

        // 3. Create entry
        let mut entry = MempoolEntry::new(
            tx,
            self.base_fee,
            account.current_nonce,
            Utility::now(), // assume helper
        );

        // 4. Store globally
        self.txs.insert(hash, entry.clone());

        // 5. If executable → push to heap
        if entry.is_executable {
            self.push_to_priority_queue(entry);
        }

        Ok(())
    }


        pub fn validate_transaction(&self, tx: &Transaction) -> Result<(), &'static str> {
        if !tx.is_valid_basic() {
            return Err("invalid tx");
        }

        if !tx.can_pay_base_fee(self.base_fee) {
            return Err("fee too low");
        }

        Ok(())
    }


        pub fn handle_replacement(&mut self, tx: Transaction) -> bool {
        let sender = tx.sender;

        if let Some(account) = self.accounts.get_mut(&sender) {
            return account.replace_tx(tx);
        }

        false
    }



    //     pub fn update_account_executability(&mut self, address: Address) {
    //     if let Some(account) = self.accounts.get_mut(&address) {
    //         let nonce = account.current_nonce;

    //         if let Some(tx) = account.get_next_executable() {
    //             let hash = tx.hash;

    //             if let Some(entry) = self.txs.get_mut(&hash) {
    //                 entry.refresh_executability(nonce);

    //                 if entry.is_executable {
    //                     self.push_to_priority_queue(entry.clone());
    //                 }
    //             }
    //         }
    //     }
    // }

    
    pub fn update_account_executability(&mut self, address: Address) {
    // Step 1: extract required data WITHOUT holding mutable borrow
    let (hash, nonce) = if let Some(account) = self.accounts.get(&address) {
        if let Some(tx) = account.get_next_executable() {
            (tx.hash, account.current_nonce)
        } else {
            return;
        }
    } else {
        return;
    };

    // Step 2: work on tx entry in a separate scope
    let entry_to_push = {
        if let Some(entry) = self.txs.get_mut(&hash) {
            entry.refresh_executability(nonce);

            if entry.is_executable {
                Some(entry.clone())
            } else {
                None
            }
        } else {
            None
        }
    }; // 🔥 mutable borrow of self.txs ends HERE

    // Step 3: now safe to borrow self again
    if let Some(entry) = entry_to_push {
        self.push_to_priority_queue(entry);
    }
}
 
 
        pub fn promote_to_executable(&mut self, entry: MempoolEntry) {
        if entry.is_executable {
            self.priority_queue.push(entry);
        }
    }
        pub fn push_to_priority_queue(&mut self, entry: MempoolEntry) {
        self.priority_queue.push(entry);
    }


        pub fn pop_best_tx(&mut self) -> Option<MempoolEntry> {
        while let Some(entry) = self.priority_queue.pop() {
            let hash = entry.tx.hash;

            // Skip if removed or stale
            if !self.txs.contains_key(&hash) {
                continue;
            }

            let sender = entry.tx.sender;

            if let Some(account) = self.accounts.get_mut(&sender) {
                // Ensure still executable
                if entry.tx.nonce != account.current_nonce {
                    continue;
                }

                // Remove from account + global map
                account.pop_next_executable();
                self.txs.remove(&hash);

                // 🔥 Unlock next tx in chain
                self.update_account_executability(sender);

                return Some(entry);
            }
        }

        None
    }


        pub fn remove_tx(&mut self, tx_hash: u64) {
        if let Some(entry) = self.txs.remove(&tx_hash) {
            if let Some(account) = self.accounts.get_mut(&entry.tx.sender) {
                account.remove_tx(entry.tx.nonce);
            }
        }
    }




        pub fn evict_low_priority(&mut self) {
        if self.txs.len() <= self.max_size {
            return;
        }

        // naive: remove random low-fee entries
        let mut to_remove = vec![];

        for (hash, entry) in self.txs.iter() {
            if entry.effective_gas_price < self.base_fee {
                to_remove.push(*hash);
            }

            if to_remove.len() > 1000 {
                break;
            }
        }

        for hash in to_remove {
            self.remove_tx(hash);
        }
    }


        pub fn expire_old_transactions(&mut self, now: u64) {
        let ttl = 300; // seconds

        let mut to_remove = vec![];

        for (hash, entry) in self.txs.iter() {
            if now - entry.received_at > ttl {
                to_remove.push(*hash);
            }
        }

        for hash in to_remove {
            self.remove_tx(hash);
        }
    }


        pub fn get_tx(&self, hash: u64) -> Option<&MempoolEntry> {
        self.txs.get(&hash)
    }

    pub fn size(&self) -> usize {
        self.txs.len()
    }
}