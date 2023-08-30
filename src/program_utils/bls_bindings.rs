use blst::BLST_ERROR;
use chia_bls::{signature, PublicKey, SecretKey, Signature};
use chia_protocol::{Bytes48, Bytes96};
pub struct AugSchemeMPL {}
impl AugSchemeMPL {
    pub fn aggregate(sigs: Vec<chia_bls::Signature>) -> Bytes96 {
        let mut sigs_converted: Vec<chia_bls::Signature> = Vec::new();
        for sig in sigs {
            let sig_bytes = sig.to_bytes();
            let sig = signature::Signature::from_bytes(&sig_bytes).unwrap();
            sigs_converted.push(sig);
        }

        let bytes = signature::aggregate(&sigs_converted).to_bytes();
        Bytes96::from(bytes)
    }
    pub fn sign(sk: &SecretKey, message: Vec<u8>) -> Signature {
        let sig = signature::sign(&sk, &message);
        sig
    }
}
