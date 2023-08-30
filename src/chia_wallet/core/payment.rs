use num::ToPrimitive;
use num_bigint::BigInt;

use crate::program_utils::program::Program;

use super::{
    bytes::{Bytes, Puzzlehash},
    conditions::create_coin_condition::CreateCoinCondition,
};

pub struct Payment {
    pub amount: BigInt,
    pub puzzle_hash: Puzzlehash,
    pub memos: Option<Vec<Bytes>>,
}

impl Payment {
    pub fn new(amount: BigInt, puzzlehash: Puzzlehash, memos: Option<Vec<Bytes>>) -> Self {
        Payment {
            amount,
            puzzle_hash: puzzlehash,
            memos,
        }
    }
    pub fn amount_u64(&self) -> u64 {
        self.amount.to_u64().unwrap()
    }
    pub fn to_create_coin_condition(&self) -> CreateCoinCondition {
        CreateCoinCondition::new(
            self.puzzle_hash.clone(),
            self.amount.clone(),
            self.memos.clone(),
        )
    }
    pub fn to_program(&self) -> Program {
        let mut program_list = vec![
            Program::from(&self.puzzle_hash.to_bytes().raw()),
            Program::from(&self.amount),
        ];

        if let Some(memos) = &self.memos {
            let items: Vec<Program> = memos.iter().map(|memo| Program::from(memo.raw())).collect();
            program_list.push(Program::from(items));
        }

        Program::from(program_list)
    }
    pub fn from_program(program: Program) -> Self {
        let program_list = program.clone().to_list();
        let memos = program_list.get(2).map(|p| {
            p.clone()
                .as_atom_list()
                .iter()
                .cloned()
                .map(Bytes::from)
                .collect()
        });

        Payment {
            amount: program_list[1].as_int().unwrap(),
            puzzle_hash: Puzzlehash::from_atom(program_list[0].clone()),
            memos,
        }
    }
}

impl Clone for Payment {
    fn clone(&self) -> Self {
        Payment {
            amount: self.amount.clone(),
            puzzle_hash: self.puzzle_hash.clone(),
            memos: self.memos.clone(),
        }
    }
}
