use crate::program_utils::utils::hash_256;
use chia_utils_streamable_macro::{
    bytes_utils::uint_to_64_bits,
    chia_streamable,
    sized_bytes::{bigint_to_u64, u64_to_bytes, vec_u8_to_bigint, Bytes32, Bytes8, SizedBytes},
};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Coin {
    pub amount: Bytes8,
    pub parent_coin_info: Bytes32,
    pub puzzle_hash: Bytes32,
}
impl Coin {
    pub fn amount(&self) -> u64 {
        vec_u8_to_bigint(self.amount.to_bytes())
    }
    pub fn new(amount: u64, parent_coin_info: Bytes32, puzzle_hash: Bytes32) -> Self {
        let amount_bytes = uint_to_64_bits(amount);
        let amount = Bytes8::from(amount_bytes);
        Coin {
            amount,
            parent_coin_info,
            puzzle_hash,
        }
    }
}

chia_streamable!(Coin {
    amount: Bytes8,
    parent_coin_info: Bytes32,
    puzzle_hash: Bytes32
});

#[test]
fn test_coin() {
    let parent = Bytes32::from(
        "482b49902d310c53065c3531d398d41808f1390590d566815d67040f6a32d124".to_string(),
    );
    let puzzle = Bytes32::from("43715a0d654e83c037f53e903a7554b4c21cd4ca29dab2e18185987a4ca4ce17");
    let coin2 = Coin::new(100, parent, puzzle);
    let hash = Bytes32::from(
        "63585ad45a980bbb5fbebb80e6466a042937312ce2604a4194157f51be6d3a85".to_string(),
    );
    let name = coin2.name();

    assert_eq!(coin2.name(), hash);
    assert_eq!(coin2.amount(), 100);
}
