use crate::models::transaction::Transaction;

pub struct Block {
    pub number: u64,
    pub transactions: Vec<Transaction>,

    pub gas_used: u64,
    pub gas_limit: u64,

    pub base_fee: u64,
}


impl Block {
    pub fn new(number: u64, base_fee: u64) -> Self {
        Self {
            number,
            transactions: Vec::new(),
            gas_used: 0,
            gas_limit: 30_000_000, // Ethereum-like default
            base_fee,
        }
    }


        pub fn can_include(&self, tx: &Transaction) -> bool {
        // 1. Gas limit check
        if self.gas_used + tx.gas_limit > self.gas_limit {
            return false;
        }

        // 2. Fee validity (EIP-1559)
        if !tx.can_pay_base_fee(self.base_fee) {
            return false;
        }

        true
    }


        pub fn add_tx(&mut self, tx: Transaction) {
        self.gas_used += tx.gas_limit;
        self.transactions.push(tx);
    }
}