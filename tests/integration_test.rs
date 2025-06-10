use quantumcryptlib::key_exchange::{generate_key_pair, encrypt, decrypt};

#[test]
fn test_multiple_key_exchange_cycles() {
    for _ in 0..10 {
        let (pk, sk) = generate_key_pair();
        let result = encrypt(&pk).expect("Encryption failed");

        let (shared_secret, ciphertext) = result;
        let decrypted_secret = decrypt(&sk, &ciphertext).expect("Decryption failed");

        assert_eq!(shared_secret, decrypted_secret);
    }
}

