**QuantumCryptLib** is a Rust library for building **post-quantum secure communication channels** using **Kyber512** (a NIST-selected post-quantum Key Encapsulation Mechanism) combined with modern **AEAD encryption**.

Rather than encrypting data directly with Kyber, this library follows **industry-correct cryptographic design**:

> **Kyber KEM → Shared Secret → AEAD (ChaCha20-Poly1305)**

This approach protects today’s communications and stored data against **future quantum attacks** (“harvest now, decrypt later”).
>  **Warning**
>
> This crate provides **cryptographic building blocks only**.
>
> It does **NOT** implement:
> - Authentication or identity verification
> - Replay protection
> - Key lifecycle management
> - A full network or transport protocol (e.g., TLS)
>
> Users are responsible for integrating these primitives
> into a secure, authenticated protocol design.
---

##  Key Capabilities

-  **Post-quantum key establishment** using **Kyber512 (KEM)**
-  **Shared secret derivation** for secure sessions
-  **Authenticated encryption** via **ChaCha20-Poly1305**
-  Fully tested **secure channel handshake**
-  Written in **Rust 2021** for safety and performance

---

##  Cryptographic Model (Important)

Kyber512 is a **Key Encapsulation Mechanism (KEM)** — it is **not** used to encrypt application data directly.

QuantumCryptLib implements the recommended construction:

1. **Key Encapsulation (Kyber512)**  
   Establishes a shared secret between two parties
2. **Key Derivation**  
   Derives a symmetric encryption key from the shared secret
3. **AEAD Encryption (ChaCha20-Poly1305)**  
   Encrypts and authenticates application data

This model is used in:
- Post-quantum TLS / mTLS
- Zero-Trust service-to-service communication
- Secure tunnels and long-term data protection

---

##  Real-World Use Cases

- **Post-Quantum Secure Channels** (TLS / mTLS alternatives)
- **Zero-Trust Microservices Communication**
- **Long-Term Data Protection & Archival Encryption**
- **Financial, Government, and Critical Infrastructure Systems**
- **Blockchain & Distributed System Secure Messaging**

---

##  Installation
️⃣ **Clone the repository:**
```sh
git clone https://github.com/0rlych1kk4/quantumcryptlib.git
cd quantumcryptlib
```
## Build project:
```sh
cargo build
cargo run --bin quantumcryptlib_bin
```
## Usage
 **Generating Kyber Key Pair**
 ```sh
use quantumcryptlib::key_exchange::generate_key_pair;

let (public_key, secret_key) = generate_key_pair();
```
 **Establish a Post-Quantum Shared Secret (KEM)**
```sh
use quantumcryptlib::key_exchange::{encapsulate, decapsulate};

// Initiator
let (shared_secret_a, kem_ciphertext) = encapsulate(&public_key)?;

// Responder
let shared_secret_b = decapsulate(&secret_key, &kem_ciphertext)?;

assert_eq!(shared_secret_a, shared_secret_b);

 **Encrypt and Decrypt Data Using AEAD**
use quantumcryptlib::secure_channel::{aead_encrypt, aead_decrypt};

let message = b"hello post-quantum world";

// Encrypt
let (nonce, ciphertext) = aead_encrypt(&shared_secret_a, message)?;

// Decrypt
let plaintext = aead_decrypt(&shared_secret_b, &nonce, &ciphertext)?;

assert_eq!(message.to_vec(), plaintext);
```
---
##  Testing

Integration tests validate the following:

- Multiple independent KEM handshakes
- Shared secret correctness
- AEAD encryption and decryption cycles

### Run tests

```bash
cargo test
```
---
##  Security Notes

- Secrets are **never printed**
- AEAD ensures **confidentiality and authenticity**
- Kyber512 is **NIST-selected** for post-quantum key exchange
- This library is a **secure communication building block**, not a full TLS replacement
---
### Planned Hardening

- HKDF
- Hybrid classical + post-quantum key exchange
- Secret zeroization
- `no_std` support

---

##  Contributing

1. Fork the repository  
2. Create a feature branch  
   ```sh
   git checkout -b feature-branch
3. Commit your changes
   ```sh
   git commit -m "Add feature"
4. Push and open a pull request
---
