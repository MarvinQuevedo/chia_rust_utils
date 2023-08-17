use num::ToPrimitive;
use num_bigint::BigInt;

use crate::{
    chia_wallet::core::bytes_utils::{int_to_bytes, Endian},
    program_utils::program::Program,
};

pub struct ConditionWithArgs {
    pub condition_opcode: ConditionOpcode,
    pub vars: Vec<Vec<u8>>,
}

impl Clone for ConditionWithArgs {
    fn clone(&self) -> Self {
        ConditionWithArgs {
            condition_opcode: self.condition_opcode.clone(),
            vars: self.vars.clone(),
        }
    }
}

#[derive(Clone, Hash)]
pub struct ConditionOpcode(u8);

impl ConditionOpcode {
    // AGG_SIG is ascii "1"

    // the conditions below require bls12-381 signatures

    const AGG_SIG_UNSAFE: ConditionOpcode = ConditionOpcode::from_int(49);
    const AGG_SIG_ME: ConditionOpcode = ConditionOpcode::from_int(50);

    // the conditions below reserve coin amounts and have to be accounted for in output totals

    const CREATE_COIN: ConditionOpcode = ConditionOpcode::from_int(51);
    const RESERVE_FEE: ConditionOpcode = ConditionOpcode::from_int(52);

    // the conditions below deal with announcements, for inter-coin communication

    const CREATE_COIN_ANNOUNCEMENT: ConditionOpcode = ConditionOpcode::from_int(60);
    const ASSERT_COIN_ANNOUNCEMENT: ConditionOpcode = ConditionOpcode::from_int(61);
    const CREATE_PUZZLE_ANNOUNCEMENT: ConditionOpcode = ConditionOpcode::from_int(62);
    const ASSERT_PUZZLE_ANNOUNCEMENT: ConditionOpcode = ConditionOpcode::from_int(63);

    // the conditions below let coins inquire about themselves

    const ASSERT_MY_COIN_ID: ConditionOpcode = ConditionOpcode::from_int(70);
    const ASSERT_MY_PARENT_ID: ConditionOpcode = ConditionOpcode::from_int(71);
    const ASSERT_MY_PUZZLEHASH: ConditionOpcode = ConditionOpcode::from_int(72);
    const ASSERT_MY_AMOUNT: ConditionOpcode = ConditionOpcode::from_int(73);

    // the conditions below ensure that we're "far enough" in the future

    // wall-clock time
    const ASSERT_SECONDS_RELATIVE: ConditionOpcode = ConditionOpcode::from_int(80);
    const ASSERT_SECONDS_ABSOLUTE: ConditionOpcode = ConditionOpcode::from_int(81);

    // block index
    const ASSERT_HEIGHT_RELATIVE: ConditionOpcode = ConditionOpcode::from_int(82);
    const ASSERT_HEIGHT_ABSOLUTE: ConditionOpcode = ConditionOpcode::from_int(83);

    const fn from_int(value: u8) -> Self {
        ConditionOpcode(value)
    }

    fn to_int(&self) -> u8 {
        self.0
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        int_to_bytes(self.0.into(), 2, Endian::Big, false)
    }
}

impl From<BigInt> for ConditionOpcode {
    fn from(value: BigInt) -> Self {
        ConditionOpcode::from_int(value.to_u8().unwrap())
    }
}

impl From<Program> for ConditionOpcode {
    fn from(value: Program) -> Self {
        ConditionOpcode::from(value.as_int().unwrap())
    }
}

impl PartialEq for ConditionOpcode {
    fn eq(&self, other: &Self) -> bool {
        self.to_int() == other.to_int()
    }
}

impl Eq for ConditionOpcode {}

impl std::fmt::Display for ConditionOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConditionOpcode({})", self.to_int())
    }
}

impl std::fmt::Debug for ConditionOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConditionOpcode({})", self.to_int())
    }
}
