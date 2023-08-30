use std::collections::{HashMap, HashSet};

use crate::chia_wallet::core::bytes::Puzzlehash;
use clvm_tools_rs::classic::clvm::__type_compatibility__::{Bytes, BytesFromType};
use linked_hash_map::LinkedHashMap;

use super::bytes_utils::{int_from_32_bits_stream, int_to_32_bits};
use super::exceptions::PuzzlehashNotFound;
use super::serialization::PublicKeyWrapper;
use super::singleton_wallet_vector::SingletonWalletVector;
use super::unhardened_wallet_vector::UnhardenedWalletVector;
use super::wallet_puzzlehash::WalletPuzzlehash;
use super::wallet_vector::WalletVector;

pub struct WalletKeychain {
    hardened_map: LinkedHashMap<Puzzlehash, WalletVector>,
    unhardened_map: LinkedHashMap<Puzzlehash, UnhardenedWalletVector>,
    singleton_wallet_vectors_map: HashMap<PublicKeyWrapper, SingletonWalletVector>,
}

impl WalletKeychain {
    fn new(
        hardened_map: LinkedHashMap<Puzzlehash, WalletVector>,
        unhardened_map: LinkedHashMap<Puzzlehash, UnhardenedWalletVector>,
        singleton_wallet_vectors_map: HashMap<PublicKeyWrapper, SingletonWalletVector>,
    ) -> Self {
        WalletKeychain {
            hardened_map: hardened_map,
            unhardened_map: unhardened_map,
            singleton_wallet_vectors_map: singleton_wallet_vectors_map,
        }
    }

    fn from_bytes(bytes: &Vec<u8>) -> Self {
        let mut iterator = bytes.iter();
        Self::from_stream(&mut iterator)
    }

    fn from_stream(iterator: &mut std::slice::Iter<u8>) -> Self {
        let mut hardened_map = LinkedHashMap::new();
        let mut unhardened_map = LinkedHashMap::new();
        let mut singleton_wallet_vectors_map = HashMap::new();

        let hardened_map_length = int_from_32_bits_stream(iterator);

        for index in 0..hardened_map_length {
            let puzzlehash = Puzzlehash::from_stream(iterator);
            let hardened_wallet_vector = WalletVector::from_stream(iterator, index);
            hardened_map.insert(puzzlehash, hardened_wallet_vector);
        }

        let unhardened_map_length = int_from_32_bits_stream(iterator);

        for index in 0..unhardened_map_length {
            let puzzlehash = Puzzlehash::from_stream(iterator);
            let unhardened_wallet_vector = UnhardenedWalletVector::from_stream(iterator, index);
            unhardened_map.insert(puzzlehash, unhardened_wallet_vector);
        }

        let singleton_wallet_vectors_map_length = int_from_32_bits_stream(iterator);

        for _ in 0..singleton_wallet_vectors_map_length {
            let singleton_wallet_vector = SingletonWalletVector::from_stream(iterator);
            singleton_wallet_vectors_map.insert(
                PublicKeyWrapper(singleton_wallet_vector.singleton_owner_public_key()),
                singleton_wallet_vector,
            );
        }

        WalletKeychain::new(hardened_map, unhardened_map, singleton_wallet_vectors_map)
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut bytes_list = Vec::new();
        bytes_list.extend_from_slice(&int_to_32_bits(self.hardened_map.len() as i32).to_vec());

        for (index, hardened_wallet_vector) in self.hardened_map.iter() {
            bytes_list.extend_from_slice(&index.byte_list().to_vec());
            bytes_list.extend_from_slice(&hardened_wallet_vector.to_bytes().raw());
        }

        bytes_list.extend_from_slice(&int_to_32_bits(self.unhardened_map.len() as i32).to_vec());

        for (index, unhardened_wallet_vector) in self.unhardened_map.iter() {
            bytes_list.extend_from_slice(&index.byte_list().to_vec());
            bytes_list.extend_from_slice(&unhardened_wallet_vector.to_bytes().raw());
        }

        bytes_list.extend_from_slice(
            &int_to_32_bits(self.singleton_wallet_vectors_map.len() as i32).to_vec(),
        );

        for singleton_wallet_vector in self.singleton_wallet_vectors_map.values() {
            bytes_list.extend_from_slice(&singleton_wallet_vector.to_bytes().raw());
        }

        Bytes::new(Some(BytesFromType::Raw(bytes_list)))
    }

    pub fn singleton_wallet_vectors_map(&self) -> Vec<SingletonWalletVector> {
        self.singleton_wallet_vectors_map
            .values()
            .cloned()
            .collect()
    }

    pub fn get_wallet_vector(&self, puzzlehash: Puzzlehash) -> Option<&WalletVector> {
        if let Some(unhardened_wallet_vector) = self.unhardened_map.get(&puzzlehash) {
            return Some(&unhardened_wallet_vector.wallet_vector);
        }

        self.hardened_map.get(&puzzlehash)
    }

    fn get_wallet_vector_or_throw(
        &self,
        puzzlehash: Puzzlehash,
    ) -> Result<&WalletVector, PuzzlehashNotFound> {
        let cloned_puzzlehash = puzzlehash.clone();

        if let Some(wallet_vector) = self.get_wallet_vector(puzzlehash) {
            Ok(wallet_vector)
        } else {
            Err(PuzzlehashNotFound(cloned_puzzlehash))
        }
    }

    fn puzzlehashes(&self) -> Vec<Puzzlehash> {
        let unique_puzzlehashes: HashSet<Puzzlehash> = self
            .unhardened_map
            .values()
            .map(|wv| wv.wallet_vector.puzzlehash.clone())
            .collect();

        unique_puzzlehashes.into_iter().collect()
    }

    fn puzzlehashes_hardened(&self) -> Vec<Puzzlehash> {
        let unique_puzzlehashes: HashSet<Puzzlehash> = self
            .hardened_map
            .values()
            .map(|wv| wv.puzzlehash.clone())
            .collect();

        unique_puzzlehashes.into_iter().collect()
    }

    fn wallet_puzzlehashes(&self) -> Vec<WalletPuzzlehash> {
        let unique_wallet_puzzlehashes: HashSet<WalletPuzzlehash> = self
            .unhardened_map
            .values()
            .map(|wv| wv.to_wallet_puzzlehash())
            .collect();

        unique_wallet_puzzlehashes.into_iter().collect()
    }

    fn wallet_puzzlehashes_hardened(&self) -> Vec<WalletPuzzlehash> {
        let unique_wallet_puzzlehashes: HashSet<WalletPuzzlehash> = self
            .hardened_map
            .values()
            .map(|wv| wv.to_wallet_puzzlehash())
            .collect();

        unique_wallet_puzzlehashes.into_iter().collect()
    }
    fn get_outer_puzzle_hashes_for_asset_id(
        &self,
        asset_id: Puzzlehash,
    ) -> Result<Vec<Puzzlehash>, String> {
        if !self.unhardened_map.values().any(|v| {
            v.wallet_vector
                .asset_id_to_outer_puzzlehash
                .contains_key(&asset_id)
        }) {
            return Err(String::from(
                "Puzzlehashes for given Asset Id are not in keychain",
            ));
        }

        let outer_puzzlehashes: Vec<Puzzlehash> = self
            .unhardened_map
            .values()
            .map(|v| {
                v.wallet_vector
                    .asset_id_to_outer_puzzlehash
                    .get(&asset_id)
                    .unwrap()
                    .clone()
            })
            .collect();

        Ok(outer_puzzlehashes)
    }
}

impl Clone for WalletKeychain {
    fn clone(&self) -> Self {
        let mut hardened_map = LinkedHashMap::new();
        let mut unhardened_map = LinkedHashMap::new();
        let mut singleton_wallet_vectors_map = HashMap::new();

        for (puzzlehash, wallet_vector) in self.hardened_map.iter() {
            hardened_map.insert(puzzlehash.clone(), wallet_vector.clone());
        }

        for (puzzlehash, unhardened_wallet_vector) in self.unhardened_map.iter() {
            unhardened_map.insert(puzzlehash.clone(), unhardened_wallet_vector.clone());
        }

        for (public_key_wrapper, singleton_wallet_vector) in
            self.singleton_wallet_vectors_map.iter()
        {
            singleton_wallet_vectors_map
                .insert(public_key_wrapper.clone(), singleton_wallet_vector.clone());
        }

        WalletKeychain::new(
            hardened_map.clone(),
            unhardened_map.clone(),
            singleton_wallet_vectors_map.clone(),
        )
    }
}
