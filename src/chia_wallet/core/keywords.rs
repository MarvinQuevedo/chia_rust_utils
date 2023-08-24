use lazy_static::lazy_static;
use std::collections::HashMap;

use num_bigint::BigInt;

macro_rules! keyword {
    ($map:expr, $key:expr, $value:expr) => {
        $map.insert($key.to_string(), BigInt::from($value));
    };
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<String, BigInt> = {
        let mut map = HashMap::new();
        keyword!(map, "q", 0x01);
        keyword!(map, "a", 0x02);
        keyword!(map, "i", 0x03);
        keyword!(map, "c", 0x04);
        keyword!(map, "f", 0x05);
        keyword!(map, "r", 0x06);
        keyword!(map, "l", 0x07);
        keyword!(map, "x", 0x08);
        keyword!(map, "=", 0x09);
        keyword!(map, ">s", 0x0a);
        keyword!(map, "sha256", 0x0b);
        keyword!(map, "substr", 0x0c);
        keyword!(map, "strlen", 0x0d);
        keyword!(map, "concat", 0x0e);
        keyword!(map, "+", 0x10);
        keyword!(map, "-", 0x11);
        keyword!(map, "*", 0x12);
        keyword!(map, "/", 0x13);
        keyword!(map, "divmod", 0x14);
        keyword!(map, ">", 0x15);
        keyword!(map, "ash", 0x16);
        keyword!(map, "lsh", 0x17);
        keyword!(map, "logand", 0x18);
        keyword!(map, "logior", 0x19);
        keyword!(map, "logxor", 0x1a);
        keyword!(map, "lognot", 0x1b);
        keyword!(map, "point_add", 0x1d);
        keyword!(map, "pubkey_for_exp", 0x1e);
        keyword!(map, "not", 0x20);
        keyword!(map, "any", 0x21);
        keyword!(map, "all", 0x22);
        keyword!(map, ".", 0x23);
        keyword!(map, "softfork", 0x24);
        map
    };
}

pub fn keyword(name: &str) -> &BigInt {
    KEYWORDS.get(name).unwrap()
}

// create test here

#[test]
fn test_keyword() {
    let keyword = keyword("q");
    assert_eq!(keyword, &BigInt::from(0x01));
}
