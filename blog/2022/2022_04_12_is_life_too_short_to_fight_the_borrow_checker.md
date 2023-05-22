+++
date = 2022-04-12T02:00:00Z
title = "Is life too short to fight Rust's borrow checker?"
type = "post"
tags = ["programming", "rust"]
authors = ["Sylvain Kerkour"]
url = "/life-is-short-rust-borrow-checker"

[extra]
lang = "en"

comment ="""
"""
+++

While solving a not-so-easy lifetime problem in Rust related to async closures, I came to think:

Is life too short to fight the borrow checker?

I mean, don't get me wrong, I love solving problems and the beauty of efficient and correct code, but sometimes, too much is too much.

*[Rust's price for improved control is the curse of choice:](https://matklad.github.io/2020/09/20/why-not-rust.html)*
```rust
struct Foo {
  bar: Bar
}

struct Foo<'a> {
  bar: &'a Bar
}

struct Foo<'a> {
  bar: &'a mut Bar
}

struct Foo {
  bar: Box<Bar>
}

struct Foo {
  bar: Rc<Bar>
}

struct Foo {
  bar: Arc<Bar>
}
```

While in most languages, we would simply:
```go
type Foor struct {
  bar Bar
}
```

call it a day, finish work early and go to the beach for sunset.

So I'm asking you: Is life too short to fight the borrow checker?


![Sunset](/2022/life-is-short-rust-borrow-checker/sunset.jpg)
