+++
date = 2021-09-27T06:00:00Z
title = "Reproducible cross-compilation for Rust (with Docker)"
type = "post"
tags = ["rust", "programming", "tutorial", "hacking", "devops"]
authors = ["Sylvain Kerkour"]
url = "/rust-reproducible-cross-compilation-with-docker"

[extra]
lang = "en"

comment ="""

"""
+++

```shell
Error: Toolchain / Library XX not found. Aborting compilation.
```

How many times did you get this kind of message when trying to follow the build instructions of a project, or cross compile it?

This problem can be solved with immutable toolchains and reproductible build environements. All of that powered by Docker.


## Cross-compile a Rust application from Linux to Windows


**Dockerfile.windows**
```dockerfile
FROM rust:latest

RUN apt update && apt upgrade -y
RUN apt install -y g++-mingw-w64-x86-64

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup toolchain install stable-x86_64-pc-windows-gnu

WORKDIR /app

CMD ["cargo", "build", "--target", "x86_64-pc-windows-gnu"]
```


```shell
$ docker build . -t rust_cross_compile/windows -f Dockerfile.windows
# in your Rust project
$ docker run --rm -ti -v `pwd`:/app rust_cross_compile/windows
```


## Cross-compile a Rust application to aarch64 (armv8)


**Dockerfile.aarch64**
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

```shell
$ docker build . -t rust_cross_compile/aarch64 -f Dockerfile.aarch64
# in your Rust project
$ docker run --rm -ti -v `pwd`:/app rust_cross_compile/aarch64
```

## Cross-compile a Rust application to armv7


**Dockerfile.armv7**
```dockerfile
FROM rust:latest

RUN apt update && apt upgrade -y
RUN apt install -y g++-arm-linux-gnueabihf libc6-dev-armhf-cross

RUN rustup target add armv7-unknown-linux-gnueabihf
RUN rustup toolchain install stable-armv7-unknown-linux-gnueabihf

WORKDIR /app

ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
    CC_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc \
    CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++

CMD ["cargo", "build", "--target", "armv7-unknown-linux-gnueabihf"]
```

```shell
$ docker build . -t rust_cross_compile/armv7 -f Dockerfile.armv7
# in your Rust project
$ docker run --rm -ti -v `pwd`:/app rust_cross_compile/armv7
```

## Cross

Now, enough for the theory. In practice, you should use [cross](https://github.com/rust-embedded/cross), a project developed and maintained by the [Tools team](https://github.com/rust-embedded/wg#the-tools-team) which allows you to easily cross-compile Rust projects without messing with custom Dockerfiles.

```shell
$ cargo install -f cross
```

```shell
$ cross build --target aarch64-unknown-linux-gnu
$ cross build --target x86_64-pc-windows-gnu
$ cross build --target armv7-unknown-linux-gnueabihf
# ...
```

You can learn more about it on [GitHub](https://github.com/rust-embedded/cross).


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_reproductible_cross_compilation) (please don't forget to star the repo 🙏).

