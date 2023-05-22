+++
date = 2022-03-01T03:00:00Z
title = "The foundations of end-to-end encryption: Key exchange (with code example in Rust)"
type = "post"
tags = ["rust",  "cryptography", "programming", "tutorial", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/end-to-end-encryption-key-exchange-cryptography-rust"

[extra]
lang = "en"

comment ="""
"""
+++


> This post is part 3 of the series: [The foundations of end-to-end encryption](/tags/cryptography) and contains excerpts from my book [Black Hat Rust](/black-hat-rust) about Security, Rust and Cryptography.


When 2 parties, let's say Alice and Bob, want to exchange messages over an insecure channel, they need to find a way to share a common secret that can't be guessed by potential eavesdroppers.


![Communication with encryption](/2021/signatures-modern-end-to-end-encryption/encrypted_communication.svg)



## Public-key cryptography

![Asymetric encryption](/2022/key-exchange/ch11_e2ee_public_key.png)


Because I need to know Bob's public key before sending him a message, his public key is kind of his digital **identity**. Usually, I can get Bob's public key through the same app I'm using to send him a message, but I need to verify (using another channel, like a face-to-face meeting) with him that the public key the service served me is Bob's one and not a malicious one.


Because only the owner of the private key is able to decrypt content encrypted with the public key, from a cryptographic point of view, **1 public key = 1 identity**.


Is it enough to secure our communication? Could we simply encrypt our message with [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) and call it a day?


Wait a minute!

Reality is quite different: public-key encryption is limited in the length of the messages it can encrypt, is painfully slow, and prone to bugs, especially RSA.


## Diffie–Hellman key exchange


The modern way to achieve that is called [Diffie–Hellman key exchange](https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange) (or, more commonly, key exchange). It's a method to establish a **shared secret** between two parties through a public channel.

The same shared secret can be derived from Alice's public key and Bob's private key than from Bob's public key and Alice's private key. Thus, both Alice and Bob can compute the same shared secret using their respective private keys and the other one's public key.

Nowadays, the recommended key exchange functions to use are [Elliptic-curve Diffie–Hellman (**ECDH**)](https://en.wikipedia.org/wiki/Elliptic-curve_Diffie%E2%80%93Hellman), which are way simpler to implement than RSA encryption.


However,  shared secrets computed through ECDH key exchange can't be used directly for symmetric encryption. Most AEAD algorithms expect a uniformly random symmetric key which shared secrets are not. Thus, to "increase their entropy", we pass the output of the key exchange function into a **Key Derivation Function (KDF)** to generate a shared secret key that can be used for symmetric encryption.

![Key exchange](/2022/key-exchange/ch11_key_exchange.png)


The (certainly) most famous and used Key Exchange algorithm (and the one I recommend you to use if you have no specific requirement) is: `x25519`.


## Talk is cheap. Show me the code.

We are going to use the [x25519-dalek](https://crates.io/crates/x25519-dalek) for key exchange, and [blake2](https://crates.io/crates/blake2) for KDF.

**Cargo.toml**
```toml
[package]
name = "rust_key_exchange"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
blake2 = "0.9"
x25519-dalek = "1"
rand = "0.7"
```

First the constants.

**main.rs**
```rust
const XCHACHA20_POLY1305_KEY_SIZE: usize = 32;
pub const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24;
```

Then we generate the keypairs (private key, public key) for both Alice and Bob, and the nonce used for symmetric encryption.

```rust
fn main() {
    let mut rand_generator = OsRng {};

    let mut nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
    rand_generator.fill_bytes(&mut nonce);

    let alice_private_key = StaticSecret::new(rand_generator);
    let alice_public_key = PublicKey::from(&alice_private_key);

    let bob_private_key = StaticSecret::new(rand_generator);
    let bob_public_key = PublicKey::from(&bob_private_key);
```

To derive the shared secret for Bob, we need his private key and Alice's public key.

We then use the Diffie–Hellman secret with `blake2b` in keyed mode to generate our cryptographically secure key that can be used for encryption.

```rust
fn derive_secret_for_bob(
    bob_private_key: &StaticSecret,
    alice_public_key: &PublicKey,
    nonce: &[u8; XCHACHA20_POLY1305_NONCE_SIZE],
) -> Vec<u8> {
    let dh_secret = bob_private_key.diffie_hellman(&alice_public_key);

    let mut kdf = blake2::VarBlake2b::new_keyed(dh_secret.as_bytes(), XCHACHA20_POLY1305_KEY_SIZE);
    kdf.update(nonce);
    let shared_key = kdf.finalize_boxed();

    return shared_key.into();
}
```

Deriving Alice's shared key is the same operation but using her private key and Bob's public key instead.

```rust
fn derive_secret_for_alice(
    alice_private_key: &StaticSecret,
    bob_public_key: &PublicKey,
    nonce: &[u8; XCHACHA20_POLY1305_NONCE_SIZE],
) -> Vec<u8> {
    let dh_secret = alice_private_key.diffie_hellman(&bob_public_key);

    let mut kdf = blake2::VarBlake2b::new_keyed(dh_secret.as_bytes(), XCHACHA20_POLY1305_KEY_SIZE);
    kdf.update(nonce);
    let shared_key = kdf.finalize_boxed();

    return shared_key.into();
}
```

Finally, we verify that both parties have the exact same key.

```rust
    let bob_secret = derive_secret_for_bob(&bob_private_key, &alice_public_key, &nonce);
    let alice_secret = derive_secret_for_alice(&alice_private_key, &bob_public_key, &nonce);

    assert!(bob_secret == alice_secret);

    println!("Everything is good!");
}
```


Now we have our shared key that can be used with an [AEAD function such as XChaCha20-Poly1305 to encrypt our messages](/rust-file-encryption), for example.


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2022/rust_key_exchange) (please don't forget to star the repo 🙏).


**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build our own end-to-end encrypted protocol to secure the communication of a Remote Access Tool in  Rust.**
