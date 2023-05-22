+++
date = 2022-02-08T06:00:00Z
title = "The simplest guide to error handling in Rust"
type = "post"
tags = ["rust", "programming", "tutorial", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/rust-error-handling"

[extra]
lang = "en"

comment ="""
"""
+++

Rust is loved for its reliability, and a good chunk of its reliability comes from its error handling ergonomics.

I know that there already are a few guides about error handling in Rust, but I found these guides to be too long and not straight to the point.

So here is the simplest and most straightforward guide to learn how to handle errors in Rust. The guide I would have loved to have if I started Rust today.

## Overview

There are 2 types of errors in Rust:
- Non-recoverable errors (e.g., non-checked out of bounds array access)
- Recoverable errors (e.g., function failed)


## Non-recoverable errors

For errors that can't be handled and would bring your program into an unrecoverable state, we use the [panic!](https://doc.rust-lang.org/std/macro.panic.html) macro.

```rust
fn encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
  if key.len() != 32 {
    panic!("encrypt: key length is invalid");
  }
  // ...
}
```

An alternative way to trigger a `panic` is to use the [assert!](https://doc.rust-lang.org/std/macro.assert.html) macro.

```rust
fn encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
  assert!(key.len() == 32, "encrypt: key length is invalid");
  // ...
}
```

That being said, handling errors in Rust is very ergonomic, so I see no good reason to ever intentionally `panic`.

## Recoverable errors

Errors that are meant to be handled are returned with the [Result](https://doc.rust-lang.org/std/result/enum.Result.html) [enum](https://doc.rust-lang.org/std/keyword.enum.html).

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

For example:

```rust
// Here, our error type is `String`
fn ultimate_answer(guess: i64) -> Result<(), String> {
  if guess == 42 {
    return Ok(());
  }
  return Err("Wrong answer".to_string());
}
```

Now, returning a `String` as an error is not really useful. Indeed, the same function may return many different errors, so it becomes harder and harder to handle them with precision:


```rust
fn ultimate_answer(guess: i64) -> Result<(), String> {
  if guess == 42 {
    return Ok(());
  } else if guess > 39 && guess <= 41 {
      return Err("A little bit more".to_string());
  } else if guess <= 45 && guess > 42 {
    return Err("A little bit less".to_string());
  }
  return Err("Wrong answer".to_string());
}
```

Or, the same error can be returned by many different functions:

```rust
fn do_something() -> Result<(), String> {
  // ...
  return Err("Something went wrong".to_string());
}

fn do_something_else() -> Result<(), String> {
  // ...
  return Err("Something went wrong".to_string());
}

fn do_another_thing() -> Result<(), String> {
  // ...
  return Err("Something went wrong".to_string());
}
```

This is where we need to define our own `Error` enum. Usually, we define 1 `Error` enum by crate.

```rust
pub enum Error {
  WrongAnswer,
  More,
  Less,
}

fn ultimate_answer(guess: i64) -> Result<(), Error> {
  if guess == 42 {
    return Ok(());
  } else if guess > 39 && guess <= 41 {
      return Err(Error::More);
  } else if guess <= 45 && guess > 42 {
    return Err(Error::Less);
  }
  return Err(Error::WrongAnswer);
}
```



Then, we may want to standardize the error message for each error case. For this, the community has (currently) settled on the [thiserror](https://crates.io/crates/thiserror) crate.


```rust
#[derive(thiserror::Error)]
pub enum Error {
  #[error("Wrong answer")]
  WrongAnswer,
  #[error("A little bit more")]
  More,
  #[error("A little bit less")]
  Less,
}
```

Thanks to `thiserror::Error`, your `Error` enum now implements the [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html) trait and thus also the [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html) and [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) traits.


Then we can handle a potential error with `match`.

```rust
fn question() -> Result<(), Error> {
  let x = // ...
  match ultimate_answer(x) {
    Ok(_) => // do something
    Err(Error::More) => // do something
    Err(Error::Less) => // do something
    Err(Error::WrongAnswer) => // do something
  }
  // ...
}
```

Or, the most common way to handle errors, forward them with `?`.

```rust
fn question() -> Result<(), Error> {
  let x = // ...
  ultimate_answer(x)?; // if `ultimate_answer` returns an error, `question` stops here and returns the error.
  // ...
}
```

Which is a shortcut for:

```rust
fn question() -> Result<(), Error> {
  let x = // ...
  match ultimate_answer(x) {
    Ok(_) => {},
    Err(err) => return Err(err.into()),
  };
  // ...
}
```


### Error conversion

Your program or library may use many dependencies, each with its own error types, but in order to be able to use `?`, your `Error` type needs to implement the [From](https://doc.rust-lang.org/std/convert/trait.From.html) trait for the error types of your dependencies.

```rust
#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error.")]
    Internal(String),
    #[error("Not found.")]
    NotFound,
    #[error("Permission Denied.")]
    PermissionDenied,
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::Internal(err.to_string()),
        }
    }
}

// ...
```



### Unwrap and Expect


You can panic on recoverable errors with [.unwrap()](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap) and [.expect()](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect) which may be useful when doing exploratory programming, but should be used carefully for programs intended for production. A good practice is to add a comment near each `unwrap` or `expect` to explain why it's safe to use it.

```rust
fn do_something() -> Result<(), Error> {
  // ...
}

fn main() {
  // panic if do_something returns Err(_)
  do_something().unwrap();
}

// or

fn main() {
  // panic if do_something returns Err(_) with the message below
  do_something().expect("do_something returned an error");
}
```


## Returning errors from `main`

Finally, you may want to return errors from your `main` function.

There are two ways to do this:

You can use the [anyhow](https://crates.io/crates/anyhow) which will allow you to return an error of any type implementing the [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html) trait and will display a nicely formatted error message if the program exits with an error.


```rust
#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error.")]
    Internal(String),
    #[error("Not found.")]
    NotFound,
    #[error("Permission Denied.")]
    PermissionDenied,
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

// ...

fn main() -> Result<(), anyhow::Error> {
    my_function()?;

    anothercrate::function_that_return_another_error_type()?;

    Ok(())
}
```


Or, since Rust `1.61`, you can implement the [std::process::Termination](https://doc.rust-lang.org/std/process/trait.Termination.html) for your error type to exit with custom error codes.


```rust
use std::process::{ExitCode, Termination};

pub enum MyError {
    Internal,
    Other,
}

impl Termination for MyError {
    fn report(self) -> ExitCode {
        match self {
          Internal => ExitCode::from(1),
          Other => ExitCode::from(255),
        }
    }
}

// ...

fn main() -> Result<(), MyError> {
    my_function()?;
    Ok(())
}
```
