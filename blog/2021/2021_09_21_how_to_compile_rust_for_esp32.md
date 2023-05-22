+++
date = 2021-09-21T13:00:00Z
title = "How to compile Rust and LLVM for ESP32 on a Raspberry Pi (aarch64)"
type = "post"
tags = ["rust", "programming", "tutorial", "raspberry-pi", "esp32"]
authors = ["Sylvain Kerkour"]
url = "/compile-rust-for-esp32-xtensa-on-raspberry-pi-aarch64"

[extra]
lang = "en"

comment ="""
"""
+++

ESP32 is a series of microcontrollers that has the particularity to have both integrated Bluetooth and Wi-Fi. It makes it the perfect microcontroller when you need low-power, networked, and smart hardware that doesn't require an OS.

I had to run compile and run Rust code on an ESP32. Unfortunately, they don't run a traditional architecture but the [Xtensa instruction set](https://en.wikipedia.org/wiki/Tensilica#Xtensa_instruction_set) that is currently not officially supported by the Rust compiler.

The [esp-rs](https://github.com/esp-rs) GitHub organization contains the [fork of the Rust](https://github.com/esp-rs/rust) compiler to support the Xtensa architecture.

Unfortunately, it seems that the `src/llvm-project` submodule is out of date and does not support the `aarch64` (`arm64`) architecture, as when trying to follow the instructions I encountered the following error:

```
cargo:warning=cc: error: esp32/rust/src/llvm-project/compiler-rt/lib/builtins/aarch64/lse.S: No such file or directory
cargo:warning=cc: fatal error: no input files
cargo:warning=compilation terminated.
exit status: 1
```

Which, according to [this issue](https://github.com/esp-rs/rust/issues/62), is due to an outdated LLVM version.

Here is how I fixed the build and successfully built Rust and LLVM to compile Rust binaries for ESP32:

## The prerequisites

You will need at least 20GB of free space and a Rust toolchain: [https://rustup.rs](https://rustup.rs).

## Building Rust

```shell
# starting from $HOME
$ mkdir esp32
$ cd esp32
$ git clone https://github.com/esp-rs/rust.git
$ cd rust
```

Here is the trick: you need to update the `src/llvm-project` submodule:
```shell
$ git submodule update --init --recursive --remote src/llvm-project
```

Then, you can finally build Rust (**warning**: it will take a lot of time, so I recommend to run it for the night):

```shell
$ ./configure --experimental-targets=Xtensa
$ python3 x.py dist --stage 2
```

```shell
# still in esp32/rust
$ rustup toolchain link esp `pwd`/build/aarch64-unknown-linux-gnu/stage2
```


## Building LLVM

Still in your recently cloned `rust` folder (**warning**: it will take a lot of time, so I recommend to run it for the night):

```shell
$ cd src/llvm-project
$ mkdir build
$ cd build
$ cmake -G Ninja -DLLVM_ENABLE_PROJECTS='clang' -DCMAKE_BUILD_TYPE=Release ../llvm
$ cmake --build .
$ export PATH="$HOME/esp32/rust/src/llvm-project/build/bin:$PATH"
```


Your Rust toolcahin for ESP32 is now set up!

You can check it by running:
```shell
$ rustc --print target-list | grep xtensa
```

and:
```shell
$ which clang
```

Which should return your newly built `clang`.

See you tomorrow for a post on [how to build and flash a Rust program to an ESP32](https://kerkour.com/rust-on-esp32/).
