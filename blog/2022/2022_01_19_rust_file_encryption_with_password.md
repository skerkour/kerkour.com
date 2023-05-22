+++
date = 2022-01-19T10:00:00Z
title = "How to securely encrypt a file with an insecure password in Rust (using Streaming Encryption + Argon2)"
type = "post"
tags = ["hacking", "security", "rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-file-encryption-chacha20poly1305-argon2"

[extra]
lang = "en"

comment ="""
"""
+++


[Last month we saw how to encrypt large files that don't fit in memory](https://kerkour.com/rust-file-encryption) using a streaming cipher. The article presupposes that you already have a secure key generation mechanism.

Today, we are going to see how to securely encrypt data using a cryptographically insecure password. Passwords can't be used as keys for encryption: they are not secure (random) enough and most of the time don't have the required size. But, by using a **Password-based Key Derivation Function (PBKDF)**, we can turn our insecure password into a secure random key. Let's find how!


> This post contains excerpts from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)



## Key Derivation Functions

Key Derivation Functions (KDFs) allow creating a secure key from a not-so-secure source.


![Key Derivation Functions](https://kerkour.com/2022/rust-file-encryption-chacha20poly1305-argon2/kdf.png)


There are two kinds of Key Derivation Functions:

Functions that accept as input a low entropy input, such as a password, a passphrase or a big number, and produce a high-entropy, secure output. They are known as **PBKDF for Password-Based Key Derivation Functions**. For example `Argon2d` and `PBKDF2`.

And functions that accept a high-entropy input, such as an already securely generated random vector, and produce an also high-entropy output. For example: `Blake2b`.

Note that a function like `Blake2b` is polyvalent, and you can also use it with a secret key as a MAC.

## Why Argon2

[Argon2](https://en.wikipedia.org/wiki/Argon2) was selected as the winner of the Password Hashing Competition in July 2015 and is thus recommended by most security professionals to hash passwords.

Argon2 provides 3 variants:
* Argon2d, which is optimized to resist GPU cracking attacks.
* Argon2i, which is optimized to resist [side channel attacks](https://en.wikipedia.org/wiki/Side-channel_attack).
* Argon2id, which is a hybrid version of the two above, combining their properties.

Argon2id let's you specify how much resources it is going to use to hash your password which is also the resources an attacker is going to use for each guess of a bruteforce attack.

You can find recommendations for the parameters, depending on your use case, on the [OWASP Password Storage Cheat Sheet page](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html) or in the [Section 4 - Parameter Choice of RFC 9106](https://datatracker.ietf.org/doc/rfc9106/).


If you have no specific reason not to use it, **you should use Argon2id**.


## Using Argon2 to turn a password into a key

```rust
fn argon2_config<'a>() -> argon2::Config<'a> {
    return argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    };
}

let password = "password";
let mut salt = [0u8; 32];
OsRng.fill_bytes(&mut salt);

let argon2_config = argon2_config();

let mut key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;
```

The code above turns an insecure password `password` into a secure (high entropy) `32 bytes` key that can then be used for encryption.

Please note that the salt SHOULD NOT be discarded as we are going to need it for the decryption.


## Encrypting the file

```rust
fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: ./encryptor <file>"));
    }

    let file = args[1].clone();
    let mut password = rpassword::prompt_password_stdout("password:")?;

    let dist = file.clone() + ".encrypted";
    encrypt_file(&file, &dist, &password)?;

    password.zeroize();

    Ok(())
}

fn encrypt_file(
    source_file_path: &str,
    dist_file_path: &str,
    password: &str,
) -> Result<(), anyhow::Error> {
    let argon2_config = argon2_config();

    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 19];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let mut key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;

    let aead = XChaCha20Poly1305::new(key[..32].as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

    let mut source_file = File::open(source_file_path)?;
    let mut dist_file = File::create(dist_file_path)?;

    dist_file.write(&salt)?;
    dist_file.write(&nonce)?;
```

First, we turn our `password` into a secure 32 bytes `key`.

Then, we append the `salt` and the `nonce` to the encrypted file in order to use them later when decrypting the file. These values are safe to be publicly available.

Why a 19 bytes nonce while `XChaCha20Poly1305` uses a 24 bytes nonce? Because the [last 5 bytes of the nonce are used for a 32 bits counter, and a 1-byte "last block" flag](https://docs.rs/aead/latest/aead/stream/struct.StreamBE32.html), so we only need to generate 19 bytes of random data.


```rust
    const BUFFER_LEN: usize = 500;
    let mut buffer = [0u8; BUFFER_LEN];

    loop {
        let read_count = source_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let ciphertext = stream_encryptor
                .encrypt_next(buffer.as_slice())
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dist_file.write(&ciphertext)?;
        } else {
            let ciphertext = stream_encryptor
                .encrypt_last(&buffer[..read_count])
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dist_file.write(&ciphertext)?;
            break;
        }
    }

    salt.zeroize();
    nonce.zeroize();
    key.zeroize();

    Ok(())
}
```



Then, we encrypt the file 500 bytes by 500 bytes. Using a small buffer allows us to encrypt a file or stream of any size, even if it doesn't fit in RAM.

Finally, we `zeroize` the sensitive variables to clear them out from memory.

## Decrypting the file


```rust
fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: ./encryptor <file>"));
    }

    let file = args[1].clone();
    let mut password = rpassword::prompt_password_stdout("password:")?;

    let dist = file.strip_suffix(".encrypted").unwrap().to_string() + ".decrypted";
    decrypt_file(&file, &dist, &password)?;

    password.zeroize();

    Ok(())
}

fn decrypt_file(
    encrypted_file_path: &str,
    dist: &str,
    password: &str,
) -> Result<(), anyhow::Error> {
    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 19];

    let mut encrypted_file = File::open(encrypted_file_path)?;
    let mut dist_file = File::create(dist)?;

    let mut read_count = encrypted_file.read(&mut salt)?;
    if read_count != salt.len() {
        return Err(anyhow!("Error reading salt."));
    }

    read_count = encrypted_file.read(&mut nonce)?;
    if read_count != nonce.len() {
        return Err(anyhow!("Error reading nonce."));
    }

    let argon2_config = argon2_config();

    let mut key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;

    let aead = XChaCha20Poly1305::new(key[..32].as_ref().into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_ref().into());
```

```rust
    const BUFFER_LEN: usize = 500 + 16;
    let mut buffer = [0u8; BUFFER_LEN];

    loop {
        let read_count = encrypted_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let plaintext = stream_decryptor
                .decrypt_next(buffer.as_slice())
                .map_err(|err| anyhow!("Decrypting large file: {}", err))?;
            dist_file.write(&plaintext)?;
        } else if read_count == 0 {
            break;
        } else {
            let plaintext = stream_decryptor
                .decrypt_last(&buffer[..read_count])
                .map_err(|err| anyhow!("Decrypting large file: {}", err))?;
            dist_file.write(&plaintext)?;
            break;
        }
    }

    salt.zeroize();
    nonce.zeroize();
    key.zeroize();

    Ok(())
}
```

Decrypting the file consists of doing the invert operations:
* Read `salt` and `nonce` from encrypted file
* Generate the `key` with Argon2id from `password` and `salt`.
* Decrypt the file with `key` and `nonce`
* Clear the sensitive variable from memory


## Some Closing Thoughts

But, if the salt is public, how is this system secure against bruteforce attacks?

Of course, you can't expect this system to protect your data if your password is `password`. That being said, by playing with Argon2's parameters, you can find a tradeoff that fit your requirements.

For example, with the parameters above, deriving a key takes ~2,5 second on my 4 cores machine.



## The code is on GitHub

As always, the code is on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2022/rust_file_encryption_with_password) (please don't forget to star the repo 🙏).




**Want to learn more applied cryptography? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build our own end-to-end encrypted protocol to secure the communication of a Remote Access Tool.**
