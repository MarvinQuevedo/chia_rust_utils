use std::collections::HashMap;

use num_bigint::BigInt;

lazy_static::lazy_static! {
    static ref KEYWORDS: HashMap<String, BigInt> = {
        let mut map = HashMap::new();
        map.insert("q".to_string(), BigInt::from(0x01));
        map.insert("a".to_string(), BigInt::from(0x02));
        map.insert("i".to_string(), BigInt::from(0x03));
        map.insert("c".to_string(), BigInt::from(0x04));
        map.insert("f".to_string(), BigInt::from(0x05));
        map.insert("r".to_string(), BigInt::from(0x06));
        map.insert("l".to_string(), BigInt::from(0x07));
        map.insert("x".to_string(), BigInt::from(0x08));
        map.insert("=".to_string(), BigInt::from(0x09));
        map.insert(">s".to_string(), BigInt::from(0x0a));
        map.insert("sha256".to_string(), BigInt::from(0x0b));
        map.insert("substr".to_string(), BigInt::from(0x0c));
        map.insert("strlen".to_string(), BigInt::from(0x0d));
        map.insert("concat".to_string(), BigInt::from(0x0e));
        map.insert("+".to_string(), BigInt::from(0x10));
        map.insert("-".to_string(), BigInt::from(0x11));
        map.insert("*".to_string(), BigInt::from(0x12));
        map.insert("/".to_string(), BigInt::from(0x13));
        map.insert("divmod".to_string(), BigInt::from(0x14));
        map.insert(">".to_string(), BigInt::from(0x15));
        map.insert("ash".to_string(), BigInt::from(0x16));
        map.insert("lsh".to_string(), BigInt::from(0x17));
        map.insert("logand".to_string(), BigInt::from(0x18));
        map.insert("logior".to_string(), BigInt::from(0x19));
        map.insert("logxor".to_string(), BigInt::from(0x1a));
        map.insert("lognot".to_string(), BigInt::from(0x1b));
        map.insert("point_add".to_string(), BigInt::from(0x1d));
        map.insert("pubkey_for_exp".to_string(), BigInt::from(0x1e));
        map.insert("not".to_string(), BigInt::from(0x20));
        map.insert("any".to_string(), BigInt::from(0x21));
        map.insert("all".to_string(), BigInt::from(0x22));
        map.insert(".".to_string(), BigInt::from(0x23));
        map.insert("softfork".to_string(), BigInt::from(0x24));
        map
    };
}
