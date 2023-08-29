use num_bigint::BigInt;

use super::assert_coin_id_condition::InvalidConditionCastException;
use super::conditions::check_is_this_condition_with_parts_len;
use crate::chia_wallet::core::bytes::Puzzlehash;
use crate::chia_wallet::core::conditions::conditions::Condition;
use crate::chia_wallet::core::payment::Payment;
use crate::{chia_wallet::core::bytes::Bytes, program_utils::program::Program};
pub struct CreateCoinCondition {
    puzzle_hash: Puzzlehash,
    amount: BigInt,
    memos: Option<Vec<Bytes>>,
}

impl Condition for CreateCoinCondition {
    fn program(&self) -> Program {
        let mut program_list = vec![
            Program::from(Self::CONDITION_CODE),
            Program::from(&self.puzzle_hash.to_bytes().raw()),
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

    pub fn new(puzzle_hash: Puzzlehash, amount: BigInt, memos: Option<Vec<Bytes>>) -> Self {
        CreateCoinCondition {
            puzzle_hash,
            amount,
            memos,
        }
    }

    pub fn from_program(program: Program) -> Result<Self, InvalidConditionCastException> {
        let mut program_list = program.clone().to_list();
        if !Self::is_this_condition(&program) {
            return Err(InvalidConditionCastException);
        }

        let destination_puzzlehash = Puzzlehash::from(Bytes::from_atom(program_list[1].clone()));
        let amount = program_list[2].as_int().unwrap();
        let memos = if program_list.len() > 3 {
            Some(
                program_list[3]
                    .to_list()
                    .into_iter()
                    .map(|memo| Bytes::from_atom(memo))
                    .collect(),
            )
        } else {
            None
        };

        Ok(CreateCoinCondition {
            puzzle_hash: destination_puzzlehash,
            amount,
            memos,
        })
    }

    pub fn to_payment(&self) -> Payment {
        Payment::new(
            self.amount.clone(),
            self.puzzle_hash.clone(),
            self.memos.clone(),
        )
    }

    pub fn is_this_condition(condition: &Program) -> bool {
        check_is_this_condition_with_parts_len(condition, Self::CONDITION_CODE, 3)
    }
}
