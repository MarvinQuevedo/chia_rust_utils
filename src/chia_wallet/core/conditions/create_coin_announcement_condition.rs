use super::assert_coin_id_condition::InvalidConditionCastException;
use super::conditions::check_is_this_condition;
use crate::chia_wallet::core::conditions::conditions::Condition;
use crate::{chia_wallet::core::bytes::WrapperBytes, program_utils::program::Program};
pub struct CreateCoinAnnouncementCondition {
    announcement_hash: WrapperBytes,
}

impl Condition for CreateCoinAnnouncementCondition {
    fn program(&self) -> Program {
        let p_list = vec![
            Program::from(Self::CONDITION_CODE),
            Program::from(&self.announcement_hash.raw()),
        ];
        Program::from(p_list)
    }
}

impl CreateCoinAnnouncementCondition {
    const CONDITION_CODE: u32 = 60;

    pub fn new(announcement_hash: WrapperBytes) -> Self {
        CreateCoinAnnouncementCondition { announcement_hash }
    }

    pub fn from_program(program: Program) -> Result<Self, InvalidConditionCastException> {
        let program_list = program.clone().to_list();
        if !Self::is_this_condition(&program) {
            return Err(InvalidConditionCastException);
        }
        let announcement_hash = WrapperBytes::from_atom(program_list[1].clone());
        Ok(CreateCoinAnnouncementCondition { announcement_hash })
    }

    pub fn is_this_condition(condition: &Program) -> bool {
        return check_is_this_condition(condition, Self::CONDITION_CODE);
    }
}
