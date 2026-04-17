use crate::models::transaction::Transaction;

pub const BLOCK_TIME_SECS: u64 = 12;     // avg Ethereum block time
pub const SLOT_TIME_SECS: u64 = 12;      // PoS slot time (same as block time)

pub const TARGET_GAS_PER_BLOCK: u64 = 15_000_000; // target
pub const MAX_GAS_PER_BLOCK: u64 = 30_000_000;    // hard cap (2x target)


pub const INITIAL_BASE_FEE: u64 = 1_000_000_000; // 1 gwei
pub const MIN_BASE_FEE: u64 = 1;
pub const MAX_BASE_FEE_CHANGE_DENOM: u64 = 8; // max 12.5% change per block

pub const MIN_PRIORITY_FEE: u64 = 1_000_000_000; // 1 gwei
pub const MAX_PRIORITY_FEE: u64 = 100_000_000_000; // 100 gwei

pub const SIMPLE_TX_GAS: u64 = 21_000;      // basic ETH transfer
pub const CONTRACT_TX_GAS_MIN: u64 = 50_000;
pub const CONTRACT_TX_GAS_MAX: u64 = 5_000_000;

pub const MAX_MEMPOOL_TXS: usize = 300_000;
pub const MAX_MEMPOOL_BYTES: usize = 512 * 1024 * 1024; // ~512MB

pub const MAX_PENDING_PER_ACCOUNT: usize = 64;

pub const REPLACEMENT_BUMP_PERCENT: u64 = 10;

pub const TX_TTL_SECS: u64 = 3 * 60 * 60; // ~3 hours

pub const MIN_EFFECTIVE_GAS_PRICE: u64 = 1_000_000_000; // 1 gwei floor
pub const EVICT_BATCH_SIZE: usize = 5_000;

pub const MAX_NONCE_GAP: u64 = 64;


pub const MIN_LATENCY_MS: u64 = 50;
pub const MAX_LATENCY_MS: u64 = 400;
pub const GOSSIP_DUPLICATE_RATE: f64 = 0.1;

pub const TARGET_TPS: usize = 15;   // normal conditions
pub const BURST_TPS: usize = 200;   // during spikes




pub struct Config {
    pub target_tps: usize,
    pub block_time_secs: u64,

    pub max_mempool_size: usize,
    pub tx_ttl_secs: u64,

    pub max_gas_per_block: u64,
    pub target_gas_per_block: u64,
}


// pub struct Metrics {
//     pub total_txs: u64,
//     pub dropped_txs: u64,
//     pub included_txs: u64,

//     pub avg_latency_ms: u64,
//     pub mempool_size: usize,
// }


pub struct ReplacementInfo {
    pub replaced_tx: u64,
    pub new_tx: u64,
    pub fee_bump: u64,
}

