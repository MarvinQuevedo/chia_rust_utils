use std::collections::HashMap;
use num::BigInt;

lazy_static::lazy_static! {
    pub static ref KEYWORDS: HashMap<String, BigInt> = {
        let mut map = HashMap::new();
        map.insert(String::from("q"), BigInt::from(0x01));
        map.insert(String::from("a"), BigInt::from(0x02));
        map.insert(String::from("i"), BigInt::from(0x03));
        map.insert(String::from("c"), BigInt::from(0x04));
        map.insert(String::from("f"), BigInt::from(0x05));
        map.insert(String::from("r"), BigInt::from(0x06));
        map.insert(String::from("l"), BigInt::from(0x07));
        map.insert(String::from("x"), BigInt::from(0x08));
        map.insert(String::from("="), BigInt::from(0x09));
        map.insert(String::from(">s"), BigInt::from(0x0a));
        map.insert(String::from("sha256"), BigInt::from(0x0b));
        map.insert(String::from("substr"), BigInt::from(0x0c));
        map.insert(String::from("strlen"), BigInt::from(0x0d));
        map.insert(String::from("concat"), BigInt::from(0x0e));
        map.insert(String::from("+"), BigInt::from(0x10));
        map.insert(String::from("-"), BigInt::from(0x11));
        map.insert(String::from("*"), BigInt::from(0x12));
        map.insert(String::from("/"), BigInt::from(0x13));
        map.insert(String::from("divmod"), BigInt::from(0x14));
        map.insert(String::from(">"), BigInt::from(0x15));
        map.insert(String::from("ash"), BigInt::from(0x16));
        map.insert(String::from("lsh"), BigInt::from(0x17));
        map.insert(String::from("logand"), BigInt::from(0x18));
        map.insert(String::from("logior"), BigInt::from(0x19));
        map.insert(String::from("logxor"), BigInt::from(0x1a));
        map.insert(String::from("lognot"), BigInt::from(0x1b));
        map.insert(String::from("point_add"), BigInt::from(0x1d));
        map.insert(String::from("pubkey_for_exp"), BigInt::from(0x1e));
        map.insert(String::from("not"), BigInt::from(0x20));
        map.insert(String::from("any"), BigInt::from(0x21));
        map.insert(String::from("all"), BigInt::from(0x22));
        map.insert(String::from("."), BigInt::from(0x23));
        map.insert(String::from("softfork"), BigInt::from(0x24));
        map
    };
}
