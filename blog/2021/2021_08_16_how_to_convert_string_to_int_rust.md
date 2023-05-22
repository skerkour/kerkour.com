+++
date = 2021-08-16T14:00:00Z
title = "How to convert String to Int and Int to String in Rust"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-string-to-int-and-int-to-string"

[extra]
lang = "en"

comment ="""

"""
+++


## String to Int

To convert a string (or a `&str`) to an integer in Rust, we can use the `parse` method:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // String to int
    let s1 = String::from("42");
    let n1 = s1.parse::<u64>()?;
    // or
    let n2: u64 = s1.parse()?;

    Ok(())
}
```


## Int to String

On the other hand, converting an integer to an owned String can be achieved with the `format!` macro.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n2: u64 = 42;

    // int to string
    let s2 = format!("{}", n2);
    Ok(())
}
```


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_string_to_int_and_int_to_string) (please don't forget to star the repo 🙏)
