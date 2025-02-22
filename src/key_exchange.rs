use pqcrypto_kyber::kyber512::*;
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _};

/// Generates a Kyber512 key pair (public key, secret key).
pub fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
    let (public_key, secret_key) = keypair();
    (public_key.as_bytes().to_vec(), secret_key.as_bytes().to_vec())
}

/// Encrypts using the given public key, returning the shared secret and ciphertext.
pub fn encrypt(public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| "Invalid public key format".to_string())?;

    let (shared_secret, ciphertext) = encapsulate(&pk);

    Ok((shared_secret.as_bytes().to_vec(), ciphertext.as_bytes().to_vec()))
}

/// Decrypts using the given secret key and ciphertext, returning the shared secret.
pub fn decrypt(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| "Invalid secret key format".to_string())?;

    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| "Invalid ciphertext format".to_string())?;

    let shared_secret = decapsulate(&ct, &sk);

    Ok(shared_secret.as_bytes().to_vec())
}
