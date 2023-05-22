+++
date = 2022-04-19T02:00:00Z
title = "Building a crawler in Rust: Synchronization (Atomic Types and Barriers)"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-crawler-synchronization-atomic-types-barrier"

[extra]
lang = "en"

comment ="""
"""
+++

[Last week we saw how to design a crawler](https://kerkour.com/rust-crawler-associated-types). Today we are going to see implementation details: how to use Rust's synchrnoization primitives to make our crawler as efficient as possible.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)

**Building a crawler in Rust**:
* [Building a crawler in Rust: Design and Associated Types](https://kerkour.com/rust-crawler-associated-types)
* **[Building a crawler in Rust: Synchronization (Atomic Types and Barriers)](https://kerkour.com/rust-crawler-synchronization-atomic-types-barrier)**
* [Building a crawler in Rust: Implementing the crawler](https://kerkour.com/rust-crawler-implementation)
* [Building a crawler in Rust: Scraping and Parsing HTML](https://kerkour.com/rust-crawler-scraping-and-parsing-html)
* [Building a crawler in Rust: Crawling a JSON API](https://kerkour.com/rust-crawler-json-api)
* [Building a crawler in Rust: Crawling Javascript Single Page Applications (SPA) with a headless browser](https://kerkour.com/rust-crawler-javascript-single-page-application-headless-browser)

## Atomic types

Atomic types, like mutexes, are shared-memory types: they can be safely shared between multiple threads.

They allow not to have to use a mutex, and thus and all the ritual around `lock()` which may introduce bugs such as deadlocks.

You should use an atomic if you want to share a boolean or an integer (such as a counter) across threads instead of a `Mutex<bool>` or `Mutex<i64>`.

Operations on atomic types require an ordering argument. The reason is out of the topic of this book, but you can read more about it on this excellent post: [Explaining Atomics in Rust](https://cfsamsonbooks.gitbook.io/explaining-atomics-in-rust/).

To keep things simple, use [`Ordering::SeqCst`](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.SeqCst) which provides the strongest guarantees.


**[ch_05/snippets/atomic/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/snippets/atomic/src/main.rs)**
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    // creating a new atomic
    let my_atomic = AtomicUsize::new(42);

    // adding 1
    my_atomic.fetch_add(1, Ordering::SeqCst);

    // geting the value
    assert!(my_atomic.load(Ordering::SeqCst) == 43);

    // substracting 1
    my_atomic.fetch_sub(1, Ordering::SeqCst);

    // replacing the value
    my_atomic.store(10, Ordering::SeqCst);
    assert!(my_atomic.load(Ordering::SeqCst) == 10);

    // other avalable operations
    // fetch_xor, fetch_or, fetch_nand, fetch_and...

    // creating a new atomic that can be shared between threads
    let my_arc_atomic = Arc::new(AtomicUsize::new(4));

    let second_ref_atomic = my_arc_atomic.clone();
    thread::spawn(move|| {
        second_ref_atomic.store(42, Ordering::SeqCst);
    });
}
```

The available types are:

- `AtomicBool`
- `AtomicI8`
- `AtomicI16`
- `AtomicI32`
- `AtomicI64`
- `AtomicIsize`
- `AtomicPtr`
- `AtomicU8`
- `AtomicU16`
- `AtomicU32`
- `AtomicU64`
- `AtomicUsize`

You can learn more about atomic type in the [Rust doc](https://doc.rust-lang.org/std/sync/atomic/).


## Barrier

A barrier is like a `sync.WaitGroup` in Go: it allows multiples concurrent operations to synchronize.


```rust
use tokio::sync::Barrier;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // number of concurrent operations
    let barrier = Arc::new(Barrier::new(3));

    let b2 = barrier.clone()
    tokio::spawn(async move {
        // do things
        b2.wait().await;
    });

     let b3 = barrier.clone()
    tokio::spawn(async move {
        // do things
        b3.wait().await;
    });

    barrier.wait().await;

    println!("This will print only when all the three concurrent operations have terminated");
}
```


**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build a crawler in Rust to scrape vulnerabilities and gather data about our targets.**
