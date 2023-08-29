use std::collections::HashMap;

use crate::blockchain::coin::Coin;
use crate::chia_wallet::core::conditions::condition_opcode::{ConditionOpcode, ConditionWithArgs};
use crate::chia_wallet::core::service::base_wallet::BaseWallet;
use crate::program_utils::serialized_program::SerializedProgram;
use chia_utils_streamable_macro::sized_bytes::Bytes32;
use num_bigint::BigInt;

pub fn created_outputs_for_conditions_dict(
    conditions_dict: HashMap<ConditionOpcode, Vec<ConditionWithArgs>>,
    input_coin_name: Bytes32,
) -> Vec<Coin> {
    let mut output_coins = Vec::new();
    match conditions_dict.get(&ConditionOpcode::CREATE_COIN) {
        Some(args) => {
            for cvp in args {
                let puz_hash = cvp.vars[0].clone();
                let amount = atom_to_int(&cvp.vars[1]).try_into().unwrap();
                let coin = Coin::new(amount, input_coin_name.clone(), puz_hash.into());
                output_coins.push(coin);
            }
        }
        None => {}
    }
    output_coins
}

pub fn additions_for_solution(
    coin_name: Bytes32,
    puzzle_reveal: &SerializedProgram,
    solution: &SerializedProgram,
    max_cost: u64,
) -> Vec<Coin> {
    let r =
        BaseWallet::conditions_dict_for_solution(puzzle_reveal.clone(), solution.clone(), max_cost);
    let mut map: HashMap<ConditionOpcode, Vec<ConditionWithArgs>> = HashMap::new();
    match r.1 {
        Some(value) => map = value,
        None => {}
    };

    let coins = created_outputs_for_conditions_dict(map, coin_name);
    coins
}

pub fn fee_for_solution(
    puzzle_reveal: &SerializedProgram,
    solution: &SerializedProgram,
    max_cost: u64,
) -> BigInt {
    let r =
        BaseWallet::conditions_dict_for_solution(puzzle_reveal.clone(), solution.clone(), max_cost);
    let mut conditions: HashMap<ConditionOpcode, Vec<ConditionWithArgs>> = HashMap::new();
    match r.1 {
        Some(value) => conditions = value,
        None => {}
    };
    let mut total: BigInt = 0.into();
    match conditions.get(&ConditionOpcode::RESERVE_FEE) {
        Some(conditions) => {
            for cond in conditions {
                total += atom_to_int(&cond.vars[0]);
            }
        }
        None => {
            total = 0.into();
        }
    }
    total
}

pub fn atom_to_int(bytes: &Vec<u8>) -> BigInt {
    let len = bytes.len();
    if len == 0 {
        0.into()
    } else {
        BigInt::from_signed_bytes_be(bytes)
    }
}
