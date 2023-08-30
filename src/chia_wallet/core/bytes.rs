use std::hash::{Hash, Hasher};
use std::io::Read;

use crate::program_utils::program::Program;
use chia_protocol::Bytes32;
use clvm_tools_rs::classic::clvm::__type_compatibility__::Bytes as CvlmBytes;
use clvm_tools_rs::classic::clvm::__type_compatibility__::{sha256, BytesFromType};

#[derive(Debug, Clone)]
pub struct Bytes(pub CvlmBytes);

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.to_hex() == other.to_hex()
    }
}
impl Eq for Bytes {
    // fn eq(&self, other: &Self) -> bool {
    //  self.to_hex() == other.to_hex()
    // }
}

impl Bytes {
    pub fn sha256_hash(&self) -> Bytes {
        return Bytes(sha256(self.0.clone()));
    }
    pub fn new(value: BytesFromType) -> Self {
        Bytes(CvlmBytes::new(Some(value)))
    }
    pub fn raw(&self) -> Vec<u8> {
        self.0.raw()
    }
    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, std::io::Error> {
        let bytes = hex::decode(hex).unwrap();

        Ok(Bytes(CvlmBytes::new(Some(BytesFromType::Raw(bytes)))))
    }
    pub fn to_hex(&self) -> String {
        hex::encode(self.0.raw())
    }

    pub fn from_atom(program: Program) -> Self {
        Bytes::from(program.as_vec().unwrap())
    }
    pub fn as_puzzlehash(&self) -> Puzzlehash {
        Puzzlehash::from_bytes(&self.0.raw())
    }
}
impl From<Vec<u8>> for Bytes {
    fn from(bytes: Vec<u8>) -> Self {
        Bytes(CvlmBytes::new(Some(BytesFromType::Raw(bytes))))
    }
}
impl From<CvlmBytes> for Bytes {
    fn from(bytes: CvlmBytes) -> Self {
        Bytes(bytes)
    }
}

impl From<Bytes32> for Bytes {
    fn from(bytes: Bytes32) -> Self {
        Bytes(CvlmBytes::new(Some(BytesFromType::Raw(
            bytes.to_bytes().to_vec(),
        ))))
    }
}

impl Hash for Bytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Delegate the hashing to the slice of bytes
        self.to_hex().hash(state);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzlehash([u8; 32]);

impl Puzzlehash {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut puzzlehash_bytes = [0u8; 32];

        for i in 0..32 {
            puzzlehash_bytes[i] = bytes[i];
        }
        Puzzlehash(puzzlehash_bytes)
    }
    pub fn from_atom(program: Program) -> Self {
        let mut puzzlehash_bytes = [0u8; 32];
        let atom_bytes = program.as_vec().unwrap();
        for i in 0..32 {
            puzzlehash_bytes[i] = atom_bytes[i];
        }
        Puzzlehash(puzzlehash_bytes)
    }
    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, std::io::Error> {
        let bytes = hex::decode(hex).unwrap();
        let wrapped_bytes = Bytes(CvlmBytes::new(Some(BytesFromType::Raw(bytes)))).as_puzzlehash();
        Ok(wrapped_bytes)
    }
    pub fn from_stream(_iterator: &mut std::slice::Iter<u8>) -> Puzzlehash {
        let mut puzzlehash_bytes = [0u8; 32];
        for i in 0..32 {
            puzzlehash_bytes[i] = *_iterator.next().unwrap();
        }
        Puzzlehash(puzzlehash_bytes)
    }

    pub fn byte_list(&self) -> &[u8; 32] {
        &self.0
    }
    pub fn to_bytes(&self) -> Bytes {
        return Bytes::from(self.0.to_vec());
    }
    pub fn to_bytes32(&self) -> Bytes32 {
        let bytes = self.0;
        return Bytes32::from(bytes);
    }
}

// from WrapperBytes
impl From<Bytes> for Puzzlehash {
    fn from(bytes: Bytes) -> Self {
        let mut puzzlehash_bytes = [0u8; 32];
        for i in 0..32 {
            puzzlehash_bytes[i] = bytes.raw()[i];
        }
        Puzzlehash(puzzlehash_bytes)
    }
}
impl From<Bytes32> for Puzzlehash {
    fn from(bytes: Bytes32) -> Self {
        let mut puzzlehash_bytes = [0u8; 32];
        for i in 0..32 {
            puzzlehash_bytes[i] = bytes.to_bytes()[i];
        }
        Puzzlehash(puzzlehash_bytes)
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

#[test]

fn test_bytes_from_string_hex() {
    let bytes = Bytes::from_hex(
        &"e30a9dc6c0379a72d77afa8d596a91399f9d18dbe5a87168b7a9b5381596b18c".to_string(),
    )
    .unwrap();
    let bytes_raw = bytes.raw();
    println!("bytes {:?}", bytes_raw);
    assert_eq!(bytes_raw.len(), 32);
    assert_eq!(
        bytes_raw,
        vec![
            227, 10, 157, 198, 192, 55, 154, 114, 215, 122, 250, 141, 89, 106, 145, 57, 159, 157,
            24, 219, 229, 168, 113, 104, 183, 169, 181, 56, 21, 150, 177, 140
        ]
    );
}
