use crate::blockchain::vdf_info::VdfInfo;
use chia_utils_streamable_macro::sized_bytes::Bytes32;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RewardChainSubSlot {
    pub end_of_slot_vdf: VdfInfo,
    pub challenge_chain_sub_slot_hash: Bytes32,
    pub infused_challenge_chain_sub_slot_hash: Option<Bytes32>,
    pub deficit: u8,
}
