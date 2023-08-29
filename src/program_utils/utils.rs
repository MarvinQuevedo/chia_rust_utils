use chia_protocol::Bytes32;

use sha2::{Digest, Sha256, Sha512};

pub const INFINITE_COST: u64 = 0x7FFFFFFFFFFFFFFF;

pub fn hash_256(input: Vec<u8>) -> Bytes32 {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let bytes = hasher.finalize().to_vec();
    let bytes_32: [u8; 32] = bytes[..32].try_into().expect("slice with incorrect length");
    Bytes32::from(bytes_32)
}

pub fn hash_512(input: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}
