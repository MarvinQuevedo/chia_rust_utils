use std::io::Read;

use chia_bls::secret_key::SecretKey;
pub fn secret_key_from_stream(iterator: &mut std::slice::Iter<u8>) -> SecretKey {
    let mut secret_key_bytes = [0u8; 32];
    for i in 0..32 {
        secret_key_bytes[i] = *iterator.next().unwrap();
    }

    SecretKey::from_bytes(&secret_key_bytes).unwrap()
}
