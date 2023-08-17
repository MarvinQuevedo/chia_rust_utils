use num_bigint::BigInt;

use super::assert_coin_id_condition::InvalidConditionCastException;
use super::conditions::check_is_this_condition_with_parts_len;
use crate::chia_wallet::core::bytes::Puzzlehash;
use crate::chia_wallet::core::conditions::conditions::Condition;
use crate::chia_wallet::core::payment::Payment;
use crate::{chia_wallet::core::bytes::WrapperBytes, program_utils::program::Program};
pub struct CreateCoinCondition {
    destination_puzzlehash: Puzzlehash,
    amount: BigInt,
    memos: Option<Vec<WrapperBytes>>,
}

impl Condition for CreateCoinCondition {
    fn program(&self) -> Program {
        let mut program_list = vec![
            Program::from(Self::CONDITION_CODE),
            Program::from(&self.destination_puzzlehash.to_bytes().raw()),
            Program::from(&self.amount),
        ];

        if let Some(memos) = &self.memos {
            let memo_programs: Vec<Program> =
                memos.iter().map(|memo| Program::from(memo.raw())).collect();

            program_list.push(Program::from(memo_programs));
        }

        Program::from(program_list)
    }
}

impl CreateCoinCondition {
    const CONDITION_CODE: u32 = 63;

    pub fn new(
        destination_puzzlehash: Puzzlehash,
        amount: BigInt,
        memos: Option<Vec<WrapperBytes>>,
    ) -> Self {
        CreateCoinCondition {
            destination_puzzlehash,
            amount,
            memos,
        }
    }

    pub fn from_program(program: Program) -> Result<Self, InvalidConditionCastException> {
        let mut program_list = program.clone().to_list();
        if !Self::is_this_condition(&program) {
            return Err(InvalidConditionCastException);
        }

        let destination_puzzlehash =
            Puzzlehash::from(WrapperBytes::from_atom(program_list[1].clone()));
        let amount = program_list[2].as_int().unwrap();
        let memos = if program_list.len() > 3 {
            Some(
                program_list[3]
                    .to_list()
                    .into_iter()
                    .map(|memo| WrapperBytes::from_atom(memo))
                    .collect(),
            )
        } else {
            None
        };

        Ok(CreateCoinCondition {
            destination_puzzlehash,
            amount,
            memos,
        })
    }

    pub fn to_payment(&self) -> Payment {
        Payment::new(
            self.amount.clone(),
            self.destination_puzzlehash.clone(),
            self.memos.clone(),
        )
    }

    pub fn is_this_condition(condition: &Program) -> bool {
        check_is_this_condition_with_parts_len(condition, Self::CONDITION_CODE, 3)
    }
}