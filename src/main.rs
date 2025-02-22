use quantumcryptlib::key_exchange::{generate_key_pair, encrypt, decrypt};

fn main() {
    let (pk, sk) = generate_key_pair();

    match encrypt(&pk) {
        Ok((shared_secret, ciphertext)) => {
            println!("Shared Secret: {:?}", shared_secret);
            println!("Ciphertext: {:?}", ciphertext);

            match decrypt(&sk, &ciphertext) {
                Ok(decrypted_secret) => {
                    println!("Decrypted Secret: {:?}", decrypted_secret);
                    assert_eq!(shared_secret, decrypted_secret); // Ensure they match!
                }
                Err(e) => println!("Decryption error: {}", e),
            }
        }
        Err(e) => println!("Encryption error: {}", e),
    }
}

