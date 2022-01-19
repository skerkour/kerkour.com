# [How to encrypt a file with a password in Rust (using Streaming Encryption + Argon2)](https://kerkour.com/rust-file-encryption-chacha20poly1305-argon2/)

## Usage


```bash
$ cargo run -- file.bin
$ cargo run -- file.bin.encrypted
$ shasum -a 256 file.bin file.bin.encrypted file.bin.decrypted
```

## Setup

```bash
$ head -c 512 </dev/urandom > file.bin
```
