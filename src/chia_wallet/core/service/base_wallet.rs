use chia_bls::{public_key::PublicKey, signature::Signature};

use crate::{
    blockchain::{coin::Coin, coin_spend::CoinSpend},
    chia_wallet::core::{
        blockchain_network::BlockchainNetwork,
        bytes::WrapperBytes,
        conditions::condition_opcode::ConditionOpcode,
        conditions::{
            assert_coin_announcement_condition::AssertCoinAnnouncementCondition,
            assert_puzzle_announcement_condition::AssertPuzzleAnnouncementConditionImp,
            condition_opcode::ConditionWithArgs,
            create_coin_announcement_condition::CreateCoinAnnouncementCondition,
            create_puzzle_announcement_condition::CreatePuzzleAnnouncementCondition,
        },
        conditions::{
            assert_puzzle_announcement_condition::AssertPuzzleAnnouncementCondition,
            conditions::Condition,
        },
        payment::Payment,
    },
    keyword,
    program_utils::{keywords::KEYWORDS, program::Program},
};
use std::collections::HashSet;
struct BaseWallet {
    network: BlockchainNetwork,
}

impl BaseWallet {
    pub fn check_for_duplicate_coins(coins: Vec<Coin>) -> Result<(), DuplicateCoinException> {
        let mut id_set: HashSet<String> = HashSet::new();

        for coin in coins {
            let coin_id_hex = hex::encode(&coin.name().bytes);
            if id_set.contains(&coin_id_hex) {
                return Err(DuplicateCoinException {
                    coin_id_hex: coin_id_hex.clone(),
                });
            } else {
                id_set.insert(coin_id_hex);
            }
        }

        Ok(())
    }
    pub fn make_solution_from_conditions(conditions: Vec<Box<dyn Condition>>) -> Program {
        let mut program_list = vec![Program::from(&keyword!("q"))];
        for condition in conditions {
            program_list.push(condition.program());
        }
        return Self::make_solution_from_program(Program::from(program_list));
    }
    pub fn make_solution_from_program(program: Program) -> Program {
        Program::from(vec![Program::nil(), program, Program::nil()])
    }

    pub fn parse_sexp_to_condition(
        sexp: Program,
    ) -> (Option<Exception>, Option<ConditionWithArgs>) {
        let atoms = sexp.clone().to_list();

        if atoms.is_empty() {
            return (Some(Exception("INVALID_CONDITION")), None);
        }

        match atoms.first() {
            Some(op_code_program) => {
                if let Ok(op_code_int) = op_code_program.as_int() {
                    let op_code = ConditionOpcode::from(op_code_int);
                    let bytes_vars = atoms[1..]
                        .iter()
                        .filter_map(|e| e.as_atom().and_then(|e| e.as_vec()))
                        .collect();

                    let condition_with_args = ConditionWithArgs {
                        condition_opcode: op_code,
                        vars: bytes_vars,
                    };
                    (None, Some(condition_with_args))
                } else {
                    (Some(Exception("INVALID_CONDITION")), None)
                }
            }
            None => (Some(Exception("INVALID_CONDITION")), None),
        }
    }

    pub fn parse_sexp_to_conditions(
        sexp: Program,
    ) -> (Option<Exception>, Option<Vec<ConditionWithArgs>>) {
        let mut results = Vec::new();
        let items = sexp.clone().to_list();
        if items.is_empty() {
            return (Some(Exception("INVALID_CONDITION")), None);
        }

        for item in items {
            let result = Self::parse_sexp_to_condition(item);
            if let Some(e) = result.0 {
                return (Some(e), None);
            }
            results.push(result.1.unwrap());
        }
        (None, Some(results))
    }

    fn make_solution(
        primaries: Vec<Payment>,
        coin_announcements_to_assert: Vec<AssertCoinAnnouncementCondition>,
        puzzle_announcements_to_assert: Vec<AssertPuzzleAnnouncementConditionImp>,
        coin_announcements: HashSet<WrapperBytes>,
        puzzle_announcements: HashSet<WrapperBytes>,
    ) -> Program {
        let mut conditions = Vec::new();
        if !primaries.is_empty() {
            for payment in primaries {
                let create_condition = payment.to_create_coin_condition();
                conditions.push(Box::new(create_condition) as Box<dyn Condition>);
            }
        }

        conditions.extend(coin_announcements.iter().map(|coin_announcement| {
            Box::new(CreateCoinAnnouncementCondition::new(
                coin_announcement.clone(),
            )) as Box<dyn Condition>
        }));
        conditions.extend(
            coin_announcements_to_assert
                .into_iter()
                .map(|assertion| Box::new(assertion) as Box<dyn Condition>),
        );

        conditions.extend(puzzle_announcements.iter().map(|puzzle_announcement| {
            Box::new(CreatePuzzleAnnouncementCondition::new(
                puzzle_announcement.clone(),
            )) as Box<dyn Condition>
        }));
        conditions.extend(
            puzzle_announcements_to_assert
                .into_iter()
                .map(|assertion| Box::new(assertion) as Box<dyn Condition>),
        );

        Self::make_solution_from_conditions(conditions)
    }
}

pub struct DuplicateCoinException {
    coin_id_hex: String,
}
pub struct Exception(&'static str);

struct CoinSpendAndSignature {
    coin_spend: CoinSpend,
    signature: Signature,
}

impl CoinSpendAndSignature {
    const fn new(coin_spend: CoinSpend, signature: Signature) -> Self {
        Self {
            coin_spend,
            signature,
        }
    }
}
