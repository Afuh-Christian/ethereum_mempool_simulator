pub struct FeeMarket {
    pub base_fee: u64,

    pub target_gas: u64, // e.g. 15M
    pub max_gas: u64,    // e.g. 30M

    pub base_fee_max_change_denominator: u64, // = 8 (Ethereum)
}

impl FeeMarket {
    pub fn update_base_fee(&mut self, gas_used: u64) {
        self.base_fee = self.calculate_base_fee(gas_used);
    }


        pub fn calculate_base_fee(&self, gas_used: u64) -> u64 {
        // If exactly at target → no change
        if gas_used == self.target_gas {
            return self.base_fee;
        }

        // Difference from target
        let gas_delta = if gas_used > self.target_gas {
            gas_used - self.target_gas
        } else {
            self.target_gas - gas_used
        };

        // base_fee change = base_fee * gas_delta / target_gas / denominator
        let mut fee_change = self.base_fee
            .saturating_mul(gas_delta)
            / self.target_gas
            / self.base_fee_max_change_denominator;

        // 🔥 IMPORTANT: minimum change is 1 (Ethereum rule)
        if fee_change == 0 {
            fee_change = 1;
        }

        if gas_used > self.target_gas {
            // Increase
            self.base_fee.saturating_add(fee_change)
        } else {
            // Decrease (cannot go below 0)
            self.base_fee.saturating_sub(fee_change)
        }
    }
}