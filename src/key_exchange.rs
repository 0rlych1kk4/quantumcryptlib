use pqcrypto_kyber::kyber512::{
    keypair,
    PublicKey,
    SecretKey,
    Ciphertext,
    encapsulate as kyber_encapsulate,
    decapsulate as kyber_decapsulate,
};
use pqcrypto_traits::kem::{
    PublicKey as _,
    SecretKey as _,
    Ciphertext as _,
    SharedSecret as _,
};

#[derive(Debug)]
pub enum KemError {
    InvalidPublicKey,
    InvalidSecretKey,
    InvalidCiphertext,
}

/// Generates a Kyber512 key pair (public key, secret key).
pub fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
    let (pk, sk) = keypair();
    (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
}

/// Kyber KEM encapsulation
pub fn encapsulate(public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), KemError> {
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| KemError::InvalidPublicKey)?;

    let (shared_secret, ciphertext) = kyber_encapsulate(&pk);

    Ok((
        shared_secret.as_bytes().to_vec(),
        ciphertext.as_bytes().to_vec(),
    ))
}

/// Kyber KEM decapsulation
pub fn decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, KemError> {
    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| KemError::InvalidSecretKey)?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| KemError::InvalidCiphertext)?;

    let shared_secret = kyber_decapsulate(&ct, &sk);
    Ok(shared_secret.as_bytes().to_vec())
}
