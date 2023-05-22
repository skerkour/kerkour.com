+++
date = 2021-04-06T06:00:00Z
title = "How to create small Docker images for Rust"
type = "post"
tags = ["rust", "programming", "tutorial", "docker", "devops", "musl", "debian", "linux"]
authors = ["Sylvain Kerkour"]
url = "/rust-small-docker-image"

[extra]
lang = "en"

comment ="""
resources

https://bionic.fullstory.com/rust-at-fullstory-part-2/
https://www.ditto.live/blog/posts

https://news.ycombinator.com/item?id=23008399

https://news.ycombinator.com/item?id=26263518
"""
+++


Building minimal Docker images to deploy Rust brings up a lot of benefits: it's not only good for security (reduced attack surface) but also to improve deployment times, reduce costs (less bandwidth and storage), and reduce the risk of dependency conflicts.


## Table of contents

* [Code](#code)
* [FROM scratch (15.9MB)](#from-scratch)
* [FROM alpine (21.6MB)](#from-alpine)
* [FROM gcr.io/distroless/cc (33MB)](#from-distrolesscc)
* [FROM buster-slim (79.4MB)](#from-buster-slim)
* [Conclusion](#conclusion)
* [The code is on GitHub](#the-code-is-on-github)


## Code

Our "app" is rather simple: we are going to build a command-line utility that calls `https://api.myip.com` and prints the result.

Making HTTPS calls is interesting because it requires a library to interact with TLS, usually `openssl`. But in order to build the smallest Docker image possible, we need to statically link our program, and statically linking `openssl` is not that easy. That's why we will avoid `openssl` and use [`rustls`](https://github.com/ctz/rustls) instead.

Let's ignore the `Jemalloc` thing for a moment.

```shell
$ cargo new myip
```

**Cargo.toml**
```toml
[package]
name = "myip"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
serde = { version = "1", features = ["derive"] }
reqwest = { version = "0.11", default-features = false, features = ["json", "rustls-tls", "blocking"] }


[target.'cfg(all(target_env = "musl", target_pointer_width = "64"))'.dependencies.jemallocator]
version = "0.3"
```

**main.rs**
```rust
use serde::Deserialize;
use std::error::Error;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Deserialize, Debug)]
struct ApiRes {
    ip: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let res = reqwest::blocking::get("https://api.myip.com")?.json::<ApiRes>()?;

    println!("{}", res.ip);

    Ok(())
}
```

```shell
$ cargo run
     Running `target/debug/myip`
127.0.0.1
```

## FROM scratch

Size: **15.9 MB**

In order to use `FROM scratch` as the base image, we have to statically link our program to the `musl libc` because `glibc` is unavailable in `scratch`. It can be achieved by using the `x86_64-unknown-linux-musl` target.

A problem with this approach is that `musl`'s memory allocator is not optimized for speed and may [reduce your app's performance](https://www.linkedin.com/pulse/testing-alternative-c-memory-allocators-pt-2-musl-mystery-gomes), especially when dealing with high throughput applications.

This is why we used [`jemalloc`](https://github.com/jemalloc/jemalloc), a memory allocator designed for highly concurrent applications.

Be aware that [some people are reporting errors using this allocator](https://andygrove.io/2020/05/why-musl-extremely-slow/), so watch your logs ;)

As a data point, [I've served millions of HTTP requests using it](https://github.com/skerkour/bloom), without problems.


**Dockerfile.scratch**
```dockerfile
####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=myip
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /myip

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /myip

# Copy our build
COPY --from=builder /myip/target/x86_64-unknown-linux-musl/release/myip ./

# Use an unprivileged user.
USER myip:myip

CMD ["/myip/myip"]
```

```shell
$ docker build -t myip:scratch -f Dockerfile.scratch .
# ...
$ docker run -ti --rm myip:scratch
127.0.0.1
```

{{< subscribe_form >}}


## FROM alpine

Size: **21.6MB**

[Alpine Linux](https://alpinelinux.org) *is a security-oriented, lightweight Linux distribution based on musl libc and busybox*.

It should be used when `FROM scratch` is not enough and you need a package manager to install dependencies such as `chromium` or `ssh`.

As it's based on `musl libc` is has the same constraints as `FROM scratch`, and we need to statically link our Rust program using `x86_64-unknown-linux-musl`.


**Dockerfile.alpine**
```dockerfile
####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=myip
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /myip

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /myip

# Copy our build
COPY --from=builder /myip/target/x86_64-unknown-linux-musl/release/myip ./

# Use an unprivileged user.
USER myip:myip

CMD ["/myip/myip"]
```

```shell
$ docker build -t myip:alpine -f Dockerfile.alpine .
# ...
$ docker run -ti --rm myip:alpine
127.0.0.1
```


## FROM distroless/cc

Size: **38.3**

We can also use the [distroless](https://github.com/GoogleContainerTools/distroless) family of images maintained by Google that use packages from debian, but remove all the useless packages in order to create minimal images. Thus, we no longer need to use the MUSL libc.

Here we use the the [distroless/cc](https://github.com/GoogleContainerTools/distroless/tree/main/cc) image because Rust needs `libgcc1` for unwinding.


**Dockerfile.distroless**
```dockerfile
####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=myip
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /myip

COPY ./ .

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /myip

# Copy our build
COPY --from=builder /myip/target/release/myip ./

# Use an unprivileged user.
USER myip:myip

CMD ["/myip/myip"]
```

```shell
$ docker build -t myip:distroless -f Dockerfile.distroless .
# ...
$ docker run -ti --rm myip:distroless
127.0.0.1
```



## FROM buster-slim

Size: **79.4MB**

In this last example, we will use `debian:buster-slim` as the base image. As Debian is based on `glibc` we no longer need to use the `x86_64-unknown-linux-musl` compilation target.

**Dockerfile.debian**
```dockerfile
####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=myip
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /myip

COPY ./ .

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:buster-slim

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /myip

# Copy our build
COPY --from=builder /myip/target/release/myip ./

# Use an unprivileged user.
USER myip:myip

CMD ["/myip/myip"]
```

```shell
$ docker build -t myip:debian -f Dockerfile.debian .
# ...
$ docker run -ti --rm myip:debian
127.0.0.1
```



## Conclusion

```shell
$ docker images
REPOSITORY    TAG           IMAGE ID       CREATED          SIZE
myip     scratch       795604e74501   9 minutes ago    15.9MB
myip     alpine        9a26400587a2   2 minutes ago    21.6MB
myip     distroless    b789c964a680   19 seconds ago   33MB
myip     debian        c388547b9486   12 seconds ago   79.4MB
```

Here we focused on Docker, but if the images are still too large for you and you know what you are doing, [here are a few tricks to minimize Rust binary size](https://kerkour.com/optimize-rust-binary-size) and reduces the size of the images further.


<!-- [https://github.com/johnthagen/min-sized-rust](https://github.com/johnthagen/min-sized-rust) and [https://arusahni.net/blog/2020/03/optimizing-rust-binary-size.html](https://arusahni.net/blog/2020/03/optimizing-rust-binary-size.html). -->

For example, using the following in **Cargo.toml**:
```toml
[profile.release]
lto = true
codegen-units = 1
```

and adding the following in the **Dockerfile** after the `cargo build` step:

```dockerfile
RUN strip -s /myip/target/release/myip
```

gives:

```shell
$ docker images
REPOSITORY        TAG           IMAGE ID       CREATED          SIZE
myip              scratch       de26b0460262   17 minutes ago   4.2MB
myip              alpine        4188ccc82662   6 minutes ago    9.81MB
myip              distroless    c46e6a0c1ac3   7 seconds ago    25.6MB
myip              debian        0eefb58278a8   4 seconds ago    72.8MB
```


If you want to learn more from real-world Rust experience, I wrote a book where, among other things, we create and deploy a Rust service using Docker. All early-access supporters get a special discount and awesome bonuses: [https://kerkour.com/black-hat-rust](https://kerkour.com/black-hat-rust)



## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/2021_04_06_rust_minimal_docker_image/myip) (please don't forget to star the repo 🙏).


**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) to learn Rust, Security and Cryptography and where we use Docker for offensive purpose.**
