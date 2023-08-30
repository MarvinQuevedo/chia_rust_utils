use chia_bls::PublicKey;
use chia_utils_streamable_macro::bytes_utils::Endian;
use num_bigint::BigInt;
use sha2::{Digest, Sha256};

use crate::{
    chia_wallet::standart::puzzles::p2_delegated_puzzle_or_hidden_puzzle::default_hidden_puzzle,
    program_utils::bytes::bytes_to_big_int,
};

lazy_static::lazy_static! {

    pub static ref GROUP_ORDER: BigInt = {
            let zeros = BigInt::parse_bytes(
                b"73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001",
                16,
            )
            .unwrap();
            zeros
    };

}
pub fn calculate_synthetic_offset(public_key: &PublicKey) -> BigInt {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(&public_key.to_bytes());
    bytes.extend_from_slice(&default_hidden_puzzle().tree_hash());
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let blob: Vec<u8> = hasher.finalize().to_vec();

    let offset = bytes_to_big_int(&blob, Endian::Big, true);
    let group_order = GROUP_ORDER.clone();

    let new_offset = &offset % &group_order;
    new_offset
}
