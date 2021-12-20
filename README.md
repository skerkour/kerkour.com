# (Ab)using technology for fun & profit

Code accompanying my blog [https://kerkour.com](https://kerkour.com)

## Install Rust

```ShellSession
curl <https://sh.rustup.rs> -sSf | sh
```

## Install Rust Updater

```ShellSession
cargo install cargo-update && cargo install cargo-audit && cargo install cargo-audit --features=fix
```

## Fix Cargo Vulnerabilities

```ShellSession
cargo install-update -a && cargo update && cargo audit fix
```

## 2022

* [4 ways to read a file in Rust](https://kerkour.com/rust-read-file)
* [How to encrypt a file in Rust (Using streaming AEAD encryption)](https://kerkour.com/rust-file-encryption)
* [Benchmarking symmetric encryption (AEAD) in Rust](https://kerkour.com/rust-symmetric-encryption-aead-benchmark/)
* [Building a static site generator in 100 lines of Rust](https://kerkour.com/rust-static-site-generator)
* [Reproductibe cross-compilation for Rust (with Docker)](https://kerkour.com/rust-reproductible-cross-compilation-with-docker/)
* [Rust on ESP32](https://kerkour.com/rust-on-esp32)
* [How to implement HTTP Long Polling in Go](https://kerkour.com/go-http-long-polling/)
* [How to implement HTTP Long Polling in Rust](https://kerkour.com/rust-http-long-polling/)
* [How to Write and Compile Shellcode in Rust](https://kerkour.com/shellcode-in-rust/)
* [How to build a job queue with Rust and PostgreSQL](https://kerkour.com/rust-job-queue-with-postgresql/)
* [How to sort a vector in Rust](https://kerkour.com/rust-how-to-sort-a-vector/)
* [How to deal with large Cargo workspaces in Rust](https://kerkour.com/rust-large-cargo-workspace/)
* [Rust, How to convert String to Int and Int to String](https://kerkour.com/rust-string-to-int-and-int-to-string/)
* [A fast port scanner in 100 lines of Rust](https://kerkour.com/rust-fast-port-scanner/)
* [15k inserts/s with Rust and SQLite](https://kerkour.com/high-performance-rust-with-sqlite/)
* [How to deploy Rust on Heroku (with Docker)](https://kerkour.com/deploy-rust-on-heroku-with-docker/)
* [How to implement worker pools in Rust](https://kerkour.com/rust-worker-pool/)
* [How to create small Docker images for Rust](https://kerkour.com/rust-small-docker-image/)
* [How to execute shellcodes from memory in Rust](https://kerkour.com/rust-execute-from-memory/)
* [How to send emails with Rust](https://kerkour.com/rust-send-email/)
