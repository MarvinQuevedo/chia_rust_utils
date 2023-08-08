use std::hash::{Hash, Hasher};
use std::io::Read;

use clvm_tools_rs::classic::clvm::__type_compatibility__::{Bytes, BytesFromType};

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzlehash([u8; 48]);

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
    pub fn to_bytes(&self) -> Bytes {
        return Bytes::new(Some(BytesFromType::Raw(self.0.to_vec())));
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
