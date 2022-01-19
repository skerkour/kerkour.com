

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
