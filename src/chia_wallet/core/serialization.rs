use byteorder::{BigEndian, ReadBytesExt};
use chia_bls::public_key::PublicKey;
use clvm_tools_rs::classic::clvm::__type_compatibility__::{Bytes, BytesFromType};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};
use std::str::from_utf8;

#[derive(Debug)]
pub struct PublicKeyWrapper(pub PublicKey);

impl PartialEq for PublicKeyWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bytes() == other.0.to_bytes()
    }
}

impl Eq for PublicKeyWrapper {}

impl std::hash::Hash for PublicKeyWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bytes().hash(state);
    }
}
impl Clone for PublicKeyWrapper {
    fn clone(&self) -> Self {
        PublicKeyWrapper(self.0.clone())
    }
}

pub fn string_from_stream(mut iterator: std::slice::Iter<u8>) -> Result<String> {
    let mut string_length_bytes = [0; 4];
    for i in 0..4 {
        string_length_bytes[i] = *iterator
            .next()
            .ok_or_else(|| Error::new(ErrorKind::UnexpectedEof, "Unexpected end of data"))?;
    }

    let string_length = (&string_length_bytes[..]).read_i32::<BigEndian>()?;
    let mut string_bytes = vec![0; string_length as usize];
    for i in 0..string_length {
        string_bytes[i as usize] = *iterator
            .next()
            .ok_or_else(|| Error::new(ErrorKind::UnexpectedEof, "Unexpected end of data"))?;
    }

    let string = from_utf8(&string_bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok(string.to_string())
}

pub trait ToBytesMixin {
    fn to_bytes(&self) -> Bytes;
}

impl ToBytesMixin for i64 {
    fn to_bytes(&self) -> Bytes {
        Bytes::new(Some(BytesFromType::Raw(self.to_be_bytes().to_vec())))
    }
}

impl ToBytesMixin for Bytes {
    fn to_bytes(&self) -> Bytes {
        self.clone()
    }
}

impl ToBytesMixin for bool {
    fn to_bytes(&self) -> Bytes {
        Bytes::new(Some(BytesFromType::Raw(vec![*self as u8])))
    }
}

impl ToBytesMixin for String {
    fn to_bytes(&self) -> Bytes {
        Bytes::new(Some(BytesFromType::String(self.clone())))
    }
}

impl PartialEq for dyn ToBytesMixin {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes().raw() == other.to_bytes().raw()
    }
}

impl Eq for dyn ToBytesMixin {}

impl std::hash::Hash for dyn ToBytesMixin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_bytes().raw().hash(state);
    }
}
pub fn serialize_hex_item(hex: &str) -> Bytes {
    let mut bytes = Vec::new();
    for i in 0..hex.len() / 2 {
        let byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap();
        bytes.push(byte);
    }
    Bytes::new(Some(BytesFromType::Raw(bytes)))
}

pub fn serialize_item(item: &dyn ToBytesMixin) -> Bytes {
    let mut bytes = Vec::new();
    let item_bytes = item.to_bytes();
    let length = item_bytes.length() as i32;
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(&item_bytes.raw());

    Bytes::new(Some(BytesFromType::Raw(bytes)))
}

pub fn serialize_map(map: &HashMap<Box<dyn ToBytesMixin>, Box<dyn ToBytesMixin>>) -> Bytes {
    let mut bytes = Vec::new();
    let length = map.len() as i32;
    bytes.extend_from_slice(&length.to_be_bytes());

    for (key, value) in map {
        let key_bytes = serialize_item(key.as_ref());
        let value_bytes = serialize_item(value.as_ref());
        bytes.extend_from_slice(&key_bytes.raw());
        bytes.extend_from_slice(&value_bytes.raw());
    }

    Bytes::new(Some(BytesFromType::Raw(bytes)))
}

pub fn serialize_list(list: &Vec<Box<dyn ToBytesMixin>>) -> Bytes {
    let mut bytes = Vec::new();
    let length = list.len() as i32;
    bytes.extend_from_slice(&length.to_be_bytes());

    for item in list {
        let item_bytes = serialize_item(item.as_ref());
        bytes.extend_from_slice(&item_bytes.raw());
    }

    Bytes::new(Some(BytesFromType::Raw(bytes)))
}
