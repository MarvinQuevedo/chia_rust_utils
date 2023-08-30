use chia_bls::{derive_keys, SecretKey};
use chia_utils_streamable_macro::bytes_utils::Endian;

use num_bigint::BigInt;

use crate::{
    api::bytes_to_hex,
    chia_wallet::core::derivations_utils::{calculate_synthetic_offset, GROUP_ORDER},
    program_utils::{
        bytes::{big_int_to_bytes, bytes_to_big_int},
        program::Program,
        serialized_program::SerializedProgram,
    },
};

pub fn calculate_synthetic_public_key_program() -> Program {
    SerializedProgram::from_hex("ff1dff02ffff1effff0bff02ff05808080".to_string())
        .to_program()
        .unwrap()
}

pub fn calculate_synthetic_private_key(sk: &SecretKey) -> SecretKey {
    let secret_exponent = bytes_to_big_int(&sk.to_bytes().to_vec(), Endian::Big, true);

    let public_key = sk.public_key();

    let synthetic_offset = calculate_synthetic_offset(&public_key);

    let synthetic_secret_exponent = (secret_exponent + &synthetic_offset) % &GROUP_ORDER.clone();

    let blob = big_int_to_bytes(&synthetic_secret_exponent);

    let blob_32: [u8; 32] = blob.try_into().unwrap();
    let syntetic_private_key = SecretKey::from_bytes(&blob_32);

    match syntetic_private_key {
        Ok(sk) => sk,
        Err(error) => {
            println!("Error: {:?}", error);
            panic!("Error: {:?}", error);
        }
    }
}

#[test]
fn test_calculate_synthetic_public_key_program() {
    let expected_master_pk_hex =
        "0101010101010101010101010101010101010101010101010101010101010101".to_string();

    let expected_master_sk =
        "65d5e87cd6a7d808687ae1af98168c1ec76b26f64ff4636fad94861a86041682".to_string();
    let expected_syntetic_sk =
        "68815906ff58b4a182addf70e659139e858a8355de6daf8a7117d7633b0d5dde".to_string();
    let bytes = [1u8; 32];
    let master_sk = SecretKey::from_bytes(&bytes).unwrap();
    assert_eq!(
        bytes_to_hex(master_sk.to_bytes().to_vec()),
        expected_master_pk_hex
    );
    let master_sk = derive_keys::master_to_wallet_hardened(&master_sk, 1);
    assert_eq!(
        bytes_to_hex(master_sk.to_bytes().to_vec()),
        expected_master_sk
    );
    let syntetic = calculate_synthetic_private_key(&master_sk);
    assert_eq!(
        bytes_to_hex(syntetic.to_bytes().to_vec()),
        expected_syntetic_sk
    );
}
