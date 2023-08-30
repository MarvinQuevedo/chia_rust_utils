use chia_utils_streamable_macro::bytes_utils::Endian;
use clvm_tools_rs::classic::clvm::casts::bigint_to_bytes_clvm;
use num::{Num, Zero};
use num_bigint::BigInt;

pub fn bytes_to_big_int(bytes: &Vec<u8>, endian: Endian, signed: bool) -> BigInt {
    if bytes.is_empty() {
        return BigInt::zero();
    }

    let mut bytes_list: Vec<u8> = bytes.clone();
    if endian == Endian::Little {
        bytes_list.reverse();
    }

    let hex = hex::encode(bytes_list);
    if signed {
        let signed_bytes = BigInt::from_str_radix(&hex, 16)
            .unwrap()
            .to_signed_bytes_le();
        BigInt::from_signed_bytes_le(&signed_bytes)
    } else {
        BigInt::from_str_radix(&hex, 16).unwrap()
    }
}
pub fn big_int_to_bytes(big_int: &BigInt) -> Vec<u8> {
    return bigint_to_bytes_clvm(big_int).raw();
}
