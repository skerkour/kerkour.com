+++
date = 2021-11-11T06:00:00Z
title = "Converting an Enum to a String in Rust"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-enum-to-string"


[extra]
lang = "en"

comment ="""

"""
+++

The easiest way to convert an `enum` to a `String` in Rust is to implement the [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait.

```rust
#[derive(Debug, Clone, Copy)]
enum Platform {
    Linux,
    MacOS,
    Windows,
    Unknown,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Platform::Linux => write!(f, "Linux"),
            Platform::Macos => write!(f, "macOS"),
            Platform::Windows => write!(f, "Windows"),
            Platform::Unknown => write!(f, "unknown"),
        }
    }
}
```


Then you can call the `to_string()` method.

```rust
fn main() {
    let platform: String = Platform::Unknown.to_string();

    println!("{}", platform);
}
```

```shell
$ cargo run
unknown
```
