use crate::chia_wallet::core::bytes::Bytes;
use crate::chia_wallet::core::conditions::conditions::Condition;
use crate::program_utils::program::Program;

use super::conditions::check_is_this_condition_with_parts_len;

pub struct CreatePuzzleAnnouncementCondition {
    message: Bytes,
}
impl Condition for CreatePuzzleAnnouncementCondition {
    fn program(&self) -> Program {
        Program::from(vec![
            Program::from(Self::CONDITION_CODE),
            Program::from(&self.message.raw()),
        ])
    }
}

impl CreatePuzzleAnnouncementCondition {
    const CONDITION_CODE: u32 = 62;

    pub fn new(message: Bytes) -> Self {
        CreatePuzzleAnnouncementCondition { message }
    }

    pub fn is_this_condition(condition: &Program) -> bool {
        check_is_this_condition_with_parts_len(condition, Self::CONDITION_CODE, 3)
    }
}
