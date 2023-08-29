use sha2::{Digest, Sha256};

pub fn bytes_to_sha256(bytes: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

pub fn flip(binary: String) -> String {
    binary
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect()
}

pub fn num_bits(mut value: i32) -> usize {
    if value == 0 {
        return 1;
    }
    let mut bits = 0;
    while value != 0 {
        value >>= 1;
        bits += 1;
    }
    bits
}

pub fn u8_to_bytes(value: u8) -> Vec<u8> {
    int_to_bytes(value.into(), 2, Endian::Big, false)
}

pub fn int_to_bytes(value: i32, size: usize, endian: Endian, signed: bool) -> Vec<u8> {
    if value < 0 && !signed {
        panic!("Cannot convert negative int to unsigned.");
    }
    let binary = format!("{:0width$b}", value, width = size * 8);
    let binary = if value < 0 {
        let flipped = flip(binary);
        format!(
            "{:0width$b}",
            i32::from_str_radix(&flipped, 2).unwrap() + 1,
            width = size * 8
        )
    } else {
        binary
    };

    let bytes: Vec<u8> = binary
        .as_bytes()
        .chunks(8)
        .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 2).unwrap())
        .collect();

    match endian {
        Endian::Little => bytes.into_iter().rev().collect(),
        Endian::Big => bytes,
    }
}

pub fn uint_to_bytes(value: u64, size: usize, endian: Endian, signed: bool) -> Vec<u8> {
    if value < 0 && !signed {
        panic!("Cannot convert negative int to unsigned.");
    }
    let binary = format!("{:0width$b}", value, width = size * 8);
    let binary = if value < 0 {
        let flipped = flip(binary);
        format!(
            "{:0width$b}",
            i32::from_str_radix(&flipped, 2).unwrap() + 1,
            width = size * 8
        )
    } else {
        binary
    };

    let bytes: Vec<u8> = binary
        .as_bytes()
        .chunks(8)
        .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 2).unwrap())
        .collect();

    match endian {
        Endian::Little => bytes.into_iter().rev().collect(),
        Endian::Big => bytes,
    }
}
pub fn int_to_64_bits(value: i32) -> Vec<u8> {
    int_to_bytes(value, 8, Endian::Big, true)
}
pub fn uint_to_64_bits(value: u64) -> Vec<u8> {
    uint_to_bytes(value, 8, Endian::Big, true)
}

pub fn int_to_32_bits(value: i32) -> Vec<u8> {
    int_to_bytes(value, 4, Endian::Big, true)
}

pub fn int_from_32_bits_stream(iterator: &mut std::slice::Iter<u8>) -> i32 {
    bytes_to_int(iterator.take(4).copied().collect(), Endian::Big, true)
}

pub fn int_from_64_bits_stream(iterator: &mut std::slice::Iter<u8>) -> i32 {
    bytes_to_int(iterator.take(8).copied().collect(), Endian::Big, true)
}

pub fn maybe_int_from_64_bits_stream(iterator: &mut std::slice::Iter<u8>) -> Option<i32> {
    let does_exist = iterator.len() >= 8;
    if !does_exist {
        return None;
    }
    Some(bytes_to_int(
        iterator.take(8).copied().collect(),
        Endian::Big,
        true,
    ))
}

pub fn int_to_8_bits(value: i32) -> Vec<u8> {
    int_to_bytes(value, 1, Endian::Big, true)
}

pub fn encode_int(value: i32) -> Vec<u8> {
    if value == 0 {
        return vec![];
    }
    let length = (num_bits(value) + 8) / 8;
    let mut bytes = int_to_bytes(value, length, Endian::Big, true);
    while bytes.len() > 1 && bytes[0] == if bytes[1] & 0x80 != 0 { 0xFF } else { 0x00 } {
        bytes.remove(0);
    }
    bytes
}

pub fn bytes_to_int(bytes: Vec<u8>, endian: Endian, signed: bool) -> i32 {
    if bytes.is_empty() {
        return 0;
    }
    let sign = if endian == Endian::Little {
        bytes[bytes.len() - 1] & 0x80 != 0
    } else {
        bytes[0] & 0x80 != 0
    };

    let binary: String = bytes.iter().map(|byte| format!("{:08b}", byte)).collect();

    let result = i32::from_str_radix(&binary, 2).unwrap();
    if sign && signed {
        -result
    } else {
        result
    }
}

pub fn decode_int(bytes: Vec<u8>) -> i32 {
    bytes_to_int(bytes, Endian::Big, true)
}

pub fn big_int_to_bytes(value: i128, size: usize, endian: Endian, signed: bool) -> Vec<u8> {
    if value < 0 && !signed {
        panic!("Cannot convert negative bigint to unsigned.");
    }
    let binary = format!("{:0width$b}", value, width = size * 8);
    let binary = if value < 0 {
        let flipped = flip(binary);
        format!(
            "{:0width$b}",
            i128::from_str_radix(&flipped, 2).unwrap() + 1,
            width = size * 8
        )
    } else {
        binary
    };

    let bytes: Vec<u8> = binary
        .as_bytes()
        .chunks(8)
        .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 2).unwrap())
        .collect();

    match endian {
        Endian::Little => bytes.into_iter().rev().collect(),
        Endian::Big => bytes,
    }
}

pub fn encode_big_int(value: i128) -> Vec<u8> {
    if value == 0 {
        return vec![];
    }
    let length = (num_bits(value as i32) + 8) / 8;
    let mut bytes = big_int_to_bytes(value, length, Endian::Big, true);
    while bytes.len() > 1 && bytes[0] == if bytes[1] & 0x80 != 0 { 0xFF } else { 0x00 } {
        bytes.remove(0);
    }
    bytes
}

#[derive(Debug, PartialEq, Eq)]
pub enum Endian {
    Little,
    Big,
}

#[test]
fn u8_to_bytes_test() {
    let result = u8_to_bytes(1);
    assert_eq!(result, vec![0, 1]);
}
