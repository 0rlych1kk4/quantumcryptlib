use quantumcryptlib::key_exchange::{decapsulate, encapsulate, generate_key_pair};
use quantumcryptlib::secure_channel::{aead_decrypt, aead_encrypt};

#[test]
fn test_multiple_pq_secure_channel_cycles() {
    for _ in 0..10 {
        let (pk, sk) = generate_key_pair();

        let (ss_a, kem_ct) = encapsulate(&pk).expect("Encapsulation failed");
        let ss_b = decapsulate(&sk, &kem_ct).expect("Decapsulation failed");
        assert_eq!(ss_a, ss_b);

        let msg = b"test message";
        let (nonce, ct) = aead_encrypt(&ss_a, msg).expect("AEAD encrypt failed");
        let pt = aead_decrypt(&ss_b, &nonce, &ct).expect("AEAD decrypt failed");

        assert_eq!(msg.to_vec(), pt);
    }
}
