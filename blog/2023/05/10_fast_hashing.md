+++
date = 2023-05-10T07:00:00Z
title = "Fast hashing, it's not that simple"
type = "post"
tags = ["cryptography", "go", "programming", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/fast-hashing-algorithms"

[extra]
lang = "en"
+++

<!--
https://crypto.stackexchange.com/questions/59375/are-hash-functions-strong-against-quantum-cryptanalysis-and-or-independent-enoug
-->

Whether you are developing a new application or defining a new protocol, you may have a hard time deciding which hash function to use. Which one is safe? Which one is fast?

Let's find out!


## Why do we need fast hashing?

*But Sylvain, I thought that hashing should be slow in order to stop hackers from cracking passwords in the event our database gets breached.*

Not these kinds of hash functions. They are actually called Password-Based Key Derivation Functions (PBKDF) by cryptographers, such as Argon2id or Scrypt. Their purpose is to turn a password (which is cryptographically insecure) into a secure hash (or key). The slower they are, the more resistant they are to cracking.

Today, we will study cryptographic hash functions designed to ensure the integrity of data. These functions should be **collision-resistant**: it must be virtually impossible to find two messages `m1` and `m2` where `hash(m1) == hash(m2)`. They should also be **preimage-resistant**: it must be virtually impossible to find the original message `m` when given `hash(m)`.

There are also fast non-cryptographic hash algorithms such as [xxHash](https://cyan4973.github.io/xxHash/) capable of reaching a throughput of 31.5 GB/s per core, sometimes faster than `memcpy`, but due to the *high* risk of collision, the possibility of finding a message `m2` such that `hash(m2) == hash(m1)`, they should be used in a few, very precise cases and will not be studied here.

Let's say that we have 150 GB of data to hash (a backup of all your pictures and videos) and you want to check its integrity. Here is how much time it would take depending on the hashing speed.

| Throughput / core | 1 core | 4 cores | 8 cores |
| --- | --- | --- | --- |
| 300 MB/s | 500s (8m20s) | 125s (2m05s) | 63s (1m03s) |
| 800 MB/s | 188s (3m08s) | 47s | 24s |
| 1700 MB/s  | 88s (1m28s) | 22s | 11s |
| 4000 MB/s | 38s | 10s | 5s |

Thus, the question is: how much time do you (or your users) want to lose waiting for your application to complete its computation?

There are a lot of debates online about which algorithm is faster due to its clever design, but what about real-life?

## The numbers

I've benchmarked the most famous hashing algorithms available today, on both `amd64` and `arm64` processors, and the results are... interesting.

Without further ado, let's see the raw numbers. Here I tested the fastest Go implementations, but you should be able to find similar results for other ecosystems.

**amd64** (AMD EPYC 7R13 - AWS c6a.4xlarge):

```
CPU:
- arch: amd64
- physical cores: 8
- logical cores: 16

CPU features:
- AVX2: true
- AVX: true
- SSE: true
- SSE2: true
- AES: true
- SHA1: false
- SHA2: false
- SHA512: false
```

| Algorithm | 64 B | 64 KB | 10 MB |
| --- | --- | --- | --- |
| SHA2-256 | 617.84 MB/s | 1776.48 MB/s | 1776.27 MB/s |
| SHA2-512 | 295.78 MB/s | 760.26 MB/s | 760.94 MB/s |
| SHA3-256 | 73.30 MB/s | 375.60 MB/s | 377.05 MB/s |
| Blake2b-256 | 387.09 MB/s | 909.01 MB/s | 908.23 MB/s |
| Blake3-256 | **812.69 MB/s** | **3903.07 MB/s** | **4010.50 MB/s** |


**arm64** (Graviton 3 - AWS c7g.4xlarge):

```
CPU:
- arch: arm64
- physical cores: 16
- logical cores: 16

CPU features:
- AES: true
- SHA1: true
- SHA2: true
- SHA512: true
```

| Algorithm | 64 B | 64 KB | 10 MB |
| --- | --- | --- | --- |
| SHA2-256 | **478.46 MB/s** | **1625.15 MB/s** | **1628.37 MB/s** |
| SHA2-512 | 133.36 MB/s | 299.19 MB/s | 299.85 MB/s |
| SHA3-256 | 80.34 MB/s | 368.37 MB/s | 369.60 MB/s |
| Blake2b-256 | 267.29 MB/s | 594.58 MB/s | 595.25 MB/s |
| Blake3-256 | 400.77 MB/s | 462.32 MB/s | 464.36 MB/s |


> **Want to learn more about applied cryptography such as how to implement end-to-end encryption in practice? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build our own end-to-end encrypted protocol to secure the communication of a Remote Access Tool in  Rust.**


Wow! Blake3 is incredibly fast... on `amd64`. Today we do not have an efficient implementation for `arm64` for Go. I've benchmarked the official implementation in Rust on `arm64` which uses `NEON` instructions and it has a throughput of roughly 900MB/s per core, less than `SHA-256`.

To reach these speeds, the `Blake3` implementation uses intel's `AVX-512` instructions while the `SHA-256` implementation use various intel's [SIMD](https://kerkour.com/open-source-weekly/7) instructions on `amd64` and `SHA2` instructions on `arm64`.

But, `AVX-512` instructions are known to [decrease in performance when the number of threads increases on at least some inter processors due to frequency scaling](https://blog.cloudflare.com/on-the-dangers-of-intels-frequency-scaling/). `SHA-256` also has an [AVX-512 backed implementation reaching up to 3498.20 MB/s](https://github.com/minio/sha256-simd#support-for-avx512) capable of reaching 3 GB/s.

Also, even if I trust the authors and the design looks sound, Blake3 is still a young algorithm which hasn't received a lot of analysis so far, compared to the others.

So, for me the answer is clear, there is no reasons **not to** choose `SHA-256` for hashing.

1.7 GB/s per core is not going to be your bottleneck, I/O (disk, network) is.

For example, the 16-inch MacBook Pro with M2 Pro in a 2TB storage configuration achieves read/write speed of roughly 5-6 GB/s which means that we can saturate it with only 3 of the processor's 12 cores. 1.7 GB/s is also enough to saturate a 10 Gb/s network link (1.7 GB/s * 8 = 13.6 Gb/s).

## Some Closing Thoughts

While not making headlines, `SHA-256` was, is, and will continue to be a safe bet for any new application or protocol in the foreseeable future, as you will be able to easily find an efficient implementation using hardware acceleration for your programming language of choice. Being ubiquitous (Bitcoin, most software update mechanisms, antiviruses...), you will know for sure if its security is compromised, which is less likely for lesser known algorithms.

Furthermore, with the boom of `arm64` processors which have dedicated `SHA2` instructions, it's actually the fastest algorithm when we consider today's fractured computing landscape.

And this is before talking about [NIST approved algorithms](https://csrc.nist.gov/projects/hash-functions).

Choose boring, choose safe, choose fast. Choose `SHA-256`.

As always, you can find the code on GitHub: [github.com/skerkour/go-benchmarks](https://github.com/skerkour/go-benchmarks) (please don't forget to star the repo 🙏).

<!-- **Want to learn more about applied cryptography such as how to implement end-to-end encryption in practice? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build our own end-to-end encrypted protocol to secure the communication of a Remote Access Tool in  Rust.** -->
