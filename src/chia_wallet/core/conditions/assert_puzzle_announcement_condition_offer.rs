use super::assert_puzzle_announcement_condition::{
    AssertPuzzleAnnouncementCondition, AssertPuzzleAnnouncementConditionImp,
};
use crate::{chia_wallet::core::bytes::Bytes, program_utils::program::Program};

pub struct AssertPuzzleAnnouncementConditionOffer {
    settlement_ph: Bytes,
    message: Bytes,
}

impl AssertPuzzleAnnouncementCondition for AssertPuzzleAnnouncementConditionOffer {
    fn to_assert_puzzle_announcement_condition(&self) -> AssertPuzzleAnnouncementConditionImp {
        AssertPuzzleAnnouncementConditionImp {
            announcement_hash: self.get_announcement_hash(),
        }
    }

    fn get_announcement_hash(&self) -> Bytes {
        let concat_values = [self.settlement_ph.raw(), self.message.raw()].concat();
        Bytes::from(concat_values).sha256_hash()
    }
}
impl AssertPuzzleAnnouncementConditionOffer {
    pub fn to_announcement_list(&self) -> Program {
        Program::from(vec![
            Program::from(&self.settlement_ph.raw()),
            Program::from(&self.message.raw()),
        ])
    }
    pub fn from_program_list(program: Program) -> Self {
        let list = program.clone().to_list();
        AssertPuzzleAnnouncementConditionOffer {
            settlement_ph: Bytes::from_atom(list[0].clone()),
            message: Bytes::from_atom(list[1].clone()),
        }
    }
}
