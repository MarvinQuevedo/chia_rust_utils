use chia_bls::{public_key::PublicKey, secret_key::SecretKey};
use clvm_tools_rs::classic::clvm::__type_compatibility__::{Bytes, BytesFromType};

use super::{bls_mods::secret_key_from_stream, bytes_utils::int_from_32_bits_stream};

pub struct SingletonWalletVector {
    singleton_owner_private_key: SecretKey,
    pooling_authentication_private_key: SecretKey,
    derivation_index: i32,
}

impl SingletonWalletVector {
    pub fn new(
        singleton_owner_private_key: SecretKey,
        pooling_authentication_private_key: SecretKey,
        derivation_index: i32,
    ) -> Self {
        SingletonWalletVector {
            singleton_owner_private_key,
            pooling_authentication_private_key,
            derivation_index,
        }
    }
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        let mut iterator = bytes.iter();
        Self::from_stream(&mut iterator)
    }
    pub fn from_stream(iterator: &mut std::slice::Iter<u8>) -> Self {
        let singleton_owner_private_key = secret_key_from_stream(iterator);
        let pooling_authentication_private_key = secret_key_from_stream(iterator);
        let derivation_index: i32 = int_from_32_bits_stream(iterator);
        SingletonWalletVector {
            singleton_owner_private_key,
            pooling_authentication_private_key,
            derivation_index,
        }
    }
    pub fn to_bytes(&self) -> Bytes {
        let mut bytes = vec![];
        bytes.extend(self.singleton_owner_private_key.to_bytes());
        bytes.extend(self.pooling_authentication_private_key.to_bytes());
        Bytes::new(Some(BytesFromType::Raw(bytes)))
    }

    pub fn singleton_owner_public_key(&self) -> PublicKey {
        self.singleton_owner_private_key.public_key()
    }

    pub fn pooling_authentication_public_key(&self) -> PublicKey {
        self.pooling_authentication_private_key.public_key()
    }
}

impl Clone for SingletonWalletVector {
    fn clone(&self) -> Self {
        SingletonWalletVector {
            singleton_owner_private_key: self.singleton_owner_private_key.clone(),
            pooling_authentication_private_key: self.pooling_authentication_private_key.clone(),
            derivation_index: self.derivation_index,
        }
    }
}
