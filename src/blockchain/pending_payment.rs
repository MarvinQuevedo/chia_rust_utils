use chia_utils_streamable_macro::sized_bytes::Bytes32;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PendingPayment {
    pub puzzle_hash: Bytes32,
    pub amount: u64,
}
