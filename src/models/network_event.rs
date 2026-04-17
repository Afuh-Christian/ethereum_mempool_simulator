use crate::models::transaction::Transaction;

pub enum NetworkEvent {
    NewTransaction(Transaction),
    DuplicateTransaction(u64),
    DroppedTransaction(u64),
}