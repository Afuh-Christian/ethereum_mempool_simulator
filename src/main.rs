
mod models;
// use simulator::Simulator;
// use mempool::Mempool;
// use block_builder::BlockBuilder;
// use fee_market::FeeMarket;
// use network::Network;
// use metrics::Metrics;
// use transaction::Transaction;

use std::collections::{HashMap, BinaryHeap, VecDeque};
use std::sync::{Arc, Mutex};

use crate::models::block_builder::BlockBuilder;
use crate::models::fee_market::FeeMarket;
use crate::models::mempool_::Mempool;
use crate::models::metrix::Metrics;
use crate::models::network_event::Network;
use crate::models::simulator::Simulator;
use crate::models::transaction::Transaction;
use crate::models::utils::Utility;

fn main() {
    // -----------------------------
    // ⚙️ INITIAL SETUP
    // -----------------------------

    let mempool = Mempool {
        txs: HashMap::new(),
        accounts: HashMap::new(),
        priority_queue: BinaryHeap::new(),
        base_fee: 1_000,
        max_size: 100_000,
    };

    let network = Network {
        queue: Arc::new(Mutex::new(VecDeque::new())),
        min_latency: 1,
        max_latency: 5,
        current_time: Utility::now()
    };

    let fee_market = FeeMarket {
        base_fee: 1_000,
        target_gas: 15_000_000,
        max_gas: 30_000_000,
        base_fee_max_change_denominator: 8,
    };

    let block_builder = BlockBuilder {
        block_number: 0,
    };

    let metrics = Metrics {
        total_received: 0,
        total_included: 0,
        total_dropped: 0,
        latencies: VecDeque::new(),
        max_samples: 1000,
        avg_latency: 0,
    };

    let mut simulator = Simulator {
        mempool,
        network,
        block_builder,
        fee_market,
        current_time: 0,
        block_time: 12,
        last_block_time: 0,
        running: true,
    };

    // -----------------------------
    // 🚀 SIMULATION LOOP
    // -----------------------------

    for i in 0..10_000 {
        // Generate fake transactions (simple)
        let tx = Utility::generate_tx(i as u64);

        simulator.network.broadcast_tx(tx);

        simulator.tick();

        // Print every 100 ticks
        if i % 100 == 0 {
            println!(
                "time: {} | mempool: {}",
                simulator.current_time,
                simulator.mempool.size()
            );
        }
    }

    println!("Simulation complete.");
}



