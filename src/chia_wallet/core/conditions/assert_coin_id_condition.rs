use super::conditions::check_is_this_condition;
use crate::chia_wallet::core::conditions::conditions::Condition;
use crate::{chia_wallet::core::bytes::WrapperBytes, program_utils::program::Program};
pub struct AssertMyCoinIdCondition {
    coin_id: WrapperBytes,
}

impl Condition for AssertMyCoinIdCondition {
    fn program(&self) -> Program {
        let p_list = vec![
            Program::from(Self::CONDITION_CODE),
            Program::from(&self.coin_id.raw()),
        ];
        Program::from(p_list)
    }
}

impl AssertMyCoinIdCondition {
    const CONDITION_CODE: u32 = 70;

    pub fn new(coin_id: WrapperBytes) -> Self {
        AssertMyCoinIdCondition { coin_id }
    }

    pub fn from_program(program: Program) -> Result<Self, InvalidConditionCastException> {
        let program_list = program.clone().to_list();
        if !Self::is_this_condition(&program) {
            return Err(InvalidConditionCastException);
        }
        let coin_id = WrapperBytes::from_atom(program_list[1].clone());
        Ok(AssertMyCoinIdCondition { coin_id })
    }

    pub fn is_this_condition(condition: &Program) -> bool {
        check_is_this_condition(condition, Self::CONDITION_CODE)
    }
}

impl std::fmt::Display for AssertMyCoinIdCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "AssertMyCoinIdCondition(code: {}, coinId: {:?})",
            Self::CONDITION_CODE,
            self.coin_id.raw()
        )
    }
}

// Define the InvalidConditionCastException struct if not already defined
pub struct InvalidConditionCastException;
