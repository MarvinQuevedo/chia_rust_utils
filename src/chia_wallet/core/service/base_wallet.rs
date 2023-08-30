use crate::{
    chia_wallet::{
        core::{
            blockchain_network::BlockchainNetwork,
            bytes::{Bytes, Puzzlehash, ZEROS_BYTES96},
            conditions::condition_opcode::ConditionOpcode,
            conditions::{agg_sig_me_condition::AggSigMeCondition, conditions::Condition},
            conditions::{
                assert_coin_announcement_condition::AssertCoinAnnouncementCondition,
                assert_puzzle_announcement_condition::AssertPuzzleAnnouncementConditionImp,
                condition_opcode::ConditionWithArgs,
                create_coin_announcement_condition::CreateCoinAnnouncementCondition,
                create_coin_condition::CreateCoinCondition,
                create_puzzle_announcement_condition::CreatePuzzleAnnouncementCondition,
                reserve_fee_condition::ReserveFeeCondition,
            },
            keychain::WalletKeychain,
            keywords::keyword,
            payment::Payment,
        },
        standart::puzzles::{
            calculate_synthetic_public_key::calculate_synthetic_private_key,
            p2_delegated_puzzle_or_hidden_puzzle::get_puzzle_from_pk,
        },
    },
    program_utils::{
        bls_bindings::AugSchemeMPL, node::Node, program::Program, serialize::node_to_bytes,
        serialized_program::SerializedProgram,
    },
};
use chia_bls::{signature::Signature, SecretKey};

use chia_protocol::{Bytes32, Coin, CoinSpend, SpendBundle};
use chia_utils_streamable_macro::bytes_utils::bytes_to_sha256;
use clvmr::allocator::Allocator;
use num::ToPrimitive;
use num_bigint::BigInt;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct BaseWallet {
    network: BlockchainNetwork,
}
pub trait WalletDecoration {
    fn make_solution_from_conditions(&self, conditions: Vec<Box<dyn Condition>>) -> Program;
    fn make_puzzle_reveal_from_puzzlehash(
        &self,
        puzzle_hash: Puzzlehash,
        keychain: WalletKeychain,
    ) -> Program;
    fn transform_standard_solution(&self, solution: Program) -> Program;
    fn make_signature_for_coin_spend(
        &self,
        coin_spend: CoinSpend,
        keychain: WalletKeychain,
    ) -> chia_bls::Signature;
}
impl<'a> WalletDecoration for &'a BaseWallet {
    fn make_solution_from_conditions(&self, conditions: Vec<Box<dyn Condition>>) -> Program {
        BaseWallet::make_solution_from_conditions(conditions)
    }
    fn make_puzzle_reveal_from_puzzlehash(
        &self,
        puzzle_hash: Puzzlehash,
        keychain: WalletKeychain,
    ) -> Program {
        let vector = keychain.get_wallet_vector(puzzle_hash);
        match vector {
            Some(vector) => {
                let pk = vector.child_public_key();
                get_puzzle_from_pk(pk)
            }
            None => {
                panic!("PuzzleHashNotInKeychain");
            }
        }
    }

    fn transform_standard_solution(&self, solution: Program) -> Program {
        BaseWallet::make_solution_from_program(solution)
    }
    fn make_signature_for_coin_spend(
        &self,
        coin_spend: CoinSpend,
        keychain: WalletKeychain,
    ) -> chia_bls::Signature {
        panic!("NotImplemented");
    }
}

impl BaseWallet {
    pub fn new(network: BlockchainNetwork) -> Self {
        BaseWallet { network }
    }

    fn create_unsigned_spend_bundle_base(
        &self,
        payments: Vec<Payment>,
        coins_input: Vec<Coin>,
        change_puzzlehash: Puzzlehash,
        fee: BigInt,
        origin_id: Option<Bytes32>,
        coin_announcements_to_assert: Vec<AssertCoinAnnouncementCondition>,
        puzzle_announcements_to_assert: Vec<AssertPuzzleAnnouncementConditionImp>,
        make_puzzle_reveal_from_puzzlehash: fn(Puzzlehash) -> Program,
        transform_standard_solution: Option<fn(Program) -> Program>,
    ) -> SpendBundle {
        let fee_amount = fee.clone().to_u64().unwrap();

        let mut coins = coins_input.clone();
        let total_coin_value = coins
            .iter()
            .fold(0, |previous_value, coin| previous_value + coin.amount);

        let total_payment_amount = payments.iter().fold(0, |previous_value, payment| {
            previous_value + payment.amount.to_u64().unwrap()
        });
        let change = (total_coin_value - total_payment_amount - fee)
            .to_u64()
            .unwrap();

        let change_puzzle_hash = change_puzzlehash.clone();

        let mut spends = Vec::<CoinSpend>::new();

        let origin_index: usize = if let Some(origin_id) = origin_id {
            coins
                .iter()
                .position(|coin| coin.coin_id() == origin_id)
                .unwrap_or(usize::MAX)
        } else {
            0
        };
        if origin_index == usize::MAX {
            panic!("OriginIdNotInCoinsException");
        }

        if origin_index != 0 {
            let origin_coin = coins.remove(origin_index);
            coins.insert(0, origin_coin);
        }

        let mut primary_assert_coin_announcement: Option<Box<AssertCoinAnnouncementCondition>> =
            None;

        let mut first = true;
        for coin in coins.clone() {
            let solution;
            if first {
                first = false;
                let mut conditions: Vec<Box<dyn Condition>> = Vec::new();
                let mut created_coins = Vec::<Coin>::new();
                for payment in payments.clone() {
                    let send_create_coin_condition = payment.to_create_coin_condition();
                    conditions.push(Box::new(send_create_coin_condition));
                    created_coins.push(Coin::new(
                        coin.name(),
                        payment.puzzle_hash.to_bytes32(),
                        payment.amount_u64(),
                    ));
                }

                if change.to_u64().unwrap() > 0 {
                    conditions.push(Box::new(CreateCoinCondition::new(
                        change_puzzle_hash.clone(),
                        change.into(),
                        None,
                    )));
                    created_coins.push(Coin {
                        parent_coin_info: coin.name(),
                        puzzle_hash: change_puzzle_hash.clone().to_bytes32(),
                        amount: change.to_u64().unwrap(),
                    });
                }

                if fee_amount > 0 {
                    conditions.push(Box::new(ReserveFeeCondition::new(BigInt::from(fee_amount))));
                }
                for coin_announcement in coin_announcements_to_assert.clone() {
                    conditions.push(Box::new(coin_announcement));
                }
                for puzzle_announcement in puzzle_announcements_to_assert.clone() {
                    conditions.push(Box::new(puzzle_announcement));
                }

                let mut existing_coins_message = Vec::new();
                for coin in coins.clone() {
                    existing_coins_message.extend_from_slice(coin.name().raw());
                }

                let mut created_coins_message = Vec::new();
                for coin in created_coins {
                    created_coins_message.extend_from_slice(&coin.name().raw());
                }

                let message = Bytes::from(bytes_to_sha256(
                    [&existing_coins_message[..], &created_coins_message[..]].concat(),
                ));

                conditions.push(Box::new(CreateCoinAnnouncementCondition::new(
                    message.clone(),
                )));

                primary_assert_coin_announcement = Some(Box::new(
                    AssertCoinAnnouncementCondition::new(Bytes::from(coin.name()), message.clone()),
                ));

                solution = WalletDecoration::make_solution_from_conditions(&self, conditions);
            } else {
                solution = WalletDecoration::make_solution_from_conditions(
                    &self,
                    vec![primary_assert_coin_announcement.clone().unwrap().clone()],
                );
            }

            let puzzle = make_puzzle_reveal_from_puzzlehash(Puzzlehash::from(coin.puzzle_hash));

            let coin_spend = CoinSpend {
                coin,
                puzzle_reveal: puzzle.to_chia_program(),
                solution: solution.to_chia_program(),
            };
            spends.push(coin_spend.clone());
        }

        let fake_signature = ZEROS_BYTES96.clone();

        SpendBundle {
            coin_spends: spends,
            aggregated_signature: fake_signature,
        }
    }
    fn create_signed_spend_bundle_base(
        &self,
        payments: Vec<Payment>,
        coins_input: Vec<Coin>,
        change_puzzlehash: Puzzlehash,
        fee: BigInt,
        origin_id: Option<Bytes32>,
        coin_announcements_to_assert: Vec<AssertCoinAnnouncementCondition>,
        puzzle_announcements_to_assert: Vec<AssertPuzzleAnnouncementConditionImp>,
        make_puzzle_reveal_from_puzzlehash: fn(Puzzlehash) -> Program,
        transform_standard_solution: Option<fn(Program) -> Program>,
        keychain: WalletKeychain,
    ) -> SpendBundle {
        let spend_bundle = self.create_unsigned_spend_bundle_base(
            payments,
            coins_input,
            change_puzzlehash,
            fee,
            origin_id,
            coin_announcements_to_assert,
            puzzle_announcements_to_assert,
            make_puzzle_reveal_from_puzzlehash,
            transform_standard_solution,
        );
        let mut signatures = Vec::<chia_bls::Signature>::new();
        let coin_spends = spend_bundle.coin_spends.clone();
        for coin_spend in coin_spends.clone() {
            let signature = WalletDecoration::make_signature_for_coin_spend(
                &self,
                coin_spend.clone(),
                keychain.clone(),
            );
            signatures.push(signature);
        }

        let aggregate = AugSchemeMPL::aggregate(signatures);

        SpendBundle {
            coin_spends: coin_spends,
            aggregated_signature: aggregate,
        }
    }

    pub fn get_add_sig_me_message_from_result(
        result: Program,
        coin: Coin,
        network: BlockchainNetwork,
    ) -> Bytes {
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

        let id = coin.coin_id();
        let extra_data = Bytes::from_hex(&network.agg_sig_me_extra_data)
            .unwrap()
            .raw();
        let mut message = Vec::new();

        message.extend_from_slice(&agg_sig_me_condition);
        message.extend_from_slice(&id);
        message.extend_from_slice(&extra_data);

        Bytes::from(message)
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
            let coin_id_hex = hex::encode(&coin.coin_id());
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
        coin_announcements: HashSet<Bytes>,
        puzzle_announcements: HashSet<Bytes>,
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
    fn make_signature(
        self,
        private_key: &SecretKey,
        coin_spend: &CoinSpend,
        use_synthetic_offset: bool,
    ) -> Signature {
        let puzzle_reveal = SerializedProgram::from(coin_spend.puzzle_reveal.clone())
            .to_program()
            .unwrap();
        let args = SerializedProgram::from(coin_spend.solution.clone())
            .to_program()
            .unwrap();
        let result = puzzle_reveal.run(args);

        let addsigmessage = Self::get_add_sig_me_message_from_result(
            result.program,
            coin_spend.coin.clone(),
            self.network,
        );

        let private_key0 = if use_synthetic_offset {
            calculate_synthetic_private_key(private_key)
        } else {
            private_key.clone()
        };

        let signature = AugSchemeMPL::sign(&private_key0, addsigmessage.raw());

        signature
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
        Bytes::from_hex("c8109361adf2cd32c07587312052ddbc8bf61eb4644fd6351e1cf1f814f272fb")
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
        Bytes::from(announcement_message.clone()),
    );
    let assert_puzzle =
        AssertPuzzleAnnouncementConditionImp::new(Bytes::from(announcement_message.clone()));
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

#[test]
fn test_coin() {
    let amount: u64 = 100;
    let ph = Bytes32::from_hex(&"e30a9dc6c0379a72d77afa8d596a91399f9d18dbe5a87168b7a9b5381596b18c")
        .unwrap();
    let parent =
        Bytes32::from_hex(&"e30a9dc6c0379a72d77afa8d596a91399f9d18dbe5a87168b7a9b5381596b18a")
            .unwrap();
    let coin = Coin::new(parent, ph, amount.into());
    let coin_id = coin.name();
    let spected_coin_id =
        Bytes32::from_hex(&"4dce323de4276e5754fdac8d615f3a537161042e1f4dbdf319e8a80c142b6835")
            .unwrap();
    assert_eq!(coin_id, spected_coin_id);
}
