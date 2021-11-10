## [Benchmarking symmetric encryption (AEAD) in Rust](https://kerkour.com/rust-symmetric-encryption-aead-benchmark/)


```shell
$ RUSTFLAGS="-Ctarget-cpu=sandybridge -Ctarget-feature=+aes,+sse2,+sse4.1,+ssse3" cargo bench
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
Benchmarking 100B/XChaCha20-Poly1305
Benchmarking 100B/XChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100B/XChaCha20-Poly1305: Collecting 100 samples in estimated 5.0039 s (2.7M iterations)
Benchmarking 100B/XChaCha20-Poly1305: Analyzing
100B/XChaCha20-Poly1305 time:   [1.8421 us 1.8483 us 1.8565 us]
                        thrpt:  [51.369 MiB/s 51.596 MiB/s 51.772 MiB/s]
                 change:
                        time:   [-0.5346% -0.0861% +0.3950%] (p = 0.72 > 0.05)
                        thrpt:  [-0.3935% +0.0862% +0.5375%]
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) high mild
  14 (14.00%) high severe
Benchmarking 100B/ChaCha20-Poly1305
Benchmarking 100B/ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100B/ChaCha20-Poly1305: Collecting 100 samples in estimated 5.0041 s (2.9M iterations)
Benchmarking 100B/ChaCha20-Poly1305: Analyzing
100B/ChaCha20-Poly1305  time:   [1.7244 us 1.7283 us 1.7334 us]
                        thrpt:  [55.018 MiB/s 55.180 MiB/s 55.305 MiB/s]
                 change:
                        time:   [-0.1776% +0.2105% +0.6016%] (p = 0.30 > 0.05)
                        thrpt:  [-0.5980% -0.2101% +0.1779%]
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  10 (10.00%) high severe
Benchmarking 100B/ring ChaCha20-Poly1305
Benchmarking 100B/ring ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100B/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.0009 s (26M iterations)
Benchmarking 100B/ring ChaCha20-Poly1305: Analyzing
100B/ring ChaCha20-Poly1305
                        time:   [191.70 ns 192.41 ns 193.26 ns]
                        thrpt:  [493.47 MiB/s 495.63 MiB/s 497.48 MiB/s]
                 change:
                        time:   [-0.3226% +0.1963% +0.7030%] (p = 0.47 > 0.05)
                        thrpt:  [-0.6981% -0.1959% +0.3236%]
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe
Benchmarking 100B/ring AES-256-GCM
Benchmarking 100B/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 100B/ring AES-256-GCM: Collecting 100 samples in estimated 5.0002 s (24M iterations)
Benchmarking 100B/ring AES-256-GCM: Analyzing
100B/ring AES-256-GCM   time:   [206.44 ns 207.26 ns 208.26 ns]
                        thrpt:  [457.92 MiB/s 460.14 MiB/s 461.97 MiB/s]
                 change:
                        time:   [-1.5486% -0.8743% -0.2836%] (p = 0.01 < 0.05)
                        thrpt:  [+0.2844% +0.8820% +1.5729%]
                        Change within noise threshold.
Found 20 outliers among 100 measurements (20.00%)
  2 (2.00%) low mild
  5 (5.00%) high mild
  13 (13.00%) high severe

Benchmarking 1kB/XChaCha20-Poly1305
Benchmarking 1kB/XChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 1kB/XChaCha20-Poly1305: Collecting 100 samples in estimated 5.0069 s (1.7M iterations)
Benchmarking 1kB/XChaCha20-Poly1305: Analyzing
1kB/XChaCha20-Poly1305  time:   [2.8889 us 2.9079 us 2.9291 us]
                        thrpt:  [325.59 MiB/s 327.95 MiB/s 330.12 MiB/s]
                 change:
                        time:   [-1.2378% -0.5573% +0.0920%] (p = 0.10 > 0.05)
                        thrpt:  [-0.0919% +0.5605% +1.2533%]
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  6 (6.00%) high mild
  11 (11.00%) high severe
Benchmarking 1kB/ChaCha20-Poly1305
Benchmarking 1kB/ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 1kB/ChaCha20-Poly1305: Collecting 100 samples in estimated 5.0119 s (1.8M iterations)
Benchmarking 1kB/ChaCha20-Poly1305: Analyzing
1kB/ChaCha20-Poly1305   time:   [2.7664 us 2.7763 us 2.7886 us]
                        thrpt:  [341.99 MiB/s 343.50 MiB/s 344.74 MiB/s]
                 change:
                        time:   [+0.0917% +0.9264% +1.7685%] (p = 0.03 < 0.05)
                        thrpt:  [-1.7378% -0.9179% -0.0916%]
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
Benchmarking 1kB/ring ChaCha20-Poly1305
Benchmarking 1kB/ring ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 1kB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.0015 s (7.1M iterations)
Benchmarking 1kB/ring ChaCha20-Poly1305: Analyzing
1kB/ring ChaCha20-Poly1305
                        time:   [695.93 ns 698.55 ns 701.51 ns]
                        thrpt:  [1.3276 GiB/s 1.3332 GiB/s 1.3383 GiB/s]
                 change:
                        time:   [-2.7160% -1.2735% -0.0062%] (p = 0.07 > 0.05)
                        thrpt:  [+0.0062% +1.2899% +2.7918%]
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe
Benchmarking 1kB/ring AES-256-GCM
Benchmarking 1kB/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 1kB/ring AES-256-GCM: Collecting 100 samples in estimated 5.0021 s (11M iterations)
Benchmarking 1kB/ring AES-256-GCM: Analyzing
1kB/ring AES-256-GCM    time:   [444.11 ns 445.73 ns 447.76 ns]
                        thrpt:  [2.0799 GiB/s 2.0894 GiB/s 2.0971 GiB/s]
                 change:
                        time:   [-2.1713% -1.1039% -0.1189%] (p = 0.04 < 0.05)
                        thrpt:  [+0.1191% +1.1162% +2.2195%]
                        Change within noise threshold.
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  13 (13.00%) high severe

Benchmarking 100kB/XChaCha20-Poly1305
Benchmarking 100kB/XChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100kB/XChaCha20-Poly1305: Collecting 100 samples in estimated 5.3180 s (45k iterations)
Benchmarking 100kB/XChaCha20-Poly1305: Analyzing
100kB/XChaCha20-Poly1305
                        time:   [116.53 us 117.53 us 118.63 us]
                        thrpt:  [803.90 MiB/s 811.44 MiB/s 818.42 MiB/s]
                 change:
                        time:   [-1.2407% -0.5759% +0.0658%] (p = 0.09 > 0.05)
                        thrpt:  [-0.0657% +0.5792% +1.2562%]
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe
Benchmarking 100kB/ChaCha20-Poly1305
Benchmarking 100kB/ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100kB/ChaCha20-Poly1305: Collecting 100 samples in estimated 5.3070 s (45k iterations)
Benchmarking 100kB/ChaCha20-Poly1305: Analyzing
100kB/ChaCha20-Poly1305 time:   [116.61 us 117.38 us 118.28 us]
                        thrpt:  [806.26 MiB/s 812.48 MiB/s 817.85 MiB/s]
                 change:
                        time:   [-1.0944% -0.2780% +0.5110%] (p = 0.51 > 0.05)
                        thrpt:  [-0.5084% +0.2788% +1.1065%]
                        No change in performance detected.
Found 19 outliers among 100 measurements (19.00%)
  2 (2.00%) high mild
  17 (17.00%) high severe
Benchmarking 100kB/ring ChaCha20-Poly1305
Benchmarking 100kB/ring ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 100kB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.1659 s (101k iterations)
Benchmarking 100kB/ring ChaCha20-Poly1305: Analyzing
100kB/ring ChaCha20-Poly1305
                        time:   [50.831 us 51.154 us 51.549 us]
                        thrpt:  [1.8067 GiB/s 1.8206 GiB/s 1.8322 GiB/s]
                 change:
                        time:   [-0.6142% +0.3971% +1.5845%] (p = 0.47 > 0.05)
                        thrpt:  [-1.5598% -0.3956% +0.6180%]
                        No change in performance detected.
Found 22 outliers among 100 measurements (22.00%)
  1 (1.00%) high mild
  21 (21.00%) high severe
Benchmarking 100kB/ring AES-256-GCM
Benchmarking 100kB/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 100kB/ring AES-256-GCM: Collecting 100 samples in estimated 5.0894 s (192k iterations)
Benchmarking 100kB/ring AES-256-GCM: Analyzing
100kB/ring AES-256-GCM  time:   [26.325 us 26.526 us 26.799 us]
                        thrpt:  [3.4751 GiB/s 3.5109 GiB/s 3.5377 GiB/s]
                 change:
                        time:   [+0.2081% +1.2669% +2.3929%] (p = 0.02 < 0.05)
                        thrpt:  [-2.3370% -1.2511% -0.2076%]
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) high mild
  12 (12.00%) high severe

Benchmarking 1MB/XChaCha20-Poly1305
Benchmarking 1MB/XChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.9s, enable flat sampling, or reduce sample count to 60.
Benchmarking 1MB/XChaCha20-Poly1305: Collecting 100 samples in estimated 5.8577 s (5050 iterations)
Benchmarking 1MB/XChaCha20-Poly1305: Analyzing
1MB/XChaCha20-Poly1305  time:   [1.1474 ms 1.1512 ms 1.1562 ms]
                        thrpt:  [824.85 MiB/s 828.40 MiB/s 831.15 MiB/s]
                 change:
                        time:   [-1.2606% -0.7031% -0.1454%] (p = 0.01 < 0.05)
                        thrpt:  [+0.1456% +0.7081% +1.2767%]
                        Change within noise threshold.
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe
Benchmarking 1MB/ChaCha20-Poly1305
Benchmarking 1MB/ChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.8s, enable flat sampling, or reduce sample count to 60.
Benchmarking 1MB/ChaCha20-Poly1305: Collecting 100 samples in estimated 5.8138 s (5050 iterations)
Benchmarking 1MB/ChaCha20-Poly1305: Analyzing
1MB/ChaCha20-Poly1305   time:   [1.1505 ms 1.1567 ms 1.1640 ms]
                        thrpt:  [819.34 MiB/s 824.51 MiB/s 828.89 MiB/s]
                 change:
                        time:   [-0.9414% -0.1659% +0.5813%] (p = 0.67 > 0.05)
                        thrpt:  [-0.5780% +0.1662% +0.9503%]
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  11 (11.00%) high severe
Benchmarking 1MB/ring ChaCha20-Poly1305
Benchmarking 1MB/ring ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 1MB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.2207 s (10k iterations)
Benchmarking 1MB/ring ChaCha20-Poly1305: Analyzing
1MB/ring ChaCha20-Poly1305
                        time:   [514.02 us 518.36 us 523.52 us]
                        thrpt:  [1.7790 GiB/s 1.7967 GiB/s 1.8118 GiB/s]
                 change:
                        time:   [-2.9105% -1.5845% -0.2600%] (p = 0.02 < 0.05)
                        thrpt:  [+0.2607% +1.6100% +2.9978%]
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  4 (4.00%) high mild
  13 (13.00%) high severe
Benchmarking 1MB/ring AES-256-GCM
Benchmarking 1MB/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 1MB/ring AES-256-GCM: Collecting 100 samples in estimated 5.3655 s (20k iterations)
Benchmarking 1MB/ring AES-256-GCM: Analyzing
1MB/ring AES-256-GCM    time:   [261.09 us 262.18 us 263.56 us]
                        thrpt:  [3.5336 GiB/s 3.5523 GiB/s 3.5671 GiB/s]
                 change:
                        time:   [-0.6719% +0.0359% +0.8171%] (p = 0.93 > 0.05)
                        thrpt:  [-0.8104% -0.0359% +0.6764%]
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  2 (2.00%) high mild
  14 (14.00%) high severe

Benchmarking 10MB/XChaCha20-Poly1305
Benchmarking 10MB/XChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 10MB/XChaCha20-Poly1305: Collecting 100 samples in estimated 5.8159 s (500 iterations)
Benchmarking 10MB/XChaCha20-Poly1305: Analyzing
10MB/XChaCha20-Poly1305 time:   [11.566 ms 11.619 ms 11.679 ms]
                        thrpt:  [816.59 MiB/s 820.80 MiB/s 824.52 MiB/s]
                 change:
                        time:   [-1.6113% -0.7894% +0.0110%] (p = 0.06 > 0.05)
                        thrpt:  [-0.0110% +0.7956% +1.6377%]
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe
Benchmarking 10MB/ChaCha20-Poly1305
Benchmarking 10MB/ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 10MB/ChaCha20-Poly1305: Collecting 100 samples in estimated 5.8066 s (500 iterations)
Benchmarking 10MB/ChaCha20-Poly1305: Analyzing
10MB/ChaCha20-Poly1305  time:   [11.557 ms 11.620 ms 11.692 ms]
                        thrpt:  [815.65 MiB/s 820.75 MiB/s 825.17 MiB/s]
                 change:
                        time:   [-0.1881% +0.4803% +1.2111%] (p = 0.18 > 0.05)
                        thrpt:  [-1.1966% -0.4780% +0.1885%]
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  2 (2.00%) high mild
  14 (14.00%) high severe
Benchmarking 10MB/ring ChaCha20-Poly1305
Benchmarking 10MB/ring ChaCha20-Poly1305: Warming up for 3.0000 s
Benchmarking 10MB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.2056 s (1000 iterations)
Benchmarking 10MB/ring ChaCha20-Poly1305: Analyzing
10MB/ring ChaCha20-Poly1305
                        time:   [5.1481 ms 5.1752 ms 5.2097 ms]
                        thrpt:  [1.7877 GiB/s 1.7996 GiB/s 1.8091 GiB/s]
                 change:
                        time:   [-0.9251% -0.1209% +0.7725%] (p = 0.77 > 0.05)
                        thrpt:  [-0.7666% +0.1211% +0.9337%]
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  9 (9.00%) high mild
  8 (8.00%) high severe
Benchmarking 10MB/ring AES-256-GCM
Benchmarking 10MB/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 10MB/ring AES-256-GCM: Collecting 100 samples in estimated 5.0495 s (1900 iterations)
Benchmarking 10MB/ring AES-256-GCM: Analyzing
10MB/ring AES-256-GCM   time:   [2.6504 ms 2.6674 ms 2.6865 ms]
                        thrpt:  [3.4667 GiB/s 3.4915 GiB/s 3.5139 GiB/s]
                 change:
                        time:   [-1.5804% -0.3475% +0.8413%] (p = 0.58 > 0.05)
                        thrpt:  [-0.8343% +0.3487% +1.6058%]
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  9 (9.00%) high mild
  9 (9.00%) high severe

Benchmarking 100MB/XChaCha20-Poly1305
Benchmarking 100MB/XChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 12.2s, or reduce sample count to 40.
Benchmarking 100MB/XChaCha20-Poly1305: Collecting 100 samples in estimated 12.192 s (100 iterations)
Benchmarking 100MB/XChaCha20-Poly1305: Analyzing
100MB/XChaCha20-Poly1305
                        time:   [119.50 ms 119.95 ms 120.51 ms]
                        thrpt:  [791.37 MiB/s 795.03 MiB/s 798.08 MiB/s]
                 change:
                        time:   [+0.4084% +0.8653% +1.3626%] (p = 0.00 < 0.05)
                        thrpt:  [-1.3443% -0.8578% -0.4068%]
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
Benchmarking 100MB/ChaCha20-Poly1305
Benchmarking 100MB/ChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 12.0s, or reduce sample count to 40.
Benchmarking 100MB/ChaCha20-Poly1305: Collecting 100 samples in estimated 11.953 s (100 iterations)
Benchmarking 100MB/ChaCha20-Poly1305: Analyzing
100MB/ChaCha20-Poly1305 time:   [119.39 ms 119.81 ms 120.32 ms]
                        thrpt:  [792.62 MiB/s 795.96 MiB/s 798.79 MiB/s]
                 change:
                        time:   [+0.0410% +0.5555% +1.1331%] (p = 0.03 < 0.05)
                        thrpt:  [-1.1204% -0.5524% -0.0410%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
Benchmarking 100MB/ring ChaCha20-Poly1305
Benchmarking 100MB/ring ChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.5s, or reduce sample count to 90.
Benchmarking 100MB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 5.5053 s (100 iterations)
Benchmarking 100MB/ring ChaCha20-Poly1305: Analyzing
100MB/ring ChaCha20-Poly1305
                        time:   [54.203 ms 54.502 ms 54.848 ms]
                        thrpt:  [1.6980 GiB/s 1.7088 GiB/s 1.7182 GiB/s]
                 change:
                        time:   [-0.6847% +0.2052% +1.0161%] (p = 0.65 > 0.05)
                        thrpt:  [-1.0059% -0.2047% +0.6894%]
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  11 (11.00%) high severe
Benchmarking 100MB/ring AES-256-GCM
Benchmarking 100MB/ring AES-256-GCM: Warming up for 3.0000 s
Benchmarking 100MB/ring AES-256-GCM: Collecting 100 samples in estimated 6.2560 s (200 iterations)
Benchmarking 100MB/ring AES-256-GCM: Analyzing
100MB/ring AES-256-GCM  time:   [31.018 ms 31.160 ms 31.336 ms]
                        thrpt:  [2.9720 GiB/s 2.9888 GiB/s 3.0025 GiB/s]
                 change:
                        time:   [+0.8020% +1.6374% +2.5668%] (p = 0.00 < 0.05)
                        thrpt:  [-2.5026% -1.6111% -0.7956%]
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

Benchmarking 1GB/XChaCha20-Poly1305
Benchmarking 1GB/XChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 145.2s, or reduce sample count to 10.
Benchmarking 1GB/XChaCha20-Poly1305: Collecting 100 samples in estimated 145.22 s (100 iterations)
Benchmarking 1GB/XChaCha20-Poly1305: Analyzing
1GB/XChaCha20-Poly1305  time:   [1.1885 s 1.1909 s 1.1936 s]
                        thrpt:  [799.00 MiB/s 800.80 MiB/s 802.43 MiB/s]
                 change:
                        time:   [+0.5415% +0.8471% +1.1586%] (p = 0.00 < 0.05)
                        thrpt:  [-1.1453% -0.8400% -0.5386%]
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Benchmarking 1GB/ChaCha20-Poly1305
Benchmarking 1GB/ChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 118.5s, or reduce sample count to 10.
Benchmarking 1GB/ChaCha20-Poly1305: Collecting 100 samples in estimated 118.50 s (100 iterations)
Benchmarking 1GB/ChaCha20-Poly1305: Analyzing
1GB/ChaCha20-Poly1305   time:   [1.1884 s 1.1934 s 1.2002 s]
                        thrpt:  [794.59 MiB/s 799.12 MiB/s 802.47 MiB/s]
                 change:
                        time:   [+0.9954% +1.4834% +2.0502%] (p = 0.00 < 0.05)
                        thrpt:  [-2.0090% -1.4617% -0.9855%]
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Benchmarking 1GB/ring ChaCha20-Poly1305
Benchmarking 1GB/ring ChaCha20-Poly1305: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 54.6s, or reduce sample count to 10.
Benchmarking 1GB/ring ChaCha20-Poly1305: Collecting 100 samples in estimated 54.644 s (100 iterations)
Benchmarking 1GB/ring ChaCha20-Poly1305: Analyzing
1GB/ring ChaCha20-Poly1305
                        time:   [540.84 ms 542.56 ms 544.50 ms]
                        thrpt:  [1.7104 GiB/s 1.7165 GiB/s 1.7220 GiB/s]
                 change:
                        time:   [-0.2300% +0.4147% +1.0202%] (p = 0.19 > 0.05)
                        thrpt:  [-1.0099% -0.4130% +0.2305%]
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
Benchmarking 1GB/ring AES-256-GCM
Benchmarking 1GB/ring AES-256-GCM: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 30.6s, or reduce sample count to 10.
Benchmarking 1GB/ring AES-256-GCM: Collecting 100 samples in estimated 30.619 s (100 iterations)
Benchmarking 1GB/ring AES-256-GCM: Analyzing
1GB/ring AES-256-GCM    time:   [304.02 ms 305.33 ms 306.88 ms]
                        thrpt:  [3.0348 GiB/s 3.0502 GiB/s 3.0634 GiB/s]
                 change:
                        time:   [-1.8828% -1.2489% -0.5686%] (p = 0.00 < 0.05)
                        thrpt:  [+0.5719% +1.2647% +1.9190%]
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
```
