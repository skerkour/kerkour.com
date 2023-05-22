+++
date = 2022-02-09T06:00:00Z
title = "The foundations of end-to-end encryption: Domain separation (with code example in Rust)"
type = "post"
tags = ["rust",  "cryptography", "programming", "tutorial", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/end-to-end-encryption-domain-separation-cryptography"

[extra]
lang = "en"

comment ="""
"""
+++


> This post is part 2 of the series: [The foundations of end-to-end encryption](/tags/cryptography) and contains excerpts from my book [Black Hat Rust](/black-hat-rust) about Security, Rust and Cryptography.


## Building an end-to-end encrypted service


Let's say we want to build a service using end-to-end encryption to secure our users' data, a hosted [password manager](https://en.wikipedia.org/wiki/Password_manager) or an encrypted files hosting service for example.

To provide a good user experience, we want our users to have to remember only 1 password that is used both for authentication and encryption of the data.

It's very usual for internet services to use a password for authentication. Usually, the password is sent to the server, hashed, and compared to the hash of the password computed during the account creation.

[Also, as we saw last month](/rust-file-encryption-chacha20poly1305-argon2), it's possible to turn a cryptographically insecure password into a key that can be used for encryption using a **Password-based Key Derivation Function (PBKDF)**.


But, how to use the same password for both encryption and authentication?

Indeed, we can't send the password or the key derived from the password to the server, it would completely defeat the end-to-end encryption.

Also, it's a very bad idea in cryptography to use the same key for different purposes.


![Using a single key](https://kerkour.com/2022/end-to-end-encryption-domain-separation/single_key.png)



## Domain separation

This is where domain separation comes into play.

With domain separation, we can generate multiple secure keys from a single secure key.

1 key for authentication with the server and 1 key for encryption, for example.


![Domain Separation](https://kerkour.com/2022/end-to-end-encryption-domain-separation/domain_separation.png)


With the same password, we can deterministically generate the same encryption and auth keys whenever we want.



## Talk is cheap. Show me the code.

First, we define our constants:

```rust
const ENCRYPTION_DOMAIN: &str = "encryption";
const AUTH_DOMAIN: &str = "auth";
const KEY_SIZE: usize = 32;
```

Then, we ask for the password and generate a random salt that we will use for argon2id hashing:

```rust
fn main() -> Result<(), anyhow::Error> {
    let mut password = rpassword::prompt_password_stdout("password:")?;
    let argon2_config = argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    };

    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
```

Then we derive the `master_key` from the password and the `salt`:

```rust
    let mut master_key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;
```

Then come the interesting bits: we derive our two different `encryption_key` and `auth_key` from our `master_key` with **blake2b** in keyed mode, with the `master_key` as key and the 2 domain strings as data:

```rust
    let mut encryption_kdf = blake2::VarBlake2b::new_keyed(&master_key, KEY_SIZE);
    encryption_kdf.update(ENCRYPTION_DOMAIN.as_bytes());
    let mut encryption_key = encryption_kdf.finalize_boxed();

    let mut auth_kdf = blake2::VarBlake2b::new_keyed(&master_key, KEY_SIZE);
    auth_kdf.update(AUTH_DOMAIN.as_bytes());
    let mut auth_key = auth_kdf.finalize_boxed();
```


Finally, we display the keys and clean all the sensitive materials from memory.

```rust
    println!("master key:     {}", hex::encode(&master_key));
    println!("encryption key: {}", hex::encode(&encryption_key));
    println!("auth key:       {}", hex::encode(&auth_key));

    password.zeroize();
    master_key.zeroize();
    encryption_key.zeroize();
    auth_key.zeroize();

    Ok(())
}
```

```bash
$ cargo run
password:
master key:     a2edd54522eedbf7b0e393259284dcc6e8186a348ad65687c1af43c10d641801
encryption key: 7ae16b005a4ff85455faf639491dc51ce0f35434f966cfe28092ac9ff8b0d183
auth key:       046511347fae786b95b03a575980fa9d6fea21f2c0415b89c709e3e8d429594d
```


## Some Closing Thoughts

Is it enough to build our end-to-end encrypted service? No!

We may want to encrypt each individual piece of data (site credentials in the case of a password manager) with a different encryption key. That will be the next part of the series, so stay tuned :)


As usual, you can find the complete code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2022/end_to_end_encryption_domain_separation) (please don't forget to star the repo 🙏).

**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build our own end-to-end encrypted protocol to secure the communication of a Remote Access Tool in  Rust.**
