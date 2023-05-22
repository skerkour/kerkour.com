+++
date = 2021-08-03T16:44:00Z
title = "How to deploy Rust on Heroku (with Docker)"
type = "post"
tags = ["rust", "programming", "tutorial", "devops"]
authors = ["Sylvain Kerkour"]
url = "/deploy-rust-on-heroku-with-docker"

[extra]
lang = "en"

comment ="""

"""
+++

Due to its unrivaled reliability and performance, Rust is more and more appreciated by companies for web development. And when we talk about web development, Heroku is never far away.

So here is the easiest way to deploy a Rust app on Heroku (whether it be a worker or a webapp) 🚀


## Our application

We will deploy a simple web server using [warp](https://docs.rs/warp).

**Cargo.toml**
```toml
[package]
name = "myapp"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
tokio = { version = "1", features = ["full"] }
warp = "0.3"
```

**main.rs**
```rust
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Heroku injects the port as an environment variable
    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))?;

    // GET /hello
    let hello = warp::path("hello").and(warp::get()).map(|| "World");

    // GET /world
    let world = warp::path("world").and(warp::get()).map(|| "Hello");

    // fallback
    let fallback = warp::any().map(|| "Not found");

    let routes = hello.or(world).or(fallback);

    println!("Starting server on port: {}", port);
    let (_addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], port), async {
            tokio::signal::ctrl_c()
                .await
                .expect("http_server: Failed to listen for CRTL+c");
            println!("Shutting down HTTP server");
        });

    server.await;

    Ok(())
}
```

Please note how simple it is to enable graceful shutdown :)



## A small Docker image

The simplest way to deploy Rust on Heroku is to use Docker. Why? With Docker you fully control your CI pipeline (more on that later), and it can be deployed anywhere if Heroku is finally not your style.


As we [saw previously](https://kerkour.com/rust-small-docker-image/), building minimal Docker images to deploy Rust brings up a lot of benefits: it's not only good for security (reduced attack surface) but also to improve deployment times, reduce costs (less bandwidth and storage), and reduce the risk of dependency conflicts.

Here is our small, two-stages Dockerfile.


**Dockerfile**
```dockerfile
####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=myapp
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /myapp

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /myapp

# Copy our build
COPY --from=builder /myapp/target/x86_64-unknown-linux-musl/release/myapp ./

# Use an unprivileged user.
USER myapp:myapp

CMD ["/myapp/myapp"]
```

Why `FROM alpine` and not `FROM scratch`?

Because [Heroku do weird things](https://devcenter.heroku.com/articles/container-registry-and-runtime#dockerfile-commands-and-runtime) to inject the environment variables, it's easier to use an image with a shell and Docker's [CMD](https://docs.docker.com/engine/reference/builder/#cmd) instruction that to hack around [ENTRYPOINT](https://docs.docker.com/engine/reference/builder/#entrypoint).


If you try to use `FROM scratch` without an `ENTRYPOINT`, Heroku won't be able to start your container because it will look for a shell that is not present.



## CI/CD

And, last but not least, we need to set up a CI/CD pipeline to automagically test, build and deploy our app. CI pipelines are a commodity nowadays, so there is no need for Heroku's integrated solution, which is, in my opinion, not the best one.

Here we use [GitHub Actions](https://docs.github.com/en/actions), but it's roughly the same with any Devops platform.


First, we need to set up the following secrets:
```
HEROKU_API_KEY=101010...
HEROKU_APP=example
```

**.github/workflows/ci.yml**
```yaml
name: CI

on:
  push:
    branches:
      - main

jobs:

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Update local toolchain
        run: |
          rustup update
          rustup component add clippy
          rustup install nightly

      - name: Toolchain info
        run: |
          cargo --version --verbose
          rustc --version
          cargo clippy --version

      - name: Test
        run: |
          cargo check
          cargo clippy -- -D warnings
          cargo test --all


      - name: Build Docker image
        run: docker build -t registry.heroku.com/${{ secrets.HEROKU_APP }}/web:latest .

      - name: Docker image info
        run: docker images

      - name: Login to container registry
        env:
          HEROKU_API_KEY: ${{ secrets.HEROKU_API_KEY }}
        run: heroku container:login

      - name: Push Docker image
        # see here for the details of the release phase https://devcenter.heroku.com/articles/container-registry-and-runtime#release-phase
        run: docker push registry.heroku.com/${{ secrets.HEROKU_APP }}/web

      - name: Release
        env:
          HEROKU_API_KEY: ${{ secrets.HEROKU_API_KEY }}
        run: heroku container:release -a ${{ secrets.HEROKU_APP }} web
```


## Conclusion

Deploying Rust apps on Heroku is easy thanks to Docker. If Heroku is too expensive or not flexible enough for your use case, switching to another platform such as [Render](https://render.com/) or [Digital Ocean App platform](https://www.digitalocean.com/products/app-platform/) is just a few clicks away.


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/deploy_rust_on_heroku) (please don't forget to star the repo 🙏)
