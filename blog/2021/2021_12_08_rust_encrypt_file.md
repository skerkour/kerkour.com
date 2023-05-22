+++
date = 2021-12-08T06:00:00Z
title = "How to encrypt a file in Rust (Using streaming AEAD encryption)"
type = "post"
tags = ["hacking", "security", "programming", "rust", "tutorial", "cryptography"]
authors = ["Sylvain Kerkour"]
url = "/rust-file-encryption"

[extra]
lang = "en"

comment ="""
Using streaming encryption to encrypt large files in Rust (Streaming AEAD)
"""
+++

## Choosing the right encryption algorithm


When you want to encrypt data, you may face a problem: how to encrypt files or data streams that don't fit in memory?

You could simply split them into small chunks and encrypt each fragment individually. Unfortunately, it's not that simple.

An attacker could modify or reorder the chunks making decryption impossible.

This is where **streaming AEAD encryption** comes into play, to guarantee that the entire data stream has not been modified, corrupted, or reordered. You can read more about the benefits of stream ciphers in [libsodium's documentation](https://libsodium.gitbook.io/doc/secret-key_cryptography/secretstream) (but we won't use it).

In the first part of this post, we are going to see how to encrypt small files that can fit in memory, and, in the second part, how to use our AEAD cipher in stream mode to encrypt larger files or data streams that can't be encrypted in one operation.


We will use `XChaCha20-Poly1305` because [I particularly appreciate its simplicity and speed](https://kerkour.com/rust-symmetric-encryption-aead-benchmark). That being said, the RustCrypto organization uses a [trait](https://docs.rs/aead) for all their AEAD implementations. Thus, replacing `XChaCha20-Poly1305` by `AES-256-GCM` (for example) is just a few keystrokes away.


As always, the code is on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_file_encryption) (please don't forget to star the repo 🙏).


## Setup

[**Cargo.toml**](https://github.com/skerkour/kerkour.com/blob/main/2021/rust_file_encryption/Cargo.toml)
```toml
[package]
name = "rust_file_encryption"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
chacha20poly1305 = { version = "0.9.0", features = ["stream"] }
anyhow = "1.0"
rand = "0.8"
```

In both cases, the first step is to securely generate a key and a nonce.  `XChaCha20-Poly1305`'s key is **32 bytes** long, and its nonce is **24 bytes** long.

[**main.rs**](https://github.com/skerkour/kerkour.com/blob/main/2021/rust_file_encryption/src/main.rs)
```rust
let mut key = [0u8; 32];
let mut nonce = [0u8; 24];
OsRng.fill_bytes(&mut key);
OsRng.fill_bytes(&mut nonce);
```



## Encrypting small files

Encrypting small files that can be read entirely in memory is pretty straightforward:

```rust
fn encrypt_small_file(
    filepath: &str,
    dist: &str,
    key: &[u8; 32],
    nonce: &[u8; 24],
) -> Result<(), anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());

    let file_data = fs::read(filepath)?;

    let encrypted_file = cipher
        .encrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Encrypting small file: {}", err))?;

    fs::write(&dist, encrypted_file)?;

    Ok(())
}
```

## Decrypting small files

The decryption is the same. You simply need to call `decrypt` instead of `encrypt`.

```rust
fn decrypt_small_file(
    encrypted_file_path: &str,
    dist: &str,
    key: &[u8; 32],
    nonce: &[u8; 24],
) -> Result<(), anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());

    let file_data = fs::read(encrypted_file_path)?;

    let decrypted_file = cipher
        .decrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

    fs::write(&dist, decrypted_file)?;

    Ok(())
}
```

## Encrypting large files

Encrypting larger files that can't fit in memory either because they weigh many GBs, or because you want to limit your resources usage, is where all the fun is.

For that, we need to use our AEAD cipher in stream mode.

```rust
fn encrypt_large_file(
    source_file_path: &str,
    dist_file_path: &str,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> Result<(), anyhow::Error> {
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());
```

Then we need a buffer to read the file chunk by chunk.

```rust
    const BUFFER_LEN: usize = 500;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut source_file = File::open(source_file_path)?;
    let mut dist_file = File::create(dist_file_path)?;
```

And finally, the magic.

First, we read a chunk of data from the file we want to encrypt.

If the number of bytes read is equal to the size of the buffer, it means that we didn't reach the end of the file. Thus, we call `encrypt_next` on our `stream_encryptor`.

Otherwise, a `read_count < BUFFER_LEN` means that we reached the end of the file, and we need to call the special method `encrypt_last` on our `stream_encryptor`.

```rust
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

    Ok(())
}
```

Note that `encrypt_next` and `encrypt_last` allocate. Thus, our function uses roughly `BUFFER_LEN * 2` bytes of memory. We could have used [`encrypt_next_in_place`](https://docs.rs/aead/latest/aead/stream/struct.Encryptor.html#method.encrypt_next_in_place) and [`encrypt_last_in_place`](https://docs.rs/aead/latest/aead/stream/struct.Encryptor.html#method.encrypt_last_in_place) to avoid the allocation.

## Decrypting large files

Decrypting the stream is not exactly the same as encrypting it. A small tweak is needed.

```rust
fn decrypt_large_file(
    encrypted_file_path: &str,
    dist: &str,
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> Result<(), anyhow::Error> {
```

```rust
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_ref().into());
```

Then comes the tweak. Notice that our buffer is no longer 500 bytes long, but 516.

```rust
    const BUFFER_LEN: usize = 500 + 16;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut encrypted_file = File::open(encrypted_file_path)?;
    let mut dist_file = File::create(dist)?;
```

Why a buffer size of **500 + 16** bytes? Because `XChaCha20-Poly1305` is an [AEAD](https://en.wikipedia.org/wiki/Authenticated_encryption) cipher and appends a **16 bytes** [authentication tag](https://en.wikipedia.org/wiki/Message_authentication_code) to each encrypted message. Thus, each encrypted chunk has a size of **516 bytes**: **500** bytes for the encrypted piece of data, and **16** bytes for the authentication tag.

```rust
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

    Ok(())
}
```


## The code is on GitHub

As always, the code is on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_file_encryption) (please don't forget to star the repo 🙏).
