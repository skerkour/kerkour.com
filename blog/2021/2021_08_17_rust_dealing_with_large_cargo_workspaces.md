+++
date = 2021-08-17T14:00:00Z
title = "How to deal with large Cargo workspaces in Rust"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-large-cargo-workspace"

[extra]
lang = "en"

comment ="""
<!-- But then you start to have some problem with project maintenance, your dependencies declaration are all over the place and it can become tedious to update the same dependency in 10 different `Cargo.toml` files.

I've found this little trick to ease the maintenance of large projects:

Using a `libs` crate which will declare all the dependencies.

Thus, when update your dependencies, you will have only one place to look and update.
 -->
"""
+++


I'm a big fan of monoliths, but when Rust projects become larger and larger, we have to use [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). First to benefit from incremental compilation to speed up compile times, second to improve code organization.

Then a big problem arises: the declaration of our dependencies is scattered all over the place, with the same dependencies declared in dozens of different `Cargo.toml` files. Updating a single dependency used by all our packages (`uuid` for example) now requires us to crawl through all these `Cargo.toml` files and manually update the dependencies one by one. This problem is exacerbated by the tendency of Rust's ecosystem to have a lot of `0.x` packages.


**So here is a little trick I've found to eases large projects maintenance.**

The thing is to use a single `libs` crate that will declare all our dependencies, re-export these dependencies in `lib.rs`, and then import this `libs` crate in all the other crates of the workspaces.

```shell
$ ls
app
Cargo.lock
Cargo.toml
libs
one
target
three
two
```

**libs/Cargo.toml**
```toml
[package]
name = "libs"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
tokio = { version = "1", features = ["full"] }
uuid = "0.8"
```

**libs/src/libs.rs**
```rust
pub use tokio;
pub use uuid;
```

**one/Cargo.toml**
```toml
[package]
name = "one"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
libs = { path = "../libs" }
```


**one/src/libs.rs**
```rust
use std::time::Duration;
use libs::tokio::time;

pub async fn sleep_one() {
    time::sleep(Duration::from_secs(1)).await;
}
```


**app/Cargo.toml**
```toml
[package]
name = "app"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
libs = { path = "../libs" }

one = { path = "../one" }
two = { path = "../two" }
three = { path = "../three" }
```

**app/src/main.rs**
```rust
use libs::tokio;

#[tokio::main]
async fn main() {
    one::sleep_one().await;
    two::sleep_two().await;
    three::sleep_three().await;
}
```


## Limitations

Be aware that this technique may not work with all crates: some crates providing macros requires to be directly declared in the dependencies list. For example `serde` or `sqlx`.




## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_large_cargo_workspace) (please don't forget to star the repo 🙏)
