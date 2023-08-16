use num_bigint::BigInt;

use crate::program_utils::program::Program;

use super::{
    bytes::{Puzzlehash, WrapperBytes},
    conditions::create_coin_condition::CreateCoinCondition,
};

pub struct Payment {
    amount: BigInt,
    destination_puzzlehash: Puzzlehash,
    memos: Option<Vec<WrapperBytes>>,
}

impl Payment {
    pub fn new(
        amount: BigInt,
        destination_puzzlehash: Puzzlehash,
        memos: Option<Vec<WrapperBytes>>,
    ) -> Self {
        Payment {
            amount,
            destination_puzzlehash,
            memos,
        }
    }
    pub fn to_create_coin_condition(&self) -> CreateCoinCondition {
        CreateCoinCondition::new(
            self.destination_puzzlehash.clone(),
            self.amount.clone(),
            self.memos.clone(),
        )
    }
    pub fn to_program(&self) -> Program {
        let mut program_list = vec![
            Program::from(&self.destination_puzzlehash.to_bytes().raw()),
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
                .map(WrapperBytes::from)
                .collect()
        });

        Payment {
            amount: program_list[1].as_int().unwrap(),
            destination_puzzlehash: Puzzlehash::from_atom(program_list[0].clone()),
            memos,
        }
    }
}
