+++
date = 2022-05-24T02:00:00Z
title = "Optimizing the size of your Rust binaries"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/optimize-rust-binary-size"

[extra]
lang = "en"

comment ="""
"""
+++


By default, Rust produces fairly large binaries, which may be annoying when building a RAT. A larger executable means more resources used on the system, longer and less reliable downloads, and easier to be detected.

We will see a few tips to reduce the size of a Rust executable.

Note that each of the following points may come with drawbacks, so you are free to mix them according to your own needs.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


## Optimization Level

In `Cargo.toml`

```toml
[profile.release]
opt-level = 'z' # Optimize for size
```

## Link Time Optimization (LTO)

In `Cargo.toml`

```toml
[profile.release]
lto = true
```


## Parallel Code Generation Units

In `Cargo.toml`

```toml
[profile.release]
codegen-units = 1
```


Note that those techniques may slow down the compilation, especially Parallel Code Generation Units. In return, the compiler will be able to better optimize your binary.


## Choosing the right crates

Finally, choosing small crates can have the biggest impact on the size of the final executable. You can use [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) to find which crates are bloating your project and thus find alternatives, as we did for the agent's HTTP client library.
