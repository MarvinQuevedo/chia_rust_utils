use crate::blockchain::vdf_output::VdfOutput;
use chia_utils_streamable_macro::sized_bytes::Bytes32;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VdfInfo {
    pub challenge: Bytes32,
    pub output: VdfOutput,
    pub number_of_iterations: u64,
}
