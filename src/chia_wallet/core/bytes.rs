use std::hash::{Hash, Hasher};
use std::io::Read;

use clvm_tools_rs::classic::clvm::__type_compatibility__::{sha256, Bytes, BytesFromType};

use crate::program_utils::program::Program;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzlehash([u8; 48]);

impl Puzzlehash {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut puzzlehash_bytes = [0u8; 48];
        for i in 0..48 {
            puzzlehash_bytes[i] = bytes[i];
        }
        Puzzlehash(puzzlehash_bytes)
    }
    pub fn from_atom(program: Program) -> Self {
        let mut puzzlehash_bytes = [0u8; 48];
        let atom_bytes = program.as_vec().unwrap();
        for i in 0..48 {
            puzzlehash_bytes[i] = atom_bytes[i];
        }
        Puzzlehash(puzzlehash_bytes)
    }
}

// from WrapperBytes
impl From<WrapperBytes> for Puzzlehash {
    fn from(bytes: WrapperBytes) -> Self {
        let mut puzzlehash_bytes = [0u8; 48];
        for i in 0..48 {
            puzzlehash_bytes[i] = bytes.raw()[i];
        }
        Puzzlehash(puzzlehash_bytes)
    }
}

#[derive(Debug, Clone)]
pub struct WrapperBytes(pub Bytes);

impl WrapperBytes {
    pub fn sha256_hash(&self) -> WrapperBytes {
        return WrapperBytes(sha256(self.0.clone()));
    }
    pub fn new(value: BytesFromType) -> Self {
        WrapperBytes(Bytes::new(Some(value)))
    }
    pub fn raw(&self) -> Vec<u8> {
        self.0.raw()
    }

    pub fn from_atom(program: Program) -> Self {
        WrapperBytes::from(program.as_vec().unwrap())
    }
}
impl From<Vec<u8>> for WrapperBytes {
    fn from(bytes: Vec<u8>) -> Self {
        WrapperBytes(Bytes::new(Some(BytesFromType::Raw(bytes))))
    }
}
impl From<Bytes> for WrapperBytes {
    fn from(bytes: Bytes) -> Self {
        WrapperBytes(bytes)
    }
}

// Implement the necessary functions for Puzzlehash.
impl Puzzlehash {
    pub fn from_stream(_iterator: &mut std::slice::Iter<u8>) -> Puzzlehash {
        let mut puzzlehash_bytes = [0u8; 48];
        for i in 0..48 {
            puzzlehash_bytes[i] = *_iterator.next().unwrap();
        }
        Puzzlehash(puzzlehash_bytes)
    }

    pub fn byte_list(&self) -> &[u8; 48] {
        &self.0
    }
    pub fn to_bytes(&self) -> WrapperBytes {
        return WrapperBytes::from(self.0.to_vec());
    }
}

impl Clone for Puzzlehash {
    fn clone(&self) -> Self {
        Puzzlehash(self.0.clone())
    }
}

impl Hash for Puzzlehash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Delegate the hashing to the slice of bytes
        self.0.hash(state);
    }
}
