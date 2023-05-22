+++
date = 2021-08-04T15:45:00Z
title = "15k inserts/s with Rust and SQLite"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/high-performance-rust-with-sqlite"

[extra]
lang = "en"

comment ="""
"""
+++

There is this growing sentiment in tech that stacking more and more layers of complexity to reach the sacrosanct "infinite scalability" is not the way forward.

![Debt.](https://kerkour.com/2021/high-performance-rust-with-sqlite/debt.gif)

First, because it rarely materializes: you need a lot of other things than fancy tech to reach millions of people.

Second, because the tradeoffs of complex systems are often misunderstood, and most of the time, they bring more problems than benefits. And these problems compound over time.

So here is a little experiment to show you how to reach 15,000 inserts per second with simple technology, which is approximately 1.3 billion inserts per day. **1.3 Billion**.

Is it possible to improve this micro benchmark? Of course, by bundling all the inserts in a single transaction, for example, or by using another, non-async database driver, but it does not make sense as it's not how a real-world codebase accessing a database looks like. We favor **simplicity** over theorical numbers.


Without further ado, here are the results:
```
$ cargo run --release -- -c 3 -i 100000
Inserting 100000 records. concurrency: 3
Time elapsed to insert 100000 records: 6.523381395s (15329.47 inserts/s)
```


## The code

**Cargo.toml**
```toml
[package]
name = "high_performance_rust_with_sqlite"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.5", features = [ "runtime-tokio-rustls", "sqlite", "uuid", "chrono", "migrate" ] }
futures = "0.3"
chrono = "0.4"
uuid = { version = "0.8", features = ["v4"] }
clap = "2"
```

**main.rs**
```rust
use clap::{App, Arg};
use futures::*;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Pool, Sqlite,
};
use std::time::{Duration, Instant};
use std::{fs, str::FromStr};

struct User {
    id: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    username: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_matches = App::new("Rust to the mooooon")
        .version("1.0")
        .arg(
            Arg::with_name("concurrency")
                .short("c")
                .long("concurrency")
                .help("Number of concurrent inserts")
                .default_value("3"),
        )
        .arg(
            Arg::with_name("inserts")
                .short("i")
                .long("inserts")
                .help("Number of inserts to perform")
                .default_value("40000"),
        )
        .get_matches();

    let concurrency = cli_matches
        .value_of("concurrency")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1);
    let inserts = cli_matches
        .value_of("inserts")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1);

    let database_file = "db.sqlite";
    let database_url = format!("sqlite://{}", database_file);
    let pool_timeout = Duration::from_secs(30);
    // with pool_max_connections = 1, the pool timeout. maybe related to https://github.com/launchbadge/sqlx/issues/1210
    let pool_max_connections = if concurrency == 1 {
        2
    } else {
        concurrency as u32
    };

    let _ = fs::remove_file(database_file);

    let connection_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(pool_timeout);

    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(pool_max_connections)
        .connect_timeout(pool_timeout)
        .connect_with(connection_options)
        .await?;

    sqlx::migrate!("./db").run(&sqlite_pool).await?;

    sqlx::query("pragma temp_store = memory;")
        .execute(&sqlite_pool)
        .await?;
    sqlx::query("pragma mmap_size = 30000000000;")
        .execute(&sqlite_pool)
        .await?;
    sqlx::query("pragma page_size = 4096;")
        .execute(&sqlite_pool)
        .await?;

    println!(
        "Inserting {} records. concurrency: {}",
        inserts, concurrency
    );

    let start = Instant::now();
    insert(inserts, concurrency, &sqlite_pool).await;
    let duration = start.elapsed();

    let inserts_per_sec = inserts as f64 / duration.as_secs_f64();
    println!(
        "Time elapsed to insert {} records: {:?} ({:.2} inserts/s)",
        inserts, duration, inserts_per_sec
    );

    Ok(())
}

async fn insert(inserts: usize, concurrency: usize, sqlite_pool: &Pool<Sqlite>) {
    let stream = stream::iter(0..inserts);

    stream
        .for_each_concurrent(concurrency, |_| async move {
            let user = User {
                id: uuid::Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                username: String::from("Hello"),
            };

            sqlx::query(
                "INSERT INTO users (id, created_at, username)
            VALUES (?, ?, ?)",
            )
            .bind(user.id)
            .bind(user.created_at)
            .bind(&user.username)
            .execute(sqlite_pool)
            .await
            .expect("inserting in db");
        })
        .await;
}
```



**db/000_init.sql**
```sql
CREATE TABLE IF NOT EXISTS users (
    id BLOB PRIMARY KEY NOT NULL,
    created_at TEXT NOT NULL,
    username TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_users_on_id ON users(id);
```

## Concurrency

Increasing concurrency should increase performance, right?

```
$ cargo run --release -- -c 100 -i 100000
Inserting 100000 records. concurrency: 100
Time elapsed to insert 100000 records: 10.255768373s (9750.61 inserts/s)
```

What happens? [SQLite allows only one concurrent write](https://www.sqlite.org/lockingv3.html) to a database. Thus, if we increase concurrency too much, we encounter lock contention, and performance is degraded.

One way to limit lock contention is to use concurrency primitives in your own code and a good scheduler. In this example, we use a [Stream](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.for_each_concurrent) with [tokio](https://tokio.rs/) which seem way better at handling concurrency than SQLite's locking mechanism.


## The machine

For the record, the server is a [Scaleway ENT1-S](https://www.scaleway.com/en/pricing/#enterprise-instances), so not the slowest VPS of the market, but not that expensive either.

```shell
$ sudo lscpu
```

```
Architecture:                    x86_64
CPU op-mode(s):                  32-bit, 64-bit
Byte Order:                      Little Endian
Address sizes:                   40 bits physical, 48 bits virtual
CPU(s):                          8
On-line CPU(s) list:             0-7
Thread(s) per core:              1
Core(s) per socket:              8
Socket(s):                       1
NUMA node(s):                    1
Vendor ID:                       AuthenticAMD
CPU family:                      25
Model:                           1
Model name:                      AMD EPYC 7543 32-Core Processor
Stepping:                        1
CPU MHz:                         2794.750
BogoMIPS:                        5589.50
Virtualization:                  AMD-V
Hypervisor vendor:               KVM
Virtualization type:             full
L1d cache:                       512 KiB
L1i cache:                       512 KiB
L2 cache:                        4 MiB
L3 cache:                        16 MiB
NUMA node0 CPU(s):               0-7
Vulnerability Itlb multihit:     Not affected
Vulnerability L1tf:              Not affected
Vulnerability Mds:               Not affected
Vulnerability Meltdown:          Not affected
Vulnerability Spec store bypass: Mitigation; Speculative Store Bypass disabled via prctl and seccomp
Vulnerability Spectre v1:        Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:        Mitigation; Full AMD retpoline, IBPB conditional, STIBP disabled, RSB filling
Vulnerability Srbds:             Not affected
Vulnerability Tsx async abort:   Not affected
Flags:                           fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmx
                                 ext fxsr_opt pdpe1gb rdtscp lm rep_good nopl cpuid extd_apicid tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse
                                 4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_l
                                 egacy abm sse4a misalignsse 3dnowprefetch osvw perfctr_core ssbd ibpb stibp vmmcall fsgsbase tsc_adjust bmi1 avx2 s
                                 mep bmi2 rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 wbnoinvd arat npt nrip_save umip vaes vpclm
                                 ulqdq arch_capabilities
```

```shell
$ sudo lshw -class disk -class storage
```

```
  *-scsi
       description: SCSI storage controller
       product: Virtio SCSI
       vendor: Red Hat, Inc.
       physical id: 1
       bus info: pci@0000:00:01.0
       version: 01
       width: 64 bits
       clock: 33MHz
       capabilities: scsi msix bus_master cap_list
       configuration: driver=virtio-pci latency=0
       resources: iomemory:180-17f irq:21 memory:9100a000-9100afff memory:1800000000-1800003fff
  *-sata
       description: SATA controller
       product: 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode]
       vendor: Intel Corporation
       physical id: 1f.2
       bus info: pci@0000:00:1f.2
       version: 02
       width: 32 bits
       clock: 33MHz
       capabilities: sata msi ahci_1.0 bus_master cap_list
       configuration: driver=ahci latency=0
       resources: irq:36 ioport:1040(size=32) memory:91000000-91000fff
  *-scsi
       physical id: 5
       logical name: scsi0
     *-disk
          description: SCSI Disk
          product: b_ssd
          vendor: SCW
          physical id: 0.0.0
          bus info: scsi@0:0.0.0
          logical name: /dev/sda
          version: v42
          size: 13GiB (15GB)
          capabilities: 5400rpm gpt-1.00 partitioned partitioned:gpt
          configuration: ansiversion=5 guid=9ceb264d-ecc9-413c-a6dc-180fa42c5342 logicalsectorsize=512 sectorsize=4096
```


## Conclusion

**Less is more.**

This post was nothing more than just a reminder than you don't need a fancy serverless cluster (??) to crunch some serious numbers.

<!-- You don't need this new and fancy serverless architecture to scale. -->

<!-- The next time you will want to use this shiny distributed gadget, please think twice about the tradeoffs you are making and the real load (even x10) of your service. It surely won't be 1.3B inserts per day in the first few months of launching your SaaS. -->

We all agree that testing new tech is fun and exciting, myself included (after all you are on a blog talking about Rust). But new tech is often marketed by hiding its drawbacks, and they will come back to bite you at the worst time when scaling your business.

If SQLite is not the best choice for you due to its [anemic and dynamic typing](https://www.sqlite.org/datatype3.html), or its lack of high-availability, take a look at [PostgreSQL](https://www.postgresql.org) :)


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/high_performance_rust_with_sqlite) (please don't forget to star the repo 🙏)
