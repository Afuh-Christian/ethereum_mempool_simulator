use std::sync::atomic::AtomicU64;

use crate::models::{block_builder::BlockBuilder, constants::Config, fee_market::FeeMarket, mempool_::Mempool, network_event::Network, transaction::Transaction};

// pub struct Simulator {
//     pub mempool: Mempool,
//     pub fee_market: FeeMarket,

//     pub current_block: u64,
//     pub tps_counter: AtomicU64,

//     pub config: Config,
// }

pub struct Simulator {
    pub mempool: Mempool,
    pub network: Network,
    pub block_builder: BlockBuilder,
    pub fee_market: FeeMarket,

    pub current_time: u64,
    pub block_time: u64, // e.g. 12 seconds (or ms in your sim)

    pub last_block_time: u64,

    pub running: bool,
}



impl Simulator {
    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            self.tick();
        }
    }


        pub fn tick(&mut self) {
        // 1. Process incoming network txs
        while let Some(tx) = self.network.receive_tx(self.current_time) {
            self.process_incoming_tx(tx);
        }

        // 2. Produce block if it's time
        if self.current_time - self.last_block_time >= self.block_time {
            self.produce_block();
            self.last_block_time = self.current_time;
        }

        // 3. Cleanup mempool
        self.mempool.expire_old_transactions(self.current_time);

        // 4. Metrics
        self.record_metrics();

        // 5. Advance time
        self.advance_time();
    }
}


impl Simulator {
    pub fn process_incoming_tx(&mut self, tx: Transaction) {
        // Optional: validate before adding
        if let Err(_) = self.mempool.add_transaction(tx) {
            // drop invalid tx
        }
    }
}


impl Simulator {
    pub fn produce_block(&mut self) {
        let block = self.block_builder.build_block(&mut self.mempool);

        // 🔥 Update fee market based on block usage
        self.fee_market.update_base_fee(block.gas_used);

        // Sync mempool base fee
        self.mempool.base_fee = self.fee_market.base_fee;

        // (Optional) log block
        println!(
            "Block {} | txs: {} | gas_used: {} | base_fee: {}",
            block.number,
            block.transactions.len(),
            block.gas_used,
            block.base_fee
        );
    }
}


impl Simulator {
    pub fn advance_time(&mut self) {
        self.current_time += 1; // 1 tick = 1 unit (ms or abstract)
    }
}

impl Simulator {
    pub fn record_metrics(&mut self) {
        // Minimal example — expand later
        println!(
            "time: {} | mempool size: {}",
            self.current_time,
            self.mempool.size()
        );
    }
}