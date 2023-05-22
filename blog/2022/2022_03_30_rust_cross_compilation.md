+++
date = 2022-03-30T02:00:00Z
title = "Cross-compilation in Rust"
type = "post"
tags = ["rust", "programming", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/rust-cross-compilation"

[extra]
lang = "en"

comment ="""
"""
+++

> This post contains excerpts from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


Now we have a mostly secure RAT, it's time to expand our reach.


Until now, we limited our builds to Linux. While the Linux market is huge server-side, this is another story client-side, with a market share of roughly [2.5% on the desktop](https://gs.statcounter.com/os-market-share/desktop/worldwide/).

To increase the number of potential targets, we are going to use cross-compilation: we will compile a program from a Host Operating System for a different Operating System. Compiling Windows executables on Linux, for example.

But, when we are talking about cross-compilation, we are not only talking about compiling a program from an OS to another one. We are also talking about compiling an executable from one architecture to another. From `x86_64` to `aarch64` (also known as `arm64`), for example.

In this chapter, we are going to see why and how to cross-compile Rust programs and how to avoid the painful edge-cases of cross-compilation, so stay with me.


## Why multi-platform

From computers to smartphones passing by smart TVs, IoT such as cameras or "smart" fridges... Today's computing landscape is kind of the perfect illustration of the word "fragmentation".

<!-- Even if 2 different devices run the same Operating System, manufacturers love to customize the devices  -->

Thus, if we want our operations to reach more targets, our RAT needs to support many of those platforms.


### Platform specific APIs

Unfortunately, OS APIs are not portable: for example, persistence techniques(the act of making the execution of a program persist across restarts) are very different if you are on Windows or on Linux.

The specificities of each OS force us to craft platform-dependent of code.

Thus we will need to write some parts of our RAT for windows, rewrite the same part for Linux, and rewrite it for macOS...

The goal is to write as much as possible code that is shared by all the platforms.


## Cross-platform Rust

Thankfully, Rust makes it easy to write code that will be conditionally compiled depending on the platform it's compiled for.

### The `cfg` attribute

<!-- https://doc.rust-lang.org/reference/conditional-compilation.html -->

The `cfg` attribute enables the conditional compilation of code. It [supports many options](https://doc.rust-lang.org/reference/conditional-compilation.html) so you can choose on which platform to run which part of your code.


For example: `#[cfg(target_os = "linux")]`, `#[cfg(target_arch = "aarch64")]`, `#[cfg(target_pointer_width = "64")]`;


Here is an example of code that exports the same `install` function but picks the right one depending on the target platform.

**[ch_12/rat/agent/src/install/mod.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_12/rat/agent/src/install/mod.rs)**
```rust
// ...

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::install;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::install;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::install;
```

Then, in the part of the code that is shared across platforms, we can import and use it like any module.

```rust
mod install;

// ...

install::install();
```

The `cfg` attribute can also be used with `any`, `all`, and `not`:


```rust
// The function is only included in the build when compiling for macOS OR Linux
#[cfg(any(target_os = "linux", target_os = "macos"))]
// ...

// This function is only included when compiling for Linux AND the pointer size is 64 bits
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
// ...


// This function is only included when the target Os IS NOT Windows
#[cfg(not(target_os = "windows"))]
// ...
```


### Platform dependent dependencies

We can also conditionally import dependencies depending on the target.

For example, we are going to import the `winreg` crate to interact with Windows' registry, but it does not makes sense to import, or even build this crate for platforms different thant Windows.

**[ch_12/rat/agent/Cargo.toml](https://github.com/skerkour/black-hat-rust/blob/main/ch_12/rat/agent/Cargo.toml)**
```toml
[target.'cfg(windows)'.dependencies]
winreg = "0.10"
```

## Supported platforms

The Rust project categorizes the supported platforms into 3 tiers.

- **Tier 1** targets can be thought of as "guaranteed to work".
- **Tier 2** targets can be thought of as "guaranteed to build".
- **Tier 3** targets are those for which the Rust codebase has support for but which the Rust project does not build or test automatically, so they may or may not work.

Tier 1 platforms are the followings:

- `aarch64-unknown-linux-gnu`
- `i686-pc-windows-gnu`
- `i686-pc-windows-msvc`
- `i686-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `x86_64-pc-windows-gnu`
- `x86_64-pc-windows-msvc`
- `x86_64-unknown-linux-gnu`

You can find the platforms for the other tiers in the official documentation: [https://doc.rust-lang.org/nightly/rustc/platform-support.html](https://doc.rust-lang.org/nightly/rustc/platform-support.html).


In practical terms, it means that our RAT is guaranteed to work on Tier 1 platforms without problems (or it will be handled by the Rust teams). For Tier 2 platforms, you will need to write more tests to be sure that everything works as intended.



## Cross-compilation

```default
Error: Toolchain / Library XX not found. Aborting compilation.
```

How many times did you get this kind of message when trying to follow the build instructions of a project or cross-compile it?


What if, instead of writing wonky documentation, we could consign the build instructions into an immutable recipe that would guarantee us a successful build 100% of the time?

This is where Docker comes into play:


**Immutability**: The `Dockerfile`s are our immutable recipes, and `docker` would be our robot, flawlessly executing the recipes all days of the year.

<!-- But first, let see the tools required to compile Windows executable on a Linux host.


### Mingw
 -->
<!--
### Musl libc and static linking

Rust has a problem: as of today, it depends on the libc of the system, which may cause incompatibilities, while rare, it's possible.
 -->



**Cross-platform**:  Docker is itself available on the 3 major OSes (Linux, Windows, and macOS). Thus, we not only enable a team of several developers using different machines to work together, but we also greatly simplify our toolchains.

By using Docker, we are finally reducing our problem to compiling from Linux to other platforms, instead of:

- From Linux to other platforms
- From Windows to other platforms
- From macOS to other platforms
- ...



<!--
The third reason, is that Rust, may leak some paths of your


paths dans l’executable final (https://www.bleepingcomputer.com/news/security/most-loved-programming-language-rust-sparks-privacy-concerns/)

other benefits of using docker to compile (artifacts of paths in binary)
https://news.ycombinator.com/item?id=26681630https://news.ycombinator.com/item?id=24516554

 -->




## cross

The [Tools team](https://github.com/rust-embedded/wg#the-tools-team) develops and maintains a project named [cross](https://github.com/rust-embedded/cross) which allow you to easily cross-compile Rust projects using Docker, without messing with custom Dockerfiles.

It can be installed like that:
```bash
$ cargo install -f cross
```

`cross` works by using pre-made Dockerfiles, but they are maintained by the Tools team, not you, and they take care of everything.

The list of targets supported is impressive. As I'm writing this, here is the list of supported platforms: [https://github.com/rust-embedded/cross/tree/master/docker](https://github.com/rust-embedded/cross/tree/master/docker)
```default
Dockerfile.aarch64-linux-android
Dockerfile.aarch64-unknown-linux-gnu
Dockerfile.aarch64-unknown-linux-musl
Dockerfile.arm-linux-androideabi
Dockerfile.arm-unknown-linux-gnueabi
Dockerfile.arm-unknown-linux-gnueabihf
Dockerfile.arm-unknown-linux-musleabi
Dockerfile.arm-unknown-linux-musleabihf
Dockerfile.armv5te-unknown-linux-gnueabi
Dockerfile.armv5te-unknown-linux-musleabi
Dockerfile.armv7-linux-androideabi
Dockerfile.armv7-unknown-linux-gnueabihf
Dockerfile.armv7-unknown-linux-musleabihf
Dockerfile.asmjs-unknown-emscripten
Dockerfile.i586-unknown-linux-gnu
Dockerfile.i586-unknown-linux-musl
Dockerfile.i686-linux-android
Dockerfile.i686-pc-windows-gnu
Dockerfile.i686-unknown-freebsd
Dockerfile.i686-unknown-linux-gnu
Dockerfile.i686-unknown-linux-musl
```

```default
Dockerfile.mips-unknown-linux-gnu
Dockerfile.mips-unknown-linux-musl
Dockerfile.mips64-unknown-linux-gnuabi64
Dockerfile.mips64el-unknown-linux-gnuabi64
Dockerfile.mipsel-unknown-linux-gnu
Dockerfile.mipsel-unknown-linux-musl
Dockerfile.powerpc-unknown-linux-gnu
Dockerfile.powerpc64-unknown-linux-gnu
Dockerfile.powerpc64le-unknown-linux-gnu
Dockerfile.riscv64gc-unknown-linux-gnu
Dockerfile.s390x-unknown-linux-gnu
Dockerfile.sparc64-unknown-linux-gnu
Dockerfile.sparcv9-sun-solaris
Dockerfile.thumbv6m-none-eabi
Dockerfile.thumbv7em-none-eabi
Dockerfile.thumbv7em-none-eabihf
Dockerfile.thumbv7m-none-eabi
Dockerfile.wasm32-unknown-emscripten
Dockerfile.x86_64-linux-android
Dockerfile.x86_64-pc-windows-gnu
Dockerfile.x86_64-sun-solaris
Dockerfile.x86_64-unknown-freebsd
Dockerfile.x86_64-unknown-linux-gnu
Dockerfile.x86_64-unknown-linux-musl
Dockerfile.x86_64-unknown-netbsd
```


### Cross-compiling from Linux to Windows

```bash
# In the folder of your Rust project
$ cross build --target x86_64-pc-windows-gnu
```


### Cross-compiling to aarch64 (arm64)

```bash
# In the folder of you Rust project
$ cross build --target aarch64-unknown-linux-gnu
```


### Cross-compiling to armv7

```bash
# In the folder of your Rust project
$ cross build --target armv7-unknown-linux-gnueabihf
```


## Custom Dockerfiles

Sometimes, you may need specific tools in your Docker image, such as a packer (what is a packer? we will see that below) or tools to strip and rewrite the metadata of your final executable.

In this situation, it's legitimate to create a custom Dockerfile and to configure `cross` to use it for a specific target.

Create a `Cross.toml` file in the root of your project (where your `Cargo.toml` file is), with the following content:

```toml
[target.x86_64-pc-windows-gnu]
image = "my_image:tag"
```

We can also completely forget `cross` and build our own `Dockerfiles`. Here is how.

### Cross-compiling from Linux to Windows

**[ch_12/rat/docker/Dockerfile.windows](https://github.com/skerkour/black-hat-rust/blob/main/ch_12/rat/docker/Dockerfile.windows)**
```dockerfile
FROM rust:latest

RUN apt update && apt upgrade -y
RUN apt install -y g++-mingw-w64-x86-64

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup toolchain install stable-x86_64-pc-windows-gnu

WORKDIR /app

CMD ["cargo", "build", "--target", "x86_64-pc-windows-gnu"]
```

```bash
$ docker build . -t black_hat_rust/ch12_windows -f Dockerfile.windows
# in your Rust project
$ docker run --rm -ti -v `pwd`:/app black_hat_rust/ch12_windows
```

## Cross-compiling to aarch64 (arm64)

**[ch_12/rat/docker/Dockerfile.aarch64](https://github.com/skerkour/black-hat-rust/blob/main/ch_12/rat/docker/Dockerfile.aarch64)**
```dockerfile
FROM rust:latest

RUN apt update && apt upgrade -y
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross

RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup toolchain install stable-aarch64-unknown-linux-gnu

WORKDIR /app

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

CMD ["cargo", "build", "--target", "aarch64-unknown-linux-gnu"]
```


```bash
$ docker build . -t black_hat_rust/ch12_linux_aarch64 -f Dockerfile.aarch64
# in your Rust project
$ docker run --rm -ti -v `pwd`:/app black_hat_rust/ch12_linux_aarch64
```

**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) to learn Rust, Cybersecurity and Cryptography.**
