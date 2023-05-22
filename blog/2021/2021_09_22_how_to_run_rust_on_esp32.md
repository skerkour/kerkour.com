+++
date = 2021-09-22T12:00:00Z
title = "Rust on ESP32"
type = "post"
tags = ["rust", "programming", "tutorial", "esp32"]
authors = ["Sylvain Kerkour"]
url = "/rust-on-esp32"

[extra]
lang = "en"

comment ="""
"""
+++

I recently got my hands on an ESP32 microcontroller for hacking purpose and successfully ran Rust code on it 🦀

## Installing the tools

First, you need to download and install [the prebuilt binaries of the Rust and LLVM compiler forks by Espressif](https://github.com/espressif/rust-esp32-example/blob/main/docs/rust-on-xtensa.md) or follow yesterday's guide about [how to compile Rust and LLVM for ESP32](https://kerkour.com/compile-rust-for-esp32-xtensa-on-raspberry-pi-aarch64/) (**warning**: Even if Rust and LLVM compile successfully, it's today not possible to build a Rust program for ESP32 on Raspberry Pi due to [https://github.com/esp-rs/esp-idf-sys/issues/14](https://github.com/esp-rs/esp-idf-sys/issues/14), but flashing works).


First, you need to install the auxiliary tools:

```shell
$ cargo install -f ldproxy espflash espmonitor
```


Then:
```shell
$ rustup default esp
```


## Our Rust program

Here is the simplest "Hello world" you can have. As you may have noticed, the `std` lib is available :)


**main.rs**
```rust
use embedded_svc::anyerror::*;
use esp_idf_hal::prelude::*;
use esp_idf_svc::sysloop::*;
use std::{thread, time::Duration};

fn main() -> anyhow::Result<()> {
    loop {
        println!("Hello world");
        thread::sleep(Duration::from_secs(1));
    }
}
```

**build.rs** (from [https://github.com/ivmarkov/rust-esp32-std-hello](https://github.com/ivmarkov/rust-esp32-std-hello))
```rust
use std::path::PathBuf;

use embuild::{
    self, bingen,
    build::{CfgArgs, LinkArgs},
    cargo, symgen,
};

// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> anyhow::Result<()> {
    LinkArgs::output_propagated("ESP_IDF")?;

    let cfg = CfgArgs::try_from_env("ESP_IDF")?;

    if cfg.get("esp32s2").is_some() {
        let ulp_elf = PathBuf::from("ulp").join("rust-esp32-ulp-hello");
        symgen::run(&ulp_elf, 0x5000_0000)?; // This is where the RTC Slow Mem is mapped within the ESP32-S2 memory space
        bingen::run(&ulp_elf)?;

        cargo::track_file(ulp_elf);
    }

    cfg.output();

    Ok(())
}
```


**Cargo.toml**
```toml
[package]
name = "rust_on_esp32"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

# No xtensa in regular compiler yet
[package.metadata.docs.rs]
default-target = "x86_64-unknown-linux-gnu"

[profile.release]
# symbols are nice and they don't increase the size on Flash
debug = true
opt-level = "z"

[profile.dev]
opt-level = "s"

[features]
bind = []

[dependencies]
anyhow = {version = "1", features = ["backtrace"]}
esp-idf-sys = { version = "0.20" }
embedded-svc = "0.8.3"
esp-idf-svc = { version = "0.20", features = ["binstart"] }
esp-idf-hal = "0.20"

[build-dependencies]
embuild = "0.24"
anyhow = "1"
```

**.cargo/config.toml** (from [https://github.com/ivmarkov/rust-esp32-std-hello](https://github.com/ivmarkov/rust-esp32-std-hello))
```toml
[build]
target = "xtensa-esp32-espidf"

[target.xtensa-esp32-espidf]
linker = "ldproxy"

[unstable]
build-std = ["std", "panic_abort"]
build-std-features = ["panic_immediate_abort"]
configurable-env = true # No longer necessary since 1.56, as it was stabilized: https://github.com/rust-lang/cargo/blob/master/src/cargo/core/features.rs#L698
extra-link-arg = true   # No longer necessary since 1.56, as it was stabilized: https://github.com/rust-lang/cargo/blob/master/src/cargo/core/features.rs#L695

[env]
ESP_IDF_SYS_GLOB_BASE = { value = ".", relative = true }

ESP_IDF_SYS_GLOB_0 = { value = "/sdkconfig.release" }
ESP_IDF_SYS_GLOB_1 = { value = "/sdkconfig.debug" }
ESP_IDF_SYS_GLOB_2 = { value = "/sdkconfig.defaults" }
```

**sdkconfig.defaults** (from [https://github.com/ivmarkov/rust-esp32-std-hello](https://github.com/ivmarkov/rust-esp32-std-hello))
```
CONFIG_LWIP_L2_TO_L3_COPY=y
CONFIG_LWIP_IP_FORWARD=y
CONFIG_LWIP_IPV4_NAPT=y
#CONFIG_ESP_SYSTEM_USE_EH_FRAME=y
#CONFIG_COMPILER_CXX_EXCEPTIONS=y
```

## Compiling the Rust program for ESP32

Compiling it is as simple as running:
```shell
$ cargo build # or cargo build --release
```


## Flashing the ESP32

Thanks to [espflash](https://crates.io/crates/espflash) (which just reached `v1.0` 🎉) flashing our binary to an ESP32 board is easy:

Press the `BOOT` button on the ESP32, then:
```shell
$ espflash /dev/ttyUSB0 target/xtensa-esp32-espidf/debug/rust_on_esp32 # or target/xtensa-esp32-espidf/release/rust_on_esp32
```


## Monitoring

Finally, getting logs can be achieved with [espmonitor](https://crates.io/crates/espmonitor) that we previously installed:
```shell
$ espmonitor --speed 115200 /dev/ttyUSB0
```


## Getting Wi-Fi to work

You can find a more advanced and complex example (from which this Hello-World is derived) with Wi-Fi and a web server on GitHub: [https://github.com/ivmarkov/rust-esp32-std-hello](https://github.com/ivmarkov/rust-esp32-std-hello)


## Want to learn more?

Explore the [esp-rs GitHub organization](https://github.com/esp-rs) and Join the [https://matrix.to/#/#esp-rs:matrix.org](https://matrix.to/#/#esp-rs:matrix.org) matrix channel.


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_on_esp32) (please don't forget to star the repo 🙏).
