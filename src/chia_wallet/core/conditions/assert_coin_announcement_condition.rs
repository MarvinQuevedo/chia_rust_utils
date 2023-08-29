use crate::chia_wallet::core::conditions::conditions::Condition;
use crate::{chia_wallet::core::bytes::Bytes, program_utils::program::Program};

use super::conditions::check_is_this_condition;

pub struct AssertCoinAnnouncementCondition {
    pub coin_id: Bytes,
    pub message: Bytes,
    pub morph_bytes: Option<Bytes>,
}

impl Condition for AssertCoinAnnouncementCondition {
    fn program(&self) -> Program {
        let list_data = [
            Program::from(Self::CONDITION_CODE),
            Program::from(self.announcement_id().raw()),
        ]
        .to_vec();

        return Program::from(list_data);
    }
}
impl Clone for AssertCoinAnnouncementCondition {
    fn clone(&self) -> Self {
        AssertCoinAnnouncementCondition {
            coin_id: self.coin_id.clone(),
            message: self.message.clone(),
            morph_bytes: self.morph_bytes.clone(),
        }
    }
}

impl AssertCoinAnnouncementCondition {
    const CONDITION_CODE: u32 = 61;
    pub fn new(coin_id: Bytes, message: Bytes) -> Self {
        AssertCoinAnnouncementCondition {
            coin_id,
            message,
            morph_bytes: None,
        }
    }

    pub fn announcement_id(&self) -> Bytes {
        if let Some(morph_bytes) = &self.morph_bytes {
            let prefixed_message =
                Bytes::from([morph_bytes.raw(), self.message.raw()].concat()).sha256_hash();
            return Bytes::from([self.coin_id.raw(), prefixed_message.raw()].concat())
                .sha256_hash();
        } else {
            return Bytes::from([self.coin_id.raw(), self.message.raw()].concat()).sha256_hash();
        }
    }

    pub fn program_list(&self) -> Program {
        if let Some(morph_bytes) = &self.morph_bytes {
            let program_vec = [
                Program::from(self.coin_id.raw()),
                Program::from(self.message.raw()),
                Program::from(morph_bytes.raw()),
            ]
            .to_vec();
            return Program::from(program_vec);
        } else {
            let program_vec = [
                Program::from(self.coin_id.raw()),
                Program::from(self.message.raw()),
                Program::null(),
            ]
            .to_vec();
            return Program::from(program_vec);
        }
    }
    pub fn from_program_list(mut program: Program) -> Self {
        let program_list = program.to_list();
        let coin_id = Bytes::from_atom(program_list[0].clone());
        let message = Bytes::from_atom(program_list[1].clone());
        let morph_bytes = if program_list[2].is_null() {
            None
        } else {
            Some(Bytes::from_atom(program_list[2].clone()))
        };
        AssertCoinAnnouncementCondition {
            coin_id,
            message,
            morph_bytes,
        }
    }

    pub fn is_this_condition(condition: &Program) -> bool {
        check_is_this_condition(condition, Self::CONDITION_CODE)
    }
}
