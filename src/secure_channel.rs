use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore,
    ChaCha20Poly1305,
    Nonce,
};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub enum ChannelError {
    EncryptFailed,
    DecryptFailed,
}

fn derive_key(shared_secret: &[u8]) -> [u8; 32] {
    let hash = Sha256::digest(shared_secret);
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash);
    key
}

pub fn aead_encrypt(
    shared_secret: &[u8],
    plaintext: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), ChannelError> {
    let key = derive_key(shared_secret);
    let cipher = ChaCha20Poly1305::new((&key).into());

    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| ChannelError::EncryptFailed)?;

    Ok((nonce.to_vec(), ciphertext))
}

pub fn aead_decrypt(
    shared_secret: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, ChannelError> {
    let key = derive_key(shared_secret);
    let cipher = ChaCha20Poly1305::new((&key).into());

    let nonce = Nonce::from_slice(nonce);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| ChannelError::DecryptFailed)?;

    Ok(plaintext)
}
