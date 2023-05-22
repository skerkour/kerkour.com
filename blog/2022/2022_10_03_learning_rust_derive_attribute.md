+++
date = 2022-10-03T06:30:00Z
title = "Learning Rust: The derive attribute"
type = "post"
tags = ["hacking", "rust", "programming"]
authors = ["Sylvain Kerkour"]
url = "/learning-rust-derive-attribute"

[extra]
lang = "en"
+++

When you have a lot of [traits](https://kerkour.com/rust-generics-traits) to implement for your types, it can quickly become tedious and may add a lot of complexity to your code.

```rust
struct Point {
    x: u64,
    y: u64,
}

impl Debug for Point {
    // ...
}

impl Display for Point {
    // ...
}

impl Something for Point {
    // ...
}

impl SomethingElse for Point {
    // ...
}

// ...
```

Fortunately, Rust has something for us: the `derive` [attribute](https://doc.rust-lang.org/rust-by-example/attribute.html).

By using the `derive` attribute, we are actually feeding our types to a [Derive macro](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros) which is a kind of [procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html#procedural-macros).

They take code as input (in this case, our type), and create more code as output. At compile-time.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


This is especially useful for data deserialization: Just by implementing the  [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits from the [`serde`](https://docs.rs/serde) crate, the (almost) universally used serialization library in the Rust world, we can then serialize and deserialize our types to a lot of data formats: [JSON](https://github.com/serde-rs/json), [YAML](https://github.com/dtolnay/serde-yaml), [TOML](https://github.com/alexcrichton/toml-rs), [BSON](https://github.com/mongodb/bson-rust) and so on...


```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Point {
    x: u64,
    y: u64,
}
```

Without much effort, we just implemented the [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html), [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits for our `struct Point`.


One thing to note is that all the subfields of your `struct` need to implement the traits:
```rust
use serde::{Serialize, Deserialize};

// Not possible:
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Point<T> {
    x: T,
    y: T,
}

// instead, you need to do this:
use serde::{Serialize, Deserialize};
use core::fmt::Debug; // Import the Debug trait

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Point<T: Debug + Clone + Serialize + Deserialize> {
    x: T,
    y: T,
}
```

**Want to learn more about Rust, applied Cryptography and Security? Take a look at my book [Black Hat Rust](https://kerkour.com/black-hat-rust).**
