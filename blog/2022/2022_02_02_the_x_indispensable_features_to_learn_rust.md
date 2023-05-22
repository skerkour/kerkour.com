+++
date = 2022-02-02T10:00:00Z
title = "The 9 indispensable features to learn for the new Rust programmer"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/indispensable-rust-features-to-learn"

[extra]
lang = "en"

comment ="""


the X indispenaable features to learn rust


ce qui veut dire que lea autres ne sont pas indispensablea, et vous pouvez commencer a coder en Rust sans les connaitre


ready to dive ?

take alook at my book
"""
+++

Rust is a rather large and complex programming language with a lot of features. But I have good news: less than [20% of the features will bring you more than 80% of the results](https://en.wikipedia.org/wiki/Pareto_principle).

Here are the features I consider indispensable to learn when you are starting Rust.

Ready to dive? 🦀

## Enums

Enums (also called [algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type)) are certainly the favorite feature of new Rustaceans because they are the foundations of `Result` and `Option`.

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}

pub enum Option<T> {
    None,
    Some(T),
}
```

Enums allow developers to safely encode into code all the possible states of their programs and check at compile time that they didn't forget a case:

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
            // Compile time error! We forgot Windows and Unknown
        }
    }
}
```

## async-await

Threads were designed to parallelize compute-intensive tasks. However, these days, a lot of applications (such as a network scanner or a web server) are I/O (Input / Output) intensive which means that by using threads, our apps would spend a lot of time waiting for network requests to complete and use way more resources than necessary.

These are the problems solved by [`async-await`](https://rust-lang.github.io/async-book/), all while providing a great developer experience.


You can learn more about how `async-await` works in my previous posts: [Async Rust: Cooperative vs Preemptive scheduling](/cooperative-vs-preemptive-scheduling/) and [Async Rust: What is a runtime? Here is how tokio works under the hood](/rust-async-await-what-is-a-runtime/).


## Traits

You may need to switch concrete implementations of multiple similar types sharing the same behavior.

For example, a storage driver:

```rust
struct FilesystemStorage {
  get() // ...
  put() // ...
  delete() // ...
}

struct S3Storage {
  get() // ...
  put() // ...
  delete() // ...
}
```

For that, we use [traits](https://doc.rust-lang.org/book/ch10-02-traits.html), also called interfaces in other languages.


```rust
trait Storage {
  get() // ...
  put() // ...
  delete() // ...
}

impl Storage for FilesystemStorage {
  // ...
}

impl Storage for S3Storage {
  // ...
}


fn use_storage<S: Storage>(storage: S) {
  // ...
}
```


## Smart pointers

I've [already extensively covered smart pointer on this blog](/rust-avoid-lifetimes). In short, they allow developers to avoid lifetime annotations and thus write cleaner code.

They also are the foundations of [traits obejcts](https://doc.rust-lang.org/reference/types/trait-object.html) which allow you to pick the right implementation at runtime (instead of compile-time with generics).

```rust
struct MyService {
  db: Arc<DB>,
  mailer: Arc<dyn drivers::Mailer>,
  storage: Arc<dyn drivers::Storage>,
  other_service: Arc<other::Service>,
}
```

## Collections

[Rust's standard library's collections](https://doc.rust-lang.org/std/collections/index.html) are what make writing complex algorithms and business logic in Rust is so pleasant.

```rust
let dedup_subdomains: HashSet<String> = subdomains.into_iter().collect();
```



## Iterators

An [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) is an object that enables developers to traverse collections. They can be obtained from most of the collections of the standard library.

```rust
fn filter() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();

    let _positive_numbers: Vec<i32> = v.filter(|x: &i32| x.is_positive()).collect();
}
```

Iterators are lazy: they won't do anything if they are not consumed.



## Combinators

Combinators are a very interesting topic. Almost all the definitions you'll find on the internet will make your head explode 🤯 because they raise more questions than they answer.

Thus, here is my empiric definition: Combinators are methods that ease the manipulation of some type `T`. They favor a functional (method chaining) style of code.

```rust
let sum: u64 = vec![1, 2, 3].into_iter().map(|x| x * x).sum();
```

Here are a more examples:


```rust
// Convert a `Result` to an `Option`
fn result_ok() {
    let _port: Option<String> = std::env::var("PORT").ok();
}

// Use a default `Result` if `Result` is `Err`
fn result_or() {
    let _port: Result<String, std::env::VarError> =
        std::env::var("PORT").or(Ok(String::from("8080")));
}

// Use a default value if empty, then apply a function
let http_port = std::env::var("PORT")
    .map_or(Ok(String::from("8080")), |env_val| env_val.parse::<u16>())?;

// Chain a function if `Result` is `Ok` or a different function if `Result` is `Err`
let master_key = std::env::var("MASTER_KEY")
    .map_err(|_| env_not_found("MASTER_KEY"))
    .map(base64::decode)??;
```



## Streams

[Streams](https://doc.rust-lang.org/std/stream/index.html) can be roughly defined as iterators for the `async` world.

You should use them when you want to apply asynchronous operations on a sequence of items of the same type, whether it be a network socket, a file, or a long-lived HTTP request.

Anything that is too large to fit in memory and thus should be split in smaller chunks, or that may arrive later, but we don't know when, or that is simply a collection (a `Vec` or an `HashMap` for example) to which we need to apply `async` operations to.

They also allow us to easily execute operations concurrently:

```rust
async fn compute_job(job: i64) -> i64 {
  // ...
}

#[tokio::main]
async fn main() {
    let jobs = 0..100;
    let concurrency = 42;

    stream::iter(jobs)
        .for_each_concurrent(concurrency, |job| compute_job(job)).await;
}
```


You can learn more about using streams as worker pools in my previous post: [How to implement worker pools in Rust](/rust-worker-pool/)


## no_std

Finally, Rust is very well suited for [embedded development](/rust-on-esp32/) and [shellcodes](/shellcode-in-rust/). Because these environments don't rely on a proper Operating System, you generally can't use Rust's standard library and you need to use the [core](https://doc.rust-lang.org/core/) library instead.

For these usecases, we use the `#![no_std]` attribute:

```rust
#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn _start() {
  // ...
}
```
