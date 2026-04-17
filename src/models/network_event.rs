use rand::RngExt;

use crate::models::transaction::Transaction;

pub enum NetworkEvent {
    NewTransaction(Transaction),
    DuplicateTransaction(u64),
    DroppedTransaction(u64),
}


use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct Network {
    pub queue: Arc<Mutex<VecDeque<(u64, Transaction)>>>,

    pub min_latency: u64,
    pub max_latency: u64,

    pub current_time: u64, // simulation time
}


impl Network {
    pub fn broadcast_tx(&self, tx: Transaction) {
        let latency = self.simulate_latency();
        let deliver_at = self.current_time + latency;

        let mut queue = self.queue.lock().unwrap();
        queue.push_back((deliver_at, tx));
    }


        pub fn simulate_latency(&self) -> u64 {

        let mut rng = rand::rng();
        rng.random_range(self.min_latency..=self.max_latency)
    }

        pub fn receive_tx(&mut self , current_time: u64) -> Option<Transaction> {
        let mut queue = self.queue.lock().unwrap();

        if let Some(&(deliver_at, _)) = queue.front() {
            if deliver_at <= self.current_time {
                let (_, tx) = queue.pop_front().unwrap();
                return Some(tx);
            }
        }

        self.current_time = current_time;

        None
    }
}