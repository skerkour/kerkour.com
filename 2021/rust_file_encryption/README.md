# [How to encrypt a file in Rust (Using streaming AEAD encryption)](https://kerkour.com/rust-file-encryption)


```shell
$ cargo run
$ shasum -a 256 100.bin 100.encrypted 100.decrypted
$ shasum -a 256 2048.bin 2048.encrypted 2048.decrypted
$ shasum -a 256 500.bin 500.encrypted 500.decrypted
```


```bash
$ head -c 500 </dev/urandom >500.bin
```
