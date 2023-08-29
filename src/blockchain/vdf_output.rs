use chia_utils_streamable_macro::sized_bytes::UnsizedBytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VdfOutput {
    pub data: UnsizedBytes,
}
