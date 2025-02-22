// src/key_exchange.rs
use pqcrypto::kem::kyber512::{self, Kem};

pub fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
    let (public_key, secret_key) = kyber512::keypair();
    // Return raw byte representations of keys
    (public_key.to_vec(), secret_key.to_vec())
}

pub fn encrypt(public_key: &[u8]) -> Vec<u8> {
    let ciphertext = kyber512::enc(public_key).unwrap();
    ciphertext.to_vec()
}

pub fn decrypt(secret_key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let shared_secret = kyber512::dec(secret_key, ciphertext).unwrap();
    shared_secret.to_vec()
}

