use quantumcryptlib::key_exchange::{decapsulate, encapsulate, generate_key_pair};
use quantumcryptlib::secure_channel::{aead_decrypt, aead_encrypt};

fn main() {
    let (pk, sk) = generate_key_pair();

    match encapsulate(&pk) {
        Ok((shared_secret_initiator, kem_ciphertext)) => {
            // Responder derives the same secret using sk + kem_ciphertext
            match decapsulate(&sk, &kem_ciphertext) {
                Ok(shared_secret_responder) => {
                    assert_eq!(shared_secret_initiator, shared_secret_responder);
                    println!("KEM handshake OK");

                    let msg = b"hello post-quantum world";
                    let (nonce, ct) =
                        aead_encrypt(&shared_secret_initiator, msg).expect("AEAD encrypt failed");

                    let pt = aead_decrypt(&shared_secret_responder, &nonce, &ct)
                        .expect("AEAD decrypt failed");

                    assert_eq!(msg.to_vec(), pt);
                    println!("AEAD encrypt/decrypt OK");
                }
                Err(e) => println!("Decapsulation error: {:?}", e),
            }
        }
        Err(e) => println!("Encapsulation error: {:?}", e),
    }
}
