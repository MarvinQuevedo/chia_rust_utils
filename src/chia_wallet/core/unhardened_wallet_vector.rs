use std::collections::HashMap;

use chia_bls::secret_key::SecretKey;
use clvm_tools_rs::classic::clvm::__type_compatibility__::{Bytes, BytesFromType};

use super::{
    bls_mods::secret_key_from_stream,
    bytes::Puzzlehash,
    bytes_utils::{int_from_32_bits_stream, int_to_32_bits},
    wallet_puzzlehash::WalletPuzzlehash,
    wallet_vector::WalletVector,
};

pub struct UnhardenedWalletVector {
    pub wallet_vector: WalletVector,
}

impl UnhardenedWalletVector {
    pub fn new(
        child_private_key: SecretKey,
        puzzlehash: Puzzlehash,
        derivation_index: i32,
        asset_id_to_outer_puzzlehash: Option<HashMap<Puzzlehash, Puzzlehash>>,
    ) -> Self {
        UnhardenedWalletVector {
            wallet_vector: WalletVector::new(
                child_private_key,
                puzzlehash,
                derivation_index,
                asset_id_to_outer_puzzlehash,
            ),
        }
    }
    pub fn from_bytes(bytes: &Vec<u8>, derivation_index: i32) -> Self {
        let mut iterator = bytes.iter();
        Self::from_stream(&mut iterator, derivation_index)
    }

    pub fn from_stream(iterator: &mut std::slice::Iter<u8>, derivation_index: i32) -> Self {
        let child_private_key = secret_key_from_stream(iterator);
        let puzzlehash: Puzzlehash = Puzzlehash::from_stream(iterator);

        let mut asset_id_to_outer_puzzlehash = HashMap::new();

        let asset_id_map_length = int_from_32_bits_stream(iterator);

        for _ in 0..asset_id_map_length {
            let asset_id = Puzzlehash::from_stream(iterator);
            let outer_puzzlehash = Puzzlehash::from_stream(iterator);
            asset_id_to_outer_puzzlehash.insert(asset_id, outer_puzzlehash);
        }
        UnhardenedWalletVector {
            wallet_vector: WalletVector {
                child_private_key,
                puzzlehash,
                derivation_index,
                asset_id_to_outer_puzzlehash,
            },
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut bytes_list = Vec::new();
        let vector = &self.wallet_vector;
        bytes_list.extend_from_slice(&vector.child_private_key.to_bytes());
        bytes_list.extend_from_slice(&vector.puzzlehash.byte_list().to_vec());

        bytes_list.extend_from_slice(
            &int_to_32_bits(vector.asset_id_to_outer_puzzlehash.len() as i32).to_vec(),
        );

        for (asset_id, outer_puzzlehash) in &vector.asset_id_to_outer_puzzlehash {
            bytes_list.extend_from_slice(&asset_id.byte_list().to_vec());
            bytes_list.extend_from_slice(&outer_puzzlehash.byte_list().to_vec());
        }

        Bytes::new(Some(BytesFromType::Raw(bytes_list)))
    }

    pub fn to_wallet_puzzlehash(&self) -> WalletPuzzlehash {
        let vector = &self.wallet_vector;
        WalletPuzzlehash::from_puzzlehash(&vector.puzzlehash, vector.derivation_index)
    }
}
