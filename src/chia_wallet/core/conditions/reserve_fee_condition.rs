use num_bigint::BigInt;

use crate::program_utils::program::Program;

use super::{
    assert_coin_id_condition::InvalidConditionCastException,
    conditions::{check_is_this_condition, Condition},
};

pub struct ReserveFeeCondition {
    fee_amount: BigInt,
}

impl ReserveFeeCondition {
    const CONDITION_CODE: u32 = 52;

    pub fn new(fee_amount: BigInt) -> Self {
        ReserveFeeCondition { fee_amount }
    }

    pub fn from_program(program: Program) -> Result<Self, InvalidConditionCastException> {
        let program_list = program.clone().to_list();
        if !Self::is_this_condition(&program) {
            return Err(InvalidConditionCastException);
        }
        let fee_amount = program_list[1].as_int().unwrap();
        Ok(ReserveFeeCondition { fee_amount })
    }

    pub fn is_this_condition(condition: &Program) -> bool {
        check_is_this_condition(condition, Self::CONDITION_CODE)
    }
}

impl Condition for ReserveFeeCondition {
    fn program(&self) -> Program {
        Program::from(vec![
            Program::from(Self::CONDITION_CODE),
            Program::from(&self.fee_amount),
        ])
    }
}
