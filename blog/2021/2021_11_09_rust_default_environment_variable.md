+++
date = 2021-11-09T06:00:00Z
title = "Default environment variables in Rust"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-default-environment-variables"


[extra]
lang = "en"

comment ="""

"""
+++

Rust does not provide a built-in way to set a default environment variable. Yet, it's easy to achieve with the [`unwrap_or`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or) combinator.


```rust
fn main() {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
}
```

Alternatively, you can define a function that sets a default env value if none is set.

```rust
fn set_default_env_var(key: &str, value: &str) {
    if std::env::var(key).is_err() {
        std::env::set_var(key, value);
    }
}

fn main() {
    set_default_env_var("PORT", "8080");

    let port = std::env::var("PORT").unwrap();

    println!("{}", port);
}
```

```shell
$ cargo run
8080
```
