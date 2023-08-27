use crate::{
    blockchain::{coin::Coin, coin_spend::CoinSpend},
    chia_wallet::core::{
        blockchain_network::BlockchainNetwork,
        bytes::{Puzzlehash, WrapperBytes},
        conditions::condition_opcode::ConditionOpcode,
        conditions::{agg_sig_me_condition::AggSigMeCondition, conditions::Condition},
        conditions::{
            assert_coin_announcement_condition::AssertCoinAnnouncementCondition,
            assert_puzzle_announcement_condition::AssertPuzzleAnnouncementConditionImp,
            condition_opcode::ConditionWithArgs,
            create_coin_announcement_condition::CreateCoinAnnouncementCondition,
            create_puzzle_announcement_condition::CreatePuzzleAnnouncementCondition,
        },
        keywords::keyword,
        payment::Payment,
    },
    program_utils::{
        node::Node, program::Program, serialize::node_to_bytes,
        serialized_program::SerializedProgram,
    },
};
use chia_bls::signature::Signature;
use clvmr::allocator::Allocator;
use num_bigint::BigInt;
use std::collections::HashMap;
use std::collections::HashSet;
pub struct BaseWallet {
    network: BlockchainNetwork,
}

impl BaseWallet {
    pub fn get_add_sig_me_message_from_result(
        result: Program,
        coin: Coin,
        network: BlockchainNetwork,
    ) -> WrapperBytes {
        let all_items = result.clone().to_list();

        let agg_sig_me_condition =
            all_items
                .iter()
                .filter_map(|e| match AggSigMeCondition::is_this_condition(e) {
                    true => Some(e.clone()),
                    false => None,
                });

        let agg_sig_me_condition = agg_sig_me_condition.collect::<Vec<Program>>()[2]
            .as_vec()
            .unwrap();

        let id = coin.name().bytes;
        let extra_data = WrapperBytes::from_hex(&network.agg_sig_me_extra_data)
            .unwrap()
            .raw();
        let mut message = Vec::new();

        message.extend_from_slice(&agg_sig_me_condition);
        message.extend_from_slice(&id);
        message.extend_from_slice(&extra_data);

        WrapperBytes::from(message)
    }
    pub fn conditions_for_solution(
        puzzle_reveal: SerializedProgram,
        solution: SerializedProgram,
        max_cost: u64,
    ) -> (Option<Exception>, Option<Vec<ConditionWithArgs>>, BigInt) {
        let mut allocator = Allocator::new();
        match puzzle_reveal.run_with_cost(
            &mut allocator,
            max_cost,
            &Program::new(solution.to_bytes()),
        ) {
            Ok((cost, r)) => {
                let node = Node::new(&mut allocator, r);
                match node_to_bytes(&node) {
                    Ok(byte_data) => {
                        let serial_program = SerializedProgram::from_bytes(&byte_data);
                        let parsed =
                            Self::parse_sexp_to_conditions(serial_program.to_program().unwrap());
                        (parsed.0, parsed.1, BigInt::from(cost))
                    }
                    Err(_) => (Some(Exception("INVALID_SOLUTION")), None, BigInt::from(0)),
                }
            }
            Err(_) => (Some(Exception("INVALID_SOLUTION")), None, BigInt::from(0)),
        }
    }

    pub fn conditions_dict_for_solution(
        puzzle_reveal: SerializedProgram,
        solution: SerializedProgram,
        max_cost: u64,
    ) -> (
        Option<Exception>,
        Option<HashMap<ConditionOpcode, Vec<ConditionWithArgs>>>,
        BigInt,
    ) {
        let result = Self::conditions_for_solution(puzzle_reveal, solution, max_cost);
        if let Some(e) = result.0 {
            return (Some(e), None, result.2);
        }
        let dict_result = Self::conditions_by_opcode(result.1.unwrap());
        (None, Some(dict_result), result.2)
    }
    pub fn conditions_by_opcode(
        conditions: Vec<ConditionWithArgs>,
    ) -> HashMap<ConditionOpcode, Vec<ConditionWithArgs>> {
        let mut hm: HashMap<ConditionOpcode, Vec<ConditionWithArgs>> = HashMap::new();
        for cvp in conditions {
            match hm.get_mut(&cvp.condition_opcode) {
                Some(list) => {
                    list.push(cvp.clone());
                }
                None => {
                    hm.insert(cvp.condition_opcode.clone(), vec![cvp.clone()]);
                }
            }
        }
        return hm;
    }
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
        let mut program_list = vec![Program::from(keyword("q"))];
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

    pub fn make_solution(
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

#[test]
fn test_make_solution() {
    let launcher_id =
        WrapperBytes::from_hex("c8109361adf2cd32c07587312052ddbc8bf61eb4644fd6351e1cf1f814f272fb")
            .unwrap()
            .as_puzzlehash();
    let eve_full_puz = Program::from_source("(a (q #a 4 (c 2 (c 5 (c 7 0)))) (c (q (c (q . 2) (c (c (q . 1) 5) (c (a 6 (c 2 (c 11 (q 1)))) 0))) #a (i 5 (q 4 (q . 4) (c (c (q . 1) 9) (c (a 6 (c 2 (c 13 (c 11 0)))) 0))) (q . 11)) 1) 1))");
    let announcement_message = Program::from(vec![
        Program::from(eve_full_puz.tree_hash()),
        Program::from(1),
        Program::from(Vec::<Program>::new()),
    ])
    .tree_hash();
    let assert_coin_announcement = AssertCoinAnnouncementCondition::new(
        launcher_id.to_bytes(),
        WrapperBytes::from(announcement_message.clone()),
    );
    let assert_puzzle =
        AssertPuzzleAnnouncementConditionImp::new(WrapperBytes::from(announcement_message.clone()));
    let ph =
        Puzzlehash::from_hex(&"e30a9dc6c0379a72d77afa8d596a91399f9d18dbe5a87168b7a9b5381596b18c")
            .unwrap();
    let ph2 =
        Puzzlehash::from_hex(&"e30a9dc6c0379a72d77afa8d596a91399f9d18dbe5a87168b7a9b5381596b18a")
            .unwrap();
    let ph3 =
        Puzzlehash::from_hex(&"e30a9dc6c0379a72d77afa8d596a91399f9d18dbe5a87168b7a9b5381596b120")
            .unwrap();
    let ph4 =
        Puzzlehash::from_hex(&"e30a9dc6c0379a72d77afa8d596a91399f9d18dbe5a87168b7a9b5381596b112")
            .unwrap();
    let mut coin_announcements = HashSet::new();
    coin_announcements.insert(ph.to_bytes());

    let mut puzzle_announcements = HashSet::new();

    puzzle_announcements.insert(ph3.to_bytes());

    let primaries = vec![Payment::new(BigInt::from(100), ph, None)];
    let solution = BaseWallet::make_solution(
        primaries,
        vec![assert_coin_announcement],
        vec![assert_puzzle],
        coin_announcements,
        puzzle_announcements,
    );
    let equal_program =  "(() (q (63 0xe30a9dc6c0379a72d77afa8d596a91399f9d18dbe5a87168b7a9b5381596b18c 100) (60 0xe30a9dc6c0379a72d77afa8d596a91399f9d18dbe5a87168b7a9b5381596b18c) (61 0xb334ae7173a5e65e0a380952707904ae22fd097e11d5101f2c1876b74f5ca33e) (62 0xe30a9dc6c0379a72d77afa8d596a91399f9d18dbe5a87168b7a9b5381596b120) (63 0x3659e109ac94549c9393f3bcdfd950cc63c4c90c838afe822f64ba3d9167de00)) ())";

    assert_eq!(solution.to_source(None), equal_program);
}
