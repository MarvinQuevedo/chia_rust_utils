use crate::blockchain::utils::{additions_for_solution, fee_for_solution};
use crate::program_utils::serialized_program::SerializedProgram;
use crate::program_utils::utils::INFINITE_COST;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

use super::coin::Coin;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CoinSpend {
    pub coin: Coin,
    pub puzzle_reveal: SerializedProgram,
    pub solution: SerializedProgram,
}
impl CoinSpend {
    /* pub fn additions(&self) -> Vec<Coin> {
        return additions_for_solution(
            self.coin.name(),
            &self.puzzle_reveal,
            &self.solution,
            INFINITE_COST,
        );
    } */
    pub fn reserved_fee(self) -> BigInt {
        return fee_for_solution(&self.puzzle_reveal, &self.solution, INFINITE_COST.into());
    }
}
