use crate::blockchain::proof_of_space::ProofOfSpace;
use crate::blockchain::vdf_info::VdfInfo;
use chia_utils_streamable_macro::sized_bytes::{Bytes32, Bytes96};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RewardChainBlockUnfinished {
    pub total_iters: u128,
    pub signage_point_index: u8,
    pub pos_ss_cc_challenge_hash: Bytes32,
    pub proof_of_space: ProofOfSpace,
    pub challenge_chain_sp_vdf: Option<VdfInfo>,
    pub challenge_chain_sp_signature: Bytes96,
    pub reward_chain_sp_vdf: Option<VdfInfo>,
    pub reward_chain_sp_signature: Bytes96,
}
