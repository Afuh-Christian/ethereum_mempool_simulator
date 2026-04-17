use std::collections::VecDeque;

pub struct Metrics {
    // Counters
    pub total_received: u64,
    pub total_included: u64,
    pub total_dropped: u64,

    // Latency tracking (rolling window)
    pub latencies: VecDeque<u64>,
    pub max_samples: usize,

    // Derived stats
    pub avg_latency: u64,
}

impl Metrics {
    pub fn record_tx_received(&mut self) {
        self.total_received += 1;
    }

        pub fn record_tx_included(&mut self) {
        self.total_included += 1;
    }

        pub fn record_tx_dropped(&mut self) {
        self.total_dropped += 1;
    }

        pub fn update_latency(&mut self, latency: u64) {
        // Maintain fixed-size window
        if self.latencies.len() >= self.max_samples {
            self.latencies.pop_front();
        }

        self.latencies.push_back(latency);

        // Recompute average (simple + fine for sim)
        let sum: u64 = self.latencies.iter().sum();
        self.avg_latency = if self.latencies.is_empty() {
            0
        } else {
            sum / self.latencies.len() as u64
        };
    }
}