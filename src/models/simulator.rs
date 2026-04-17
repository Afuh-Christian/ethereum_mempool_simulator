use std::sync::atomic::AtomicU64;

use crate::models::{constants::Config, fee_market::FeeMarket, mempool_::Mempool};

pub struct Simulator {
    pub mempool: Mempool,
    pub fee_market: FeeMarket,

    pub current_block: u64,
    pub tps_counter: AtomicU64,

    pub config: Config,
}