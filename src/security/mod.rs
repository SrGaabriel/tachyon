pub mod compression;

use rand::{Rng, RngCore};
use rsa::{Pkcs1v15Encrypt, Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey};
use sha1::Digest;

pub struct Keypair {
    pub public: RsaPublicKey,
    pub private: RsaPrivateKey
}

pub fn generate_keypair() -> rsa::errors::Result<Keypair> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);
    Ok(Keypair {
        public: public_key,
        private: private_key
    })
}

pub fn generate_verify_token() -> Vec<u8> {
    let mut token = vec![0; 4];
    rand::thread_rng().fill_bytes(&mut token);
    token
}

pub fn verify_nonce(
    expected: Vec<u8>,
    received: Vec<u8>,
    decrypting_key: RsaPrivateKey
) -> bool {
    let decrypted_nonce = decrypt(decrypting_key, received);
    match decrypted_nonce {
        Ok(nonce) => nonce == expected,
        Err(_) => false
    }
}

pub fn decrypt(key: RsaPrivateKey, data: Vec<u8>) -> rsa::errors::Result<Vec<u8>> {
    key.decrypt(Pkcs1v15Encrypt, &data)
}

pub fn transform_byte_vec(vec: &mut Vec<u8>) -> Vec<i8> {
    let mut handicapped = Vec::new();
    for byte in vec.iter() {
        handicapped.push(*byte as i8);
    }
    handicapped
}