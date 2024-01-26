## [Benchmarking symmetric encryption (AEAD) in Rust](https://kerkour.com/rust-symmetric-encryption-aead-benchmark/)

<!--
```shell
$ RUSTFLAGS="-Ctarget-cpu=sandybridge -Ctarget-feature=+aes,+sse2,+sse4.1,+ssse3" cargo bench
```
-->

```shell
$ RUSTFLAGS="-Ctarget-cpu=native" cargo bench
```


### Raw results

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

processor       : 1
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
apicid          : 1
initial apicid  : 1
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

processor       : 2
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
core id         : 1
cpu cores       : 2
apicid          : 2
initial apicid  : 2
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

processor       : 3
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
core id         : 1
cpu cores       : 2
apicid          : 3
initial apicid  : 3
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


```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

Gnuplot not found, using plotters backend
Benchmarking 100B/RustCrypto XChaCha20-Poly1305
Benchmarking 100B/RustCrypto XChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100B/RustCrypto XChaCha20-Poly1305: Collecting 100 samples in estimated 5.0032 s (5.3M iterations)
Benchmarking 100B/RustCrypto XChaCha20-Poly1305: Analyzing
100B/RustCrypto XChaCha20-Poly1305
                        time:   [919.48 ns 928.91 ns 940.30 ns]
                        thrpt:  [101.42 MiB/s 102.67 MiB/s 103.72 MiB/s]
                 change:
                        time:   [-2.1461% -0.4035% +1.3717%] (p = 0.66 > 0.05)
                        thrpt:  [-1.3532% +0.4051% +2.1932%]
                        No change in performance detected.
Found 19 outliers among 100 measurements (19.00%)
  3 (3.00%) high mild
  16 (16.00%) high severe
Benchmarking 100B/RustCrypto ChaCha20-Poly1305
Benchmarking 100B/RustCrypto ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100B/RustCrypto ChaCha20-Poly1305: Collecting 100 samples in estimated 5.0022 s (6.2M iterations)
Benchmarking 100B/RustCrypto ChaCha20-Poly1305: Analyzing
100B/RustCrypto ChaCha20-Poly1305
                        time:   [800.09 ns 805.40 ns 811.88 ns]
                        thrpt:  [117.47 MiB/s 118.41 MiB/s 119.20 MiB/s]
                 change:
                        time:   [-2.0118% -0.4477% +1.0948%] (p = 0.58 > 0.05)
                        thrpt:  [-1.0830% +0.4497% +2.0531%]
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  10 (10.00%) high mild
  4 (4.00%) high severe
Benchmarking 100B/RustCrypto AES-256-GCM
Benchmarking 100B/RustCrypto AES-256-GCM: Warming up for 3.0000 s
Benchmarking 100B/RustCrypto AES-256-GCM: Collecting 100 samples in estimated 5.0004 s (32M iterations)
Benchmarking 100B/RustCrypto AES-256-GCM: Analyzing
100B/RustCrypto AES-256-GCM
                        time:   [153.16 ns 154.27 ns 155.49 ns]
                        thrpt:  [613.34 MiB/s 618.20 MiB/s 622.68 MiB/s]
                 change:
                        time:   [-0.6461% +0.4854% +1.6094%] (p = 0.40 > 0.05)
                        thrpt:  [-1.5839% -0.4831% +0.6503%]
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe
Benchmarking 100B/ring ChaCha20-Poly1305
Benchmarking 100B/ring ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100B/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.0002 s (25M iterations)
Benchmarking 100B/ring ChaCha20-Poly1305: Analyzing
100B/ring ChaCha20-Poly1305
                        time:   [194.85 ns 195.90 ns 197.14 ns]
                        thrpt:  [483.75 MiB/s 486.81 MiB/s 489.45 MiB/s]
                 change:
                        time:   [+0.1937% +1.1204% +2.0130%] (p = 0.01 < 0.05)
                        thrpt:  [-1.9733% -1.1080% -0.1933%]
                        Change within noise threshold.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe
Benchmarking 100B/ring AES-256-GCM
Benchmarking 100B/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 100B/ring AES-256-GCM: Collecting 100 samples in estimated 5.0009 s (23M iterations)
Benchmarking 100B/ring AES-256-GCM: Analyzing
100B/ring AES-256-GCM   time:   [213.23 ns 214.48 ns 215.90 ns]
                        thrpt:  [441.72 MiB/s 444.64 MiB/s 447.25 MiB/s]
                 change:
                        time:   [+0.8044% +1.7158% +2.6622%] (p = 0.00 < 0.05)
                        thrpt:  [-2.5932% -1.6869% -0.7980%]
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  6 (6.00%) high mild
  11 (11.00%) high severe

Benchmarking 1kB/RustCrypto XChaCha20-Poly1305
Benchmarking 1kB/RustCrypto XChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 1kB/RustCrypto XChaCha20-Poly1305: Collecting 100 samples in estimated 5.0019 s (2.5M iterations)
Benchmarking 1kB/RustCrypto XChaCha20-Poly1305: Analyzing
1kB/RustCrypto XChaCha20-Poly1305
                        time:   [1.9679 us 1.9851 us 2.0036 us]
                        thrpt:  [475.98 MiB/s 480.41 MiB/s 484.61 MiB/s]
                 change:
                        time:   [-6.7668% -5.2431% -3.7733%] (p = 0.00 < 0.05)
                        thrpt:  [+3.9212% +5.5332% +7.2580%]
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  8 (8.00%) high mild
  6 (6.00%) high severe
Benchmarking 1kB/RustCrypto ChaCha20-Poly1305
Benchmarking 1kB/RustCrypto ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 1kB/RustCrypto ChaCha20-Poly1305: Collecting 100 samples in estimated 5.0084 s (2.7M iterations)
Benchmarking 1kB/RustCrypto ChaCha20-Poly1305: Analyzing
1kB/RustCrypto ChaCha20-Poly1305
                        time:   [1.8522 us 1.8660 us 1.8813 us]
                        thrpt:  [506.92 MiB/s 511.08 MiB/s 514.90 MiB/s]
                 change:
                        time:   [-0.4790% +0.6605% +1.8862%] (p = 0.27 > 0.05)
                        thrpt:  [-1.8513% -0.6562% +0.4813%]
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  10 (10.00%) high mild
  6 (6.00%) high severe
Benchmarking 1kB/RustCrypto AES-256-GCM
Benchmarking 1kB/RustCrypto AES-256-GCM: Warming up for 3.0000 s
Benchmarking 1kB/RustCrypto AES-256-GCM: Collecting 100 samples in estimated 5.0039 s (5.5M iterations)
Benchmarking 1kB/RustCrypto AES-256-GCM: Analyzing
1kB/RustCrypto AES-256-GCM
                        time:   [906.88 ns 910.31 ns 914.58 ns]
                        thrpt:  [1.0183 GiB/s 1.0231 GiB/s 1.0269 GiB/s]
                 change:
                        time:   [-0.5238% +0.9414% +2.5276%] (p = 0.24 > 0.05)
                        thrpt:  [-2.4653% -0.9326% +0.5266%]
                        No change in performance detected.
Found 19 outliers among 100 measurements (19.00%)
  9 (9.00%) high mild
  10 (10.00%) high severe
Benchmarking 1kB/ring ChaCha20-Poly1305
Benchmarking 1kB/ring ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 1kB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.0033 s (7.1M iterations)
Benchmarking 1kB/ring ChaCha20-Poly1305: Analyzing
1kB/ring ChaCha20-Poly1305
                        time:   [698.02 ns 701.99 ns 706.46 ns]
                        thrpt:  [1.3183 GiB/s 1.3267 GiB/s 1.3342 GiB/s]
                 change:
                        time:   [-2.8653% -1.5452% -0.2813%] (p = 0.02 < 0.05)
                        thrpt:  [+0.2821% +1.5695% +2.9498%]
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe
Benchmarking 1kB/ring AES-256-GCM
Benchmarking 1kB/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 1kB/ring AES-256-GCM: Collecting 100 samples in estimated 5.0005 s (11M iterations)
Benchmarking 1kB/ring AES-256-GCM: Analyzing
1kB/ring AES-256-GCM    time:   [451.83 ns 455.70 ns 460.50 ns]
                        thrpt:  [2.0224 GiB/s 2.0437 GiB/s 2.0612 GiB/s]
                 change:
                        time:   [-0.5876% +0.4471% +1.4560%] (p = 0.39 > 0.05)
                        thrpt:  [-1.4351% -0.4451% +0.5910%]
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  6 (6.00%) high mild
  10 (10.00%) high severe

Benchmarking 100kB/RustCrypto XChaCha20-Poly1305
Benchmarking 100kB/RustCrypto XChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100kB/RustCrypto XChaCha20-Poly1305: Collecting 100 samples in estimated 5.2273 s (45k iterations)
Benchmarking 100kB/RustCrypto XChaCha20-Poly1305: Analyzing
100kB/RustCrypto XChaCha20-Poly1305
                        time:   [115.30 us 116.50 us 117.97 us]
                        thrpt:  [808.41 MiB/s 818.58 MiB/s 827.14 MiB/s]
                 change:
                        time:   [-1.0034% +0.2138% +1.3545%] (p = 0.73 > 0.05)
                        thrpt:  [-1.3364% -0.2133% +1.0136%]
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  5 (5.00%) high mild
  12 (12.00%) high severe
Benchmarking 100kB/RustCrypto ChaCha20-Poly1305
Benchmarking 100kB/RustCrypto ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100kB/RustCrypto ChaCha20-Poly1305: Collecting 100 samples in estimated 5.2475 s (45k iterations)
Benchmarking 100kB/RustCrypto ChaCha20-Poly1305: Analyzing
100kB/RustCrypto ChaCha20-Poly1305
                        time:   [115.19 us 116.02 us 117.00 us]
                        thrpt:  [815.09 MiB/s 821.96 MiB/s 827.89 MiB/s]
                 change:
                        time:   [-0.1826% +0.7606% +1.7994%] (p = 0.15 > 0.05)
                        thrpt:  [-1.7676% -0.7548% +0.1830%]
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  4 (4.00%) high mild
  14 (14.00%) high severe
Benchmarking 100kB/RustCrypto AES-256-GCM
Benchmarking 100kB/RustCrypto AES-256-GCM: Warming up for 3.0000 s
Benchmarking 100kB/RustCrypto AES-256-GCM: Collecting 100 samples in estimated 5.1204 s (61k iterations)
Benchmarking 100kB/RustCrypto AES-256-GCM: Analyzing
100kB/RustCrypto AES-256-GCM
                        time:   [84.119 us 84.677 us 85.307 us]
                        thrpt:  [1.0917 GiB/s 1.0999 GiB/s 1.1071 GiB/s]
                 change:
                        time:   [-0.1396% +0.7045% +1.5287%] (p = 0.11 > 0.05)
                        thrpt:  [-1.5057% -0.6996% +0.1398%]
                        No change in performance detected.
Found 19 outliers among 100 measurements (19.00%)
  7 (7.00%) high mild
  12 (12.00%) high severe
Benchmarking 100kB/ring ChaCha20-Poly1305
Benchmarking 100kB/ring ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100kB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.2627 s (101k iterations)
Benchmarking 100kB/ring ChaCha20-Poly1305: Analyzing
100kB/ring ChaCha20-Poly1305
                        time:   [51.008 us 51.594 us 52.282 us]
                        thrpt:  [1.7814 GiB/s 1.8051 GiB/s 1.8258 GiB/s]
                 change:
                        time:   [-1.6568% -0.4447% +0.7647%] (p = 0.46 > 0.05)
                        thrpt:  [-0.7589% +0.4467% +1.6847%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe
Benchmarking 100kB/ring AES-256-GCM
Benchmarking 100kB/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 100kB/ring AES-256-GCM: Collecting 100 samples in estimated 5.0993 s (192k iterations)
Benchmarking 100kB/ring AES-256-GCM: Analyzing
100kB/ring AES-256-GCM  time:   [26.292 us 26.476 us 26.698 us]
                        thrpt:  [3.4884 GiB/s 3.5177 GiB/s 3.5422 GiB/s]
                 change:
                        time:   [-2.3010% -1.0356% +0.2481%] (p = 0.13 > 0.05)
                        thrpt:  [-0.2475% +1.0464% +2.3552%]
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  5 (5.00%) high mild
  13 (13.00%) high severe

Benchmarking 1MB/RustCrypto XChaCha20-Poly1305
Benchmarking 1MB/RustCrypto XChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.0s, enable flat sampling, or reduce sample count to 60.
Benchmarking 1MB/RustCrypto XChaCha20-Poly1305: Collecting 100 samples in estimated 6.0088 s (5050 iterations)
Benchmarking 1MB/RustCrypto XChaCha20-Poly1305: Analyzing
1MB/RustCrypto XChaCha20-Poly1305
                        time:   [1.1487 ms 1.1579 ms 1.1686 ms]
                        thrpt:  [816.06 MiB/s 823.59 MiB/s 830.20 MiB/s]
                 change:
                        time:   [-1.6580% -0.5080% +0.5913%] (p = 0.39 > 0.05)
                        thrpt:  [-0.5878% +0.5106% +1.6860%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe
Benchmarking 1MB/RustCrypto ChaCha20-Poly1305
Benchmarking 1MB/RustCrypto ChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.8s, enable flat sampling, or reduce sample count to 60.
Benchmarking 1MB/RustCrypto ChaCha20-Poly1305: Collecting 100 samples in estimated 5.8407 s (5050 iterations)
Benchmarking 1MB/RustCrypto ChaCha20-Poly1305: Analyzing
1MB/RustCrypto ChaCha20-Poly1305
                        time:   [1.1433 ms 1.1522 ms 1.1628 ms]
                        thrpt:  [820.14 MiB/s 827.68 MiB/s 834.17 MiB/s]
                 change:
                        time:   [-1.0416% +0.0247% +1.1500%] (p = 0.96 > 0.05)
                        thrpt:  [-1.1369% -0.0247% +1.0526%]
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe
Benchmarking 1MB/RustCrypto AES-256-GCM
Benchmarking 1MB/RustCrypto AES-256-GCM: Warming up for 3.0000 s
Benchmarking 1MB/RustCrypto AES-256-GCM: Collecting 100 samples in estimated 8.5248 s (10k iterations)
Benchmarking 1MB/RustCrypto AES-256-GCM: Analyzing
1MB/RustCrypto AES-256-GCM
                        time:   [840.55 us 844.85 us 849.64 us]
                        thrpt:  [1.0961 GiB/s 1.1023 GiB/s 1.1080 GiB/s]
                 change:
                        time:   [-0.6631% +0.0052% +0.6468%] (p = 0.98 > 0.05)
                        thrpt:  [-0.6426% -0.0052% +0.6675%]
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  9 (9.00%) high mild
  8 (8.00%) high severe
Benchmarking 1MB/ring ChaCha20-Poly1305
Benchmarking 1MB/ring ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 1MB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.2462 s (10k iterations)
Benchmarking 1MB/ring ChaCha20-Poly1305: Analyzing
1MB/ring ChaCha20-Poly1305
                        time:   [555.38 us 563.75 us 573.10 us]
                        thrpt:  [1.6251 GiB/s 1.6520 GiB/s 1.6769 GiB/s]
                 change:
                        time:   [+6.6590% +8.4743% +10.397%] (p = 0.00 < 0.05)
                        thrpt:  [-9.4181% -7.8123% -6.2433%]
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Benchmarking 1MB/ring AES-256-GCM
Benchmarking 1MB/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 1MB/ring AES-256-GCM: Collecting 100 samples in estimated 5.4131 s (20k iterations)
Benchmarking 1MB/ring AES-256-GCM: Analyzing
1MB/ring AES-256-GCM    time:   [262.25 us 264.13 us 266.34 us]
                        thrpt:  [3.4967 GiB/s 3.5260 GiB/s 3.5513 GiB/s]
                 change:
                        time:   [-1.6909% -0.5226% +0.6041%] (p = 0.37 > 0.05)
                        thrpt:  [-0.6005% +0.5254% +1.7200%]
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe

Benchmarking 10MB/RustCrypto XChaCha20-Poly1305
Benchmarking 10MB/RustCrypto XChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 10MB/RustCrypto XChaCha20-Poly1305: Collecting 100 samples in estimated 5.9726 s (500 iterations)
Benchmarking 10MB/RustCrypto XChaCha20-Poly1305: Analyzing
10MB/RustCrypto XChaCha20-Poly1305
                        time:   [11.477 ms 11.571 ms 11.681 ms]
                        thrpt:  [816.44 MiB/s 824.17 MiB/s 830.95 MiB/s]
                 change:
                        time:   [-0.1135% +0.8971% +1.9056%] (p = 0.09 > 0.05)
                        thrpt:  [-1.8700% -0.8891% +0.1137%]
                        No change in performance detected.
Found 21 outliers among 100 measurements (21.00%)
  4 (4.00%) high mild
  17 (17.00%) high severe
Benchmarking 10MB/RustCrypto ChaCha20-Poly1305
Benchmarking 10MB/RustCrypto ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 10MB/RustCrypto ChaCha20-Poly1305: Collecting 100 samples in estimated 5.7298 s (500 iterations)
Benchmarking 10MB/RustCrypto ChaCha20-Poly1305: Analyzing
10MB/RustCrypto ChaCha20-Poly1305
                        time:   [11.456 ms 11.517 ms 11.587 ms]
                        thrpt:  [823.09 MiB/s 828.02 MiB/s 832.45 MiB/s]
                 change:
                        time:   [-5.3790% -4.1960% -3.0322%] (p = 0.00 < 0.05)
                        thrpt:  [+3.1270% +4.3798% +5.6847%]
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) high mild
  9 (9.00%) high severe
Benchmarking 10MB/RustCrypto AES-256-GCM
Benchmarking 10MB/RustCrypto AES-256-GCM: Warming up for 3.0000 s
Benchmarking 10MB/RustCrypto AES-256-GCM: Collecting 100 samples in estimated 5.0504 s (600 iterations)
Benchmarking 10MB/RustCrypto AES-256-GCM: Analyzing
10MB/RustCrypto AES-256-GCM
                        time:   [8.4296 ms 8.4719 ms 8.5205 ms]
                        thrpt:  [1.0930 GiB/s 1.0993 GiB/s 1.1048 GiB/s]
                 change:
                        time:   [-0.5064% +0.2125% +0.9550%] (p = 0.58 > 0.05)
                        thrpt:  [-0.9460% -0.2121% +0.5089%]
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  5 (5.00%) high mild
  13 (13.00%) high severe
Benchmarking 10MB/ring ChaCha20-Poly1305
Benchmarking 10MB/ring ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 10MB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.2122 s (1000 iterations)
Benchmarking 10MB/ring ChaCha20-Poly1305: Analyzing
10MB/ring ChaCha20-Poly1305
                        time:   [5.1531 ms 5.1991 ms 5.2542 ms]
                        thrpt:  [1.7725 GiB/s 1.7913 GiB/s 1.8073 GiB/s]
                 change:
                        time:   [-2.3787% -0.8554% +0.6841%] (p = 0.29 > 0.05)
                        thrpt:  [-0.6794% +0.8627% +2.4367%]
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) high mild
  14 (14.00%) high severe
Benchmarking 10MB/ring AES-256-GCM
Benchmarking 10MB/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 10MB/ring AES-256-GCM: Collecting 100 samples in estimated 5.0768 s (1900 iterations)
Benchmarking 10MB/ring AES-256-GCM: Analyzing
10MB/ring AES-256-GCM   time:   [2.6289 ms 2.6474 ms 2.6689 ms]
                        thrpt:  [3.4896 GiB/s 3.5179 GiB/s 3.5426 GiB/s]
                 change:
                        time:   [-0.6460% +0.3020% +1.3031%] (p = 0.56 > 0.05)
                        thrpt:  [-1.2863% -0.3011% +0.6502%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  12 (12.00%) high severe

Benchmarking 100MB/RustCrypto XChaCha20-Poly1305
Benchmarking 100MB/RustCrypto XChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 12.1s, or reduce sample count to 40.
Benchmarking 100MB/RustCrypto XChaCha20-Poly1305: Collecting 100 samples in estimated 12.136 s (100 iterations)
Benchmarking 100MB/RustCrypto XChaCha20-Poly1305: Analyzing
100MB/RustCrypto XChaCha20-Poly1305
                        time:   [117.24 ms 117.74 ms 118.29 ms]
                        thrpt:  [806.19 MiB/s 809.99 MiB/s 813.43 MiB/s]
                 change:
                        time:   [-1.7163% -1.0575% -0.3508%] (p = 0.00 < 0.05)
                        thrpt:  [+0.3520% +1.0688% +1.7462%]
                        Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) high mild
  9 (9.00%) high severe
Benchmarking 100MB/RustCrypto ChaCha20-Poly1305
Benchmarking 100MB/RustCrypto ChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 11.8s, or reduce sample count to 40.
Benchmarking 100MB/RustCrypto ChaCha20-Poly1305: Collecting 100 samples in estimated 11.796 s (100 iterations)
Benchmarking 100MB/RustCrypto ChaCha20-Poly1305: Analyzing
100MB/RustCrypto ChaCha20-Poly1305
                        time:   [117.34 ms 117.87 ms 118.47 ms]
                        thrpt:  [804.97 MiB/s 809.11 MiB/s 812.74 MiB/s]
                 change:
                        time:   [-2.0678% -1.2689% -0.5357%] (p = 0.00 < 0.05)
                        thrpt:  [+0.5386% +1.2852% +2.1115%]
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe
Benchmarking 100MB/RustCrypto AES-256-GCM
Benchmarking 100MB/RustCrypto AES-256-GCM: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.9s, or reduce sample count to 50.
Benchmarking 100MB/RustCrypto AES-256-GCM: Collecting 100 samples in estimated 8.8835 s (100 iterations)
Benchmarking 100MB/RustCrypto AES-256-GCM: Analyzing
100MB/RustCrypto AES-256-GCM
                        time:   [88.373 ms 88.666 ms 88.983 ms]
                        thrpt:  [1.0466 GiB/s 1.0504 GiB/s 1.0539 GiB/s]
                 change:
                        time:   [-1.7634% -0.8595% -0.0588%] (p = 0.05 < 0.05)
                        thrpt:  [+0.0588% +0.8670% +1.7951%]
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
Benchmarking 100MB/ring ChaCha20-Poly1305
Benchmarking 100MB/ring ChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.7s, or reduce sample count to 80.
Benchmarking 100MB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.6920 s (100 iterations)
Benchmarking 100MB/ring ChaCha20-Poly1305: Analyzing
100MB/ring ChaCha20-Poly1305
                        time:   [54.367 ms 54.879 ms 55.454 ms]
                        thrpt:  [1.6795 GiB/s 1.6971 GiB/s 1.7130 GiB/s]
                 change:
                        time:   [-0.5738% +0.8364% +2.2074%] (p = 0.23 > 0.05)
                        thrpt:  [-2.1597% -0.8295% +0.5771%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe
Benchmarking 100MB/ring AES-256-GCM
Benchmarking 100MB/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 100MB/ring AES-256-GCM: Collecting 100 samples in estimated 6.0984 s (200 iterations)
Benchmarking 100MB/ring AES-256-GCM: Analyzing
100MB/ring AES-256-GCM  time:   [30.194 ms 30.450 ms 30.756 ms]
                        thrpt:  [3.0281 GiB/s 3.0585 GiB/s 3.0845 GiB/s]
                 change:
                        time:   [-8.3415% -6.9625% -5.4539%] (p = 0.00 < 0.05)
                        thrpt:  [+5.7685% +7.4835% +9.1007%]
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) high mild
  13 (13.00%) high severe

Benchmarking 1GB/RustCrypto XChaCha20-Poly1305
Benchmarking 1GB/RustCrypto XChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 145.2s, or reduce sample count to 10.
Benchmarking 1GB/RustCrypto XChaCha20-Poly1305: Collecting 100 samples in estimated 145.23 s (100 iterations)
Benchmarking 1GB/RustCrypto XChaCha20-Poly1305: Analyzing
1GB/RustCrypto XChaCha20-Poly1305
                        time:   [1.1888 s 1.1940 s 1.1999 s]
                        thrpt:  [794.79 MiB/s 798.70 MiB/s 802.24 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
Benchmarking 1GB/RustCrypto ChaCha20-Poly1305
Benchmarking 1GB/RustCrypto ChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 118.2s, or reduce sample count to 10.
Benchmarking 1GB/RustCrypto ChaCha20-Poly1305: Collecting 100 samples in estimated 118.18 s (100 iterations)
Benchmarking 1GB/RustCrypto ChaCha20-Poly1305: Analyzing
1GB/RustCrypto ChaCha20-Poly1305
                        time:   [1.1827 s 1.1857 s 1.1892 s]
                        thrpt:  [801.93 MiB/s 804.29 MiB/s 806.37 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe
Benchmarking 1GB/RustCrypto AES-256-GCM
Benchmarking 1GB/RustCrypto AES-256-GCM: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 88.8s, or reduce sample count to 10.
Benchmarking 1GB/RustCrypto AES-256-GCM: Collecting 100 samples in estimated 88.848 s (100 iterations)
Benchmarking 1GB/RustCrypto AES-256-GCM: Analyzing
1GB/RustCrypto AES-256-GCM
                        time:   [889.63 ms 893.43 ms 897.99 ms]
                        thrpt:  [1.0371 GiB/s 1.0424 GiB/s 1.0469 GiB/s]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
Benchmarking 1GB/ring ChaCha20-Poly1305
Benchmarking 1GB/ring ChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 54.9s, or reduce sample count to 10.
Benchmarking 1GB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 54.928 s (100 iterations)
Benchmarking 1GB/ring ChaCha20-Poly1305: Analyzing
1GB/ring ChaCha20-Poly1305
                        time:   [544.21 ms 546.53 ms 549.07 ms]
                        thrpt:  [1.6962 GiB/s 1.7041 GiB/s 1.7113 GiB/s]
                 change:
                        time:   [+0.1700% +0.7324% +1.2751%] (p = 0.01 < 0.05)
                        thrpt:  [-1.2591% -0.7271% -0.1697%]
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
Benchmarking 1GB/ring AES-256-GCM
Benchmarking 1GB/ring AES-256-GCM: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 30.3s, or reduce sample count to 10.
Benchmarking 1GB/ring AES-256-GCM: Collecting 100 samples in estimated 30.310 s (100 iterations)
Benchmarking 1GB/ring AES-256-GCM: Analyzing
1GB/ring AES-256-GCM    time:   [306.16 ms 308.11 ms 310.29 ms]
                        thrpt:  [3.0014 GiB/s 3.0227 GiB/s 3.0420 GiB/s]
                 change:
                        time:   [+0.1243% +0.9111% +1.8104%] (p = 0.03 < 0.05)
                        thrpt:  [-1.7782% -0.9028% -0.1241%]
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
```
