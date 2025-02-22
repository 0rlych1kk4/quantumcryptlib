# QuantumCryptLib

 **QuantumCryptLib** is a post-quantum cryptographic library built using **Kyber512** (from the `pqcrypto_kyber` crate). It provides quantum-resistant **key exchange**, **encryption**, and **decryption** functionalities.

## Features
-  Quantum-resistant **key exchange** (Kyber512)
-  Secure **encryption & decryption** using Kyber512's encapsulation mechanism
-  Lightweight and efficient for **post-quantum security applications**
-  Written in **Rust**  for high performance and safety

---

##  Installation
️⃣ **Clone the repository:**
```sh
git clone https://github.com/0rlych1kk4/quantumcryptlib.git
cd quantumcryptlib

## Build project:
cargo build
cargo run --bin quantumcryptlib_bin

## Usage
 **Generating Key Pairs**
use quantumcryptlib::key_exchange::generate_key_pair;

let (public_key, secret_key) = generate_key_pair();
println!("Public Key: {:?}", public_key);
println!("Secret Key: {:?}", secret_key);

 **Encrypting Data**
use quantumcryptlib::key_exchange::encrypt;

let (shared_secret, ciphertext) = encrypt(&public_key).expect("Encryption failed");
println!("Shared Secret: {:?}", shared_secret);
println!("Ciphertext: {:?}", ciphertext);

 **Decrypting Data**
use quantumcryptlib::key_exchange::decrypt;

let decrypted_secret = decrypt(&secret_key, &ciphertext).expect("Decryption failed");
assert_eq!(shared_secret, decrypted_secret);

## Contributing
 **Contributions are welcome!**

Fork the repo
Create a new branch (git checkout -b feature-branch)
Commit your changes (git commit -m "Added a new feature")
Push to your branch (git push origin feature-branch)
Submit a pull request
