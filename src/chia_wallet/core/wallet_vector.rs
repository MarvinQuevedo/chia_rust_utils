use std::collections::HashMap;

use chia_bls::secret_key::SecretKey;
use clvm_tools_rs::classic::clvm::__type_compatibility__::{Bytes, BytesFromType};

use super::{
    bls_mods::secret_key_from_stream,
    bytes::Puzzlehash,
    bytes_utils::{int_from_32_bits_stream, int_to_32_bits},
    wallet_puzzlehash::WalletPuzzlehash,
};

pub struct WalletVector {
    pub child_private_key: SecretKey,
    pub puzzlehash: Puzzlehash,
    pub derivation_index: i32,
    pub asset_id_to_outer_puzzlehash: HashMap<Puzzlehash, Puzzlehash>,
}

impl WalletVector {
    pub fn new(
        child_private_key: SecretKey,
        puzzlehash: Puzzlehash,
        derivation_index: i32,
        asset_id_to_outer_puzzlehash: Option<HashMap<Puzzlehash, Puzzlehash>>,
    ) -> Self {
        WalletVector {
            child_private_key,
            puzzlehash,
            derivation_index,
            asset_id_to_outer_puzzlehash: asset_id_to_outer_puzzlehash.unwrap_or_default(),
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

        WalletVector {
            child_private_key,
            puzzlehash,
            derivation_index,
            asset_id_to_outer_puzzlehash,
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut bytes_list = Vec::new();
        bytes_list.extend_from_slice(&self.child_private_key.to_bytes());
        bytes_list.extend_from_slice(&self.puzzlehash.byte_list().to_vec());

        bytes_list.extend_from_slice(
            &int_to_32_bits(self.asset_id_to_outer_puzzlehash.len() as i32).to_vec(),
        );

        for (asset_id, outer_puzzlehash) in &self.asset_id_to_outer_puzzlehash {
            bytes_list.extend_from_slice(&asset_id.byte_list().to_vec());
            bytes_list.extend_from_slice(&outer_puzzlehash.byte_list().to_vec());
        }

        Bytes::new(Some(BytesFromType::Raw(bytes_list)))
    }

    pub fn to_wallet_puzzlehash(&self) -> WalletPuzzlehash {
        WalletPuzzlehash::from_puzzlehash(&self.puzzlehash, self.derivation_index)
    }
    pub fn child_public_key(&self) -> chia_bls::public_key::PublicKey {
        self.child_private_key.public_key()
    }
}

impl Clone for WalletVector {
    fn clone(&self) -> Self {
        WalletVector {
            child_private_key: self.child_private_key.clone(),
            puzzlehash: self.puzzlehash.clone(),
            derivation_index: self.derivation_index,
            asset_id_to_outer_puzzlehash: self.asset_id_to_outer_puzzlehash.clone(),
        }
    }
}
