use num_bigint::BigInt;

use crate::{
    chia_wallet::core::bytes_utils::{bytes_to_int, int_to_bytes, u8_to_bytes, Endian},
    program_utils::program::Program,
};

pub trait Condition {
    fn program(&self) -> Program;
}

pub fn check_is_this_condition(condition: &Program, condition_code: u32) -> bool {
    check_is_this_condition_with_parts_len(condition, condition_code, 2)
}
pub fn check_is_this_condition_with_three_parts(condition: &Program, condition_code: u32) -> bool {
    check_is_this_condition_with_parts_len(condition, condition_code, 3)
}
pub fn check_is_this_condition_with_parts_len(
    condition: &Program,
    condition_code: u32,
    parts_len: usize,
) -> bool {
    let condition_parts = condition.clone().to_list();

    if condition_parts.len() != parts_len {
        return false;
    }
    let condition_code_bytes = condition_code.to_be_bytes().to_vec();
    let condition_bigint = BigInt::from_signed_bytes_be(condition_code_bytes.as_slice());
    let condition_number = condition_parts[0].as_int().unwrap();

    if condition_number != condition_bigint {
        return false;
    }

    true
}

pub fn conditions_to_program(conditions: Vec<Box<dyn Condition>>) -> Program {
    let program_vec: Vec<Program> = conditions
        .iter()
        .map(|condition| condition.program().clone())
        .collect();
    Program::from(program_vec)
}
