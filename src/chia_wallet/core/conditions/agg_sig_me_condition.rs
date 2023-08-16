use chia_bls::public_key::PublicKey;

use crate::{chia_wallet::core::bytes::WrapperBytes, program_utils::program::Program};

use super::{
    assert_coin_id_condition::InvalidConditionCastException,
    conditions::{check_is_this_condition_with_three_parts, Condition},
};

pub struct AggSigMeCondition {
    public_key: PublicKey,
    message: WrapperBytes,
}

impl AggSigMeCondition {
    const CONDITION_CODE: u32 = 50;

    pub fn new(public_key: PublicKey, message: WrapperBytes) -> Self {
        AggSigMeCondition {
            public_key,
            message,
        }
    }

    pub fn from_program(program: Program) -> Result<Self, InvalidConditionCastException> {
        let program_list = program.clone().to_list();
        if !Self::is_this_condition(&program) {
            return Err(InvalidConditionCastException);
        }
        let public_key_bytes = program_list[1].as_vec().unwrap();
        let formated_bytes: [u8; 48] = public_key_bytes[..48].try_into().unwrap();
        let public_key = PublicKey::from_bytes(&formated_bytes).unwrap();
        let message = WrapperBytes::from_atom(program_list[2].clone());
        Ok(AggSigMeCondition {
            public_key,
            message,
        })
    }

    pub fn is_this_condition(condition: &Program) -> bool {
        check_is_this_condition_with_three_parts(condition, Self::CONDITION_CODE)
    }
}

impl Condition for AggSigMeCondition {
    fn program(&self) -> Program {
        Program::from(vec![
            Program::from(Self::CONDITION_CODE),
            Program::from(self.public_key.to_bytes().to_vec()),
            Program::from(self.message.raw()),
        ])
    }
}
