use crate::models::address::Address;



#[derive(Clone)]
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


impl Transaction {
    /// Computes the effective gas price under EIP-1559
    /// effective_fee = base_fee + min(priority_fee, max_fee - base_fee)
    pub fn effective_gas_price(&self, base_fee: u64) -> u64 {
        // If max fee can't even cover base fee, tx is not valid for inclusion
        if self.max_fee_per_gas <= base_fee {
            return 0;
        }

        let max_tip = self.max_fee_per_gas - base_fee;

        let priority_fee = self
            .max_priority_fee_per_gas
            .min(max_tip);

        base_fee + priority_fee
    }

    /// Basic structural validation (cheap checks only)
    pub fn is_valid_basic(&self) -> bool {
        // Gas constraints
        if self.gas_limit == 0 {
            return false;
        }

        // Fee constraints
        if self.max_fee_per_gas == 0 {
            return false;
        }

        // Priority fee cannot exceed max fee
        if self.max_priority_fee_per_gas > self.max_fee_per_gas {
            return false;
        }

        // Size sanity check (optional but realistic)
        if self.size == 0 {
            return false;
        }

        true
    }

    /// Checks if transaction can be included given current base fee
    pub fn can_pay_base_fee(&self, base_fee: u64) -> bool {
        self.max_fee_per_gas >= base_fee
    }
    
}