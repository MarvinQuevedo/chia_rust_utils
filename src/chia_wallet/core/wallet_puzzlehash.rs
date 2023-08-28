use std::collections::HashMap;

use clvm_tools_rs::classic::clvm::__type_compatibility__::Bytes as CvlmBytes;

use super::{
    bytes::{Bytes, Puzzlehash},
    serialization::serialize_hex_item,
};

#[derive(Debug, Clone)]
pub struct WalletPuzzlehash {
    pub bytes_list: Bytes,
    pub derivation_index: i32,
}

impl PartialEq for WalletPuzzlehash {
    fn eq(&self, other: &Self) -> bool {
        self.bytes_list.raw() == other.bytes_list.raw()
            && self.derivation_index == other.derivation_index
    }
}

impl Eq for WalletPuzzlehash {}

impl WalletPuzzlehash {
    pub fn new(bytes_list: Bytes, derivation_index: i32) -> Self {
        WalletPuzzlehash {
            bytes_list,
            derivation_index,
        }
    }

    pub fn from_puzzlehash(puzzlehash: &Puzzlehash, derivation_index: i32) -> Self {
        let bytes_list = puzzlehash.to_bytes();
        WalletPuzzlehash {
            bytes_list,
            derivation_index,
        }
    }

    pub fn from_json(json: &HashMap<String, serde_json::Value>) -> Self {
        let puzzlehash_hex = json.get("puzzlehash").unwrap().as_str().unwrap();
        let derivation_index = json.get("derivation_index").unwrap().as_i64().unwrap() as i32;
        let bytes_list = serialize_hex_item(puzzlehash_hex);
        let wrapper_bytes = Bytes::from(bytes_list);
        WalletPuzzlehash {
            bytes_list: wrapper_bytes,
            derivation_index,
        }
    }

    // fn to_address(&self, ticker: &str) -> WalletAddress {
    //     WalletAddress::from_puzzlehash(self, ticker, self.derivation_index)
    // }

    pub fn to_json(&self) -> HashMap<String, serde_json::Value> {
        let mut json = HashMap::new();
        json.insert(
            "puzzlehash".to_string(),
            serde_json::Value::String(self.to_hex()),
        );
        json.insert(
            "derivation_index".to_string(),
            serde_json::Value::Number(self.derivation_index.into()),
        );
        json
    }

    pub fn to_hex(&self) -> String {
        return hex::encode(&self.bytes_list.raw());
    }
}

impl std::hash::Hash for WalletPuzzlehash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bytes_list.raw().hash(state);
        self.derivation_index.hash(state);
    }
}
