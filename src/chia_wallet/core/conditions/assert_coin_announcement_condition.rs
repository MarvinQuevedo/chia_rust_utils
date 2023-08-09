use crate::chia_wallet::core::conditions::conditions::Condition;
use crate::{chia_wallet::core::bytes::WrapperBytes, program_utils::program::Program};

struct AssertCoinAnnouncementCondition {
    coin_id: WrapperBytes,
    message: WrapperBytes,
    morph_bytes: Option<WrapperBytes>,
}

impl Condition for AssertCoinAnnouncementCondition {
    fn program(&self) -> Program {
        let list_data = [
            Program::from(AssertCoinAnnouncementCondition::CONDITION_CODE),
            Program::from(self.announcement_id().raw()),
        ]
        .to_vec();

        return Program::from(list_data);
    }
}

impl AssertCoinAnnouncementCondition {
    const CONDITION_CODE: i32 = 61;

    pub fn announcement_id(&self) -> WrapperBytes {
        if let Some(morph_bytes) = &self.morph_bytes {
            let prefixed_message =
                WrapperBytes::from([morph_bytes.raw(), self.message.raw()].concat()).sha256_hash();
            return WrapperBytes::from([self.coin_id.raw(), prefixed_message.raw()].concat())
                .sha256_hash();
        } else {
            return WrapperBytes::from([self.coin_id.raw(), self.message.raw()].concat())
                .sha256_hash();
        }
    }

    fn program_list(&self) -> Program {
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
        let coin_id = WrapperBytes::from_atom(program_list[0].clone());
        let message = WrapperBytes::from_atom(program_list[1].clone());
        let morph_bytes = if program_list[2].is_null() {
            None
        } else {
            Some(WrapperBytes::from_atom(program_list[2].clone()))
        };
        AssertCoinAnnouncementCondition {
            coin_id,
            message,
            morph_bytes,
        }
    }
}
