use crate::models::{block::Block, mempool_::Mempool, transaction::Transaction};

pub struct BlockBuilder {
    pub block_number: u64,
}


impl BlockBuilder {
    pub fn build_block(&mut self, mempool: &mut Mempool) -> Block {
        let mut block = Block::new(self.block_number, mempool.base_fee);

        let selected_txs = self.select_transactions(mempool);

        for tx in selected_txs {
            block.add_tx(tx);
        }

        self.block_number += 1;

        block
    }


        pub fn select_transactions(&mut self, mempool: &mut Mempool) -> Vec<Transaction> {
        let mut selected = Vec::new();

        // Build a temporary block for constraint checking
        let mut temp_block = Block::new(self.block_number, mempool.base_fee);

        while let Some(entry) = mempool.pop_best_tx() {
            let tx = entry.tx;

            // 1. Check if tx still valid under current base fee + gas limit
            if !temp_block.can_include(&tx) {
                continue; // skip but continue trying others
            }

            // 2. Include tx
            temp_block.add_tx(tx.clone());
            selected.push(tx);

            // 3. Stop if block is full (optional optimization)
            if temp_block.gas_used >= temp_block.gas_limit {
                break;
            }
        }

        selected
    }
}