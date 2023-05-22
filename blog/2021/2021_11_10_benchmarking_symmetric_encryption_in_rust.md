+++
date = 2021-11-10T06:00:00Z
title = "Benchmarking symmetric encryption (AEAD) in Rust"
type = "post"
tags = ["rust", "cryptography", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-symmetric-encryption-aead-benchmark"

[extra]
lang = "en"

comment ="""

"""
+++

[As we saw last week](/signatures-modern-end-to-end-encryption), when performing symmetric encryption you should use Authenticated Encryption with Additional Data (AEAD) which provides stronger guarantees than block ciphers.

The main contenders for authenticated encryption in Rust (and in general) are:
* [RustCrypto's `XChaCha20-Poly1305`](https://github.com/RustCrypto/stream-ciphers/tree/master/chacha20)
* [RustCrypto's `ChaCha20-Poly1305`](https://github.com/RustCrypto/stream-ciphers/tree/master/chacha20)
* [RustCrypto's `AES-256-GCM`](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm)
* [ring's `ChaCha20-Poly1305`](https://github.com/briansmith/ring)
* [ring's `AES-256-GCM`](https://github.com/briansmith/ring)


[As we saw a few months ago](https://kerkour.com/rust-cryptography-ecosystem/), all these crates have been audited ✅

## Results

**TL;DR**:
* [ring's `AES-256-GCM`](https://briansmith.org/rustdoc/ring/aead/index.html): **~3.3 GiB/s**
* [ring's `ChaCha20-Poly1305`](https://briansmith.org/rustdoc/ring/aead/index.html): **~1.7 GiB/s**
* [RustCrypto's `AES-256-GCM`](https://docs.rs/aes-gcm/latest/aes_gcm/): **~1 GiB/s**
* [RustCrypto's `XChaCha20-Poly1305`](https://github.com/RustCrypto/stream-ciphers/tree/master/chacha20): **~810 MiB/s**
* [RustCrypto's `ChaCha20-Poly1305`](https://docs.rs/chacha20/latest/chacha20/index.html): **~810 MiB/s**


|   | 100B | 1kB | 100kB | 1MB | 10MB | 100MB |
|---|---|---|---|---|---|---|
| RustCrypto's `XChaCha20-Poly1305` v0.8.2 | 928.91 ns (102.67 MiB/s) | 1.9851 us (480.41 MiB/s) | 116.50 us (818.58 MiB/s) | 1.1579 ms (823.59 MiB/s) | 11.571 ms (824.17 MiB/s) | 117.74 ms (809.99 MiB/s) |
| RustCrypto's `ChaCha20-Poly1305` v0.8.2 | 805.40 ns (118.41 MiB/s) | 1.8660 us (511.08 MiB/s) | 116.02 us (821.96 MiB/s) | 1.1522 ms (827.68 MiB/s) | 11.517 ms (828.02 MiB/s) | 117.87 ms (809.11 MiB/s) |
| RustCrypto's `AES-256-GCM` v0.9.4 | 154.27 ns (618.20 MiB/s) | 910.31 ns (1.0231 GiB/s) | 84.677 us (1.0999 GiB/s) | 844.85 us (1.1023 GiB/s) | 8.4719 ms (1.0993 GiB/s) | 88.666 ms (1.0504 GiB/s) |
| ring's `ChaCha20-Poly1305` v0.16.20 | 195.90 ns (486.81 MiB/s) | 701.99 ns (1.3267 GiB/s) | 51.594 us (1.8051 GiB/s) | 563.75 us (1.6520 GiB/s) | 5.1991 ms (1.7913 GiB/s) | 54.879 ms (1.6971 GiB/s) |
| ring's `AES-256-GCM` v0.16.20 | 214.48 ns (444.64 MiB/s) | 455.70 ns (2.0437 GiB/s) | 26.476 us (3.5177 GiB/s) | 264.13 us (3.5260 GiB/s) | 2.6474 ms (3.5179 GiB/s) | 30.450 ms (3.0585 GiB/s) |




## Some Closing Thoughts



It's known that `AES-256-GCM` is fast thanks to [AES-NI](https://en.wikipedia.org/wiki/AES_instruction_set), but I'm surprised by how fast is ring's `ChaCha20-Poly1305`. Congrats!

That being said, I will continue to use RustCrypto's `XChaCha20-Poly1305` because I find its API more friendly and I prefer the simplicity of the `ChaCha` ciphers family.

As usual, you can find the code and the raw results on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/benchmarking_symmetric_encryption_in_rust) (please don't forget to star the repo 🙏).


## Annex

```
$ openssl version
OpenSSL 1.1.1f  31 Mar 2020


$ openssl speed -evp aes-256-gcm
Doing aes-256-gcm for 3s on 16 size blocks: 78827663 aes-256-gcm's in 2.99s
Doing aes-256-gcm for 3s on 64 size blocks: 52638099 aes-256-gcm's in 3.00s
Doing aes-256-gcm for 3s on 256 size blocks: 24098743 aes-256-gcm's in 3.00s
Doing aes-256-gcm for 3s on 1024 size blocks: 9134253 aes-256-gcm's in 3.00s
Doing aes-256-gcm for 3s on 8192 size blocks: 1329488 aes-256-gcm's in 3.00s
Doing aes-256-gcm for 3s on 16384 size blocks: 681386 aes-256-gcm's in 2.99s
OpenSSL 1.1.1f  31 Mar 2020
built on: Mon Aug 23 17:02:39 2021 UTC
options:bn(64,64) rc4(16x,int) des(int) aes(partial) blowfish(ptr)
compiler: gcc -fPIC -pthread -m64 -Wa,--noexecstack -Wall -Wa,--noexecstack -g -O2 -fdebug-prefix-map=/build/openssl-JWge0V/openssl-1.1.1f=. -fstack-protector-strong -Wformat -Werror=format-security -DOPENSSL_TLS_SECURITY_LEVEL=2 -DOPENSSL_USE_NODELETE -DL_ENDIAN -DOPENSSL_PIC -DOPENSSL_CPUID_OBJ -DOPENSSL_IA32_SSE2 -DOPENSSL_BN_ASM_MONT -DOPENSSL_BN_ASM_MONT5 -DOPENSSL_BN_ASM_GF2m -DSHA1_ASM -DSHA256_ASM -DSHA512_ASM -DKECCAK1600_ASM -DRC4_ASM -DMD5_ASM -DAESNI_ASM -DVPAES_ASM -DGHASH_ASM -DECP_NISTZ256_ASM -DX25519_ASM -DPOLY1305_ASM -DNDEBUG -Wdate-time -D_FORTIFY_SOURCE=2
The 'numbers' are in 1000s of bytes per second processed.
type             16 bytes     64 bytes    256 bytes   1024 bytes   8192 bytes  16384 bytes
aes-256-gcm     421820.27k  1122946.11k  2056426.07k  3117825.02k  3630388.57k  3733721.81k


$ openssl speed -evp chacha20-poly1305
Doing chacha20-poly1305 for 3s on 16 size blocks: 49024156 chacha20-poly1305's in 2.98s
Doing chacha20-poly1305 for 3s on 64 size blocks: 25597814 chacha20-poly1305's in 2.99s
Doing chacha20-poly1305 for 3s on 256 size blocks: 14016296 chacha20-poly1305's in 2.99s
Doing chacha20-poly1305 for 3s on 1024 size blocks: 6579639 chacha20-poly1305's in 3.00s
Doing chacha20-poly1305 for 3s on 8192 size blocks: 883942 chacha20-poly1305's in 2.99s
Doing chacha20-poly1305 for 3s on 16384 size blocks: 452250 chacha20-poly1305's in 3.00s
OpenSSL 1.1.1f  31 Mar 2020
built on: Mon Aug 23 17:02:39 2021 UTC
options:bn(64,64) rc4(16x,int) des(int) aes(partial) blowfish(ptr)
compiler: gcc -fPIC -pthread -m64 -Wa,--noexecstack -Wall -Wa,--noexecstack -g -O2 -fdebug-prefix-map=/build/openssl-JWge0V/openssl-1.1.1f=. -fstack-protector-strong -Wformat -Werror=format-security -DOPENSSL_TLS_SECURITY_LEVEL=2 -DOPENSSL_USE_NODELETE -DL_ENDIAN -DOPENSSL_PIC -DOPENSSL_CPUID_OBJ -DOPENSSL_IA32_SSE2 -DOPENSSL_BN_ASM_MONT -DOPENSSL_BN_ASM_MONT5 -DOPENSSL_BN_ASM_GF2m -DSHA1_ASM -DSHA256_ASM -DSHA512_ASM -DKECCAK1600_ASM -DRC4_ASM -DMD5_ASM -DAESNI_ASM -DVPAES_ASM -DGHASH_ASM -DECP_NISTZ256_ASM -DX25519_ASM -DPOLY1305_ASM -DNDEBUG -Wdate-time -D_FORTIFY_SOURCE=2
The 'numbers' are in 1000s of bytes per second processed.
type             16 bytes     64 bytes    256 bytes   1024 bytes   8192 bytes  16384 bytes
chacha20-poly1305   263216.94k   547913.08k  1200057.45k  2245850.11k  2421823.70k  2469888.00k
```

```shell
$ cat /proc/cpuinfo
processor       : 0
vendor_id       : GenuineIntel
cpu family      : 6
model           : 85
model name      : Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz
stepping        : 7
microcode       : 0xffffffff
cpu MHz         : 2593.906
cache size      : 36608 KB
physical id     : 0
siblings        : 4
core id         : 0
cpu cores       : 2
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 21
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology cpuid pni pclmulqdq vmx ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch invpcid_single pti tpr_shadow vnmi ept vpid fsgsbase bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx avx512f avx512dq rdseed adx smap clflushopt avx512cd avx512bw avx512vl xsaveopt xsavec xsaves md_clear
bugs            : cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf mds swapgs taa itlb_multihit
bogomips        : 5187.81
clflush size    : 64
cache_alignment : 64
address sizes   : 46 bits physical, 48 bits virtual
power management:
```
