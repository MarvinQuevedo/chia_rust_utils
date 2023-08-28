use chia_protocol::coin::Coin as ChiaCoin;

use crate::chia_wallet::core::bytes::Bytes;

pub struct Coin(ChiaCoin);

impl Coin {
    pub fn name(&self) -> Bytes {
        let name_bytes = self.0.coin_id().to_vec();
        Bytes::from(name_bytes)
    }
}
