+++
date = 2021-08-24T14:00:00Z
title = "Overview of the Rust cryptography ecosystem"
type = "post"
tags = ["rust", "programming", "cryptography", "hacking", "security"]
authors = ["Sylvain Kerkour"]
url = "/rust-cryptography-ecosystem"

[extra]
lang = "en"

comment ="""
Intro?
Soit parler des failles lies a la securite dans les libs de crypto
soit parler des requirements pour une bon language crpyto
"""
+++


**37.2% of vulnerabilities in cryptographic libraries are memory safety issues**, while only 27.2%  are cryptographic issues, according to an [empirical Study of Vulnerabilities in Cryptographic Libraries](https://arxiv.org/abs/2107.04940) (*Jenny Blessing, Michael A. Specter, Daniel J. Weitzner - MIT)*.

I think it's time that we move on from C as the *de-facto* language for implementing cryptographic primitive.

> This post is an excerpt of my book [Black Hat Rust](https://kerkour.com/black-hat-rust)

Due to its high-level nature with low-level controls, absence of garbage collector, portability, and [ease of embedding](https://doc.rust-lang.org/nomicon/ffi.html#calling-rust-code-from-c), Rust is our best bet to replace today's most famous crypto libraries: [OpenSSL](https://www.openssl.org), [BoringSSL](https://boringssl.googlesource.com/boringssl) and [libsodium](https://github.com/jedisct1/libsodium), which are all written in C.

It will take time for sure, but in 2019, `rustls` (a library we will see later) was [benchmarked](https://jbp.io/2019/07/01/rustls-vs-openssl-performance.html) to be 5% to 70% faster than `OpenSSL`, depending on the task. One of the most important thing (that is missing today) to see broad adoption? Certifications (such as [FIPS](https://csrc.nist.gov/publications/detail/fips/140/3/final)).

Without further ado, here is a survey of the Rust cryptography ecosystem in 2021.

### sodiumoxide

[sodiumoxide](https://github.com/sodiumoxide/sodiumoxide) is a Rust wrapper for [libsodium](https://github.com/jedisct1/libsodium), the renowned C cryptography library recommended by most applied cryptographers.

The drawback of this library is that as it's C bindings, it may introduce hard-to-debug bugs.

Also, please note that the original maintainer [announced in November 2020](https://github.com/sodiumoxide/sodiumoxide/issues/442) that he is stepping back from the project. That being said, at its current state, the project is fairly stable, and urgent issues (if any) will surely be fixed promptly.


## ring

*[ring](https://github.com/briansmith/ring) is focused on the implementation, testing, and optimization of a core set of cryptographic operations exposed via an easy-to-use (and hard-to-misuse) API. ring exposes a Rust API and is written in a hybrid of Rust, C, and assembly language.*

ring provides low-level primitives to use in your higher-level protocols and applications. The principal maintainer is known for being very serious about cryptography and the code to be high-quality.

The only problem is that some algorithms, such as `XChaCha20-Poly1305`, are missing.


### dalek cryptography

[dalek-cryptography](https://github.com/dalek-cryptography) is a GitHub organization regrouping multiple packages about pure-Rust elliptic curve cryptography such as [`x25519`](https://github.com/dalek-cryptography/x25519-dalek) and [`ed25519`](https://github.com/dalek-cryptography/ed25519-dalek).

The projects are used by organizations serious about cryptography, such as [Signal](https://github.com/signalapp/libsignal-client/blob/master/rust/protocol/Cargo.toml) and [Diem](https://github.com/diem/diem/blob/main/crypto/crypto/Cargo.toml).



### Rust Crypto

[Rust Crypto](https://github.com/RustCrypto) is a GitHub organization regrouping all the crypto primitives you will need, in pure Rust, most of the time by providing a base trait and implementing it for all the different algorithms (look at [aead](https://docs.rs/aead/) for example).

Unfortunately, not all the crates are audited by a professional third party.


### rustls

[rustls](https://github.com/ctz/rustls) is a modern TLS library written in Rust. It uses `ring` under the hood for cryptography. Its goal is to provide only safe to use features by allowing only TLS 1.2 and upper, for example.


In my opinion, this library is on the right track to replace `OpenSSL` and `BoringSSL`.


### Other crates

There are many other crates such as [`blake3`](https://crates.io/crates/blake3), but, in my opinion, they should be evaluated only if you can't find your primitive in the crates/organizations above.



## Summary

*As of June 2021*

| crate | audited | Total downloads |
| --- | --- |  --- |
| [ring](https://github.com/briansmith/ring) | [Yes ✅](https://github.com/ctz/rustls/blob/master/audit/TLS-01-report.pdf) | 10,339,221 |
| [rustls](https://github.com/rustls/rustls) | [Yes ✅](https://github.com/ctz/rustls/blob/main/audit/TLS-01-report.pdf) | 7,882,370 |
| [ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek) | ❌ No | 2,148,849 |
| [x25519-dalek](https://github.com/dalek-cryptography/x25519-dalek) | ❌ No | 1,554,105 |
| [aes-gcm](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm) | [Yes ✅](https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/) | 2,203,807 |
| [chacha20poly1305](https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305) | [Yes ✅](https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/) | 864,288 |
| [sodiumoxide](https://github.com/sodiumoxide/sodiumoxide) | ❌ No | 842,287 |

