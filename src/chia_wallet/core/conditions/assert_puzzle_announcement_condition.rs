use super::assert_coin_id_condition::InvalidConditionCastException;
use super::conditions::check_is_this_condition;
use crate::chia_wallet::core::conditions::conditions::Condition;
use crate::{chia_wallet::core::bytes::Bytes, program_utils::program::Program};

pub trait AssertPuzzleAnnouncementCondition {
    fn get_announcement_hash(&self) -> Bytes;
    fn to_assert_puzzle_announcement_condition(&self) -> AssertPuzzleAnnouncementConditionImp;
}
pub struct AssertPuzzleAnnouncementConditionImp {
    pub announcement_hash: Bytes,
}
impl AssertPuzzleAnnouncementCondition for AssertPuzzleAnnouncementConditionImp {
    fn to_assert_puzzle_announcement_condition(&self) -> AssertPuzzleAnnouncementConditionImp {
        AssertPuzzleAnnouncementConditionImp {
            announcement_hash: self.announcement_hash.clone(),
        }
    }

    fn get_announcement_hash(&self) -> Bytes {
        self.announcement_hash.clone()
    }
}

impl Condition for AssertPuzzleAnnouncementConditionImp {
    fn program(&self) -> Program {
        let p_list = vec![
            Program::from(Self::CONDITION_CODE),
            Program::from(&self.announcement_hash.raw()),
        ];
        Program::from(p_list)
    }
}

impl Clone for AssertPuzzleAnnouncementConditionImp {
    fn clone(&self) -> Self {
        AssertPuzzleAnnouncementConditionImp {
            announcement_hash: self.announcement_hash.clone(),
        }
    }
}

impl AssertPuzzleAnnouncementConditionImp {
    const CONDITION_CODE: u32 = 63;

    pub fn new(announcement_hash: Bytes) -> Self {
        AssertPuzzleAnnouncementConditionImp { announcement_hash }
    }

    pub fn from_program(program: Program) -> Result<Self, InvalidConditionCastException> {
        let program_list = program.clone().to_list();
        if !Self::is_this_condition(&program) {
            return Err(InvalidConditionCastException);
        }
        let announcement_hash = Bytes::from_atom(program_list[1].clone());
        Ok(AssertPuzzleAnnouncementConditionImp { announcement_hash })
    }

    pub fn is_this_condition(condition: &Program) -> bool {
        return check_is_this_condition(condition, Self::CONDITION_CODE);
    }
}
