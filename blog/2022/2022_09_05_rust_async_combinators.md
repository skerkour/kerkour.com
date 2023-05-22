+++
date = 2022-09-05T06:30:00Z
title = "Learning Rust: Async Combinators"
type = "post"
tags = ["hacking", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-async-combinators"

[extra]
lang = "en"
+++

[Last week, we learned what are and how to use combinators](/rust-combinators). Today, we see how to use Rust's `async` combinators and how they can make your code more functional and clean.


You may be wondering: what combinators have to do with `async`?

Well, the [Future](https://docs.rs/futures/latest/futures/future/trait.Future.html) and the [Stream](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) traits have two friends, the [FutureExt](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html) and the [StreamExt](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html) traits. Those traits add combinators to the `Future` and `Stream` types, respectively.


> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


#### `FutureExt`

[then](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.then) calls a function returning a Future after the initial Future finished:

```rust
async fn compute_a() -> i64 {
    40
}

async fn compute_b(a: i64) -> i64 {
    a + 2
}

let b = compute_a().then(compute_b).await;
// b = 42
```

[map](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.map) converts a Future's output to a different type by calling a **non-async** function:

```rust
async fn get_port() -> String {
    // ...
}

fn parse_port() -> Result<u16, Error> {
    // ...
}

let port: Result<u16, Error> = get_port().map(parse_port).await;
```

[flatten](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.flatten) merges a Future of Future (`Future<Output=Future<Output=String>>` for example) into a simple Future (`Future<Output=String>`).

```rust
let nested_future = async { async { 42 } };

let f = nested_future.flatten();
let forty_two = f.await;
```

[into_stream](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.into_stream) converts a future into a single element stream.

```rust
let f = async { 42 };
let stream = f.into_stream();
```

<!-- [boxed](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.boxed) -->


You can find the other (and in my experience, less commonly used) combinators for the `FutureExt` type online: [https://docs.rs/futures/latest/futures/future/trait.FutureExt.html](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html).



#### `StreamExt`

As we saw, Streams are like async iterators, and this is why you will find the same combinators, such as [filter](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.filter), [fold](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.fold), [for_each](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.for_each), [map](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.map) and so on.

Like Iterators, Streams **should be consumed** to have any effect.


Additionally, there are some specific combinators that can be used to process elements concurrently:
[for_each_concurrent](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.for_each_concurrent) and [buffer_unordered](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.buffer_unordered).

As you will notice, the difference between the two is that `buffer_unordered` produces a Stream that needs to be consumed while `for_each_concurrent` actually consumes the Stream.

Here is a quick example:

**[ch_03/snippets/concurrent_stream/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/concurrent_stream/src/main.rs)**
```rust
use futures::{stream, StreamExt};
use rand::{thread_rng, Rng};
use std::time::Duration;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    stream::iter(0..200u64)
        .for_each_concurrent(20, |number| async move {
            let mut rng = thread_rng();
            let sleep_ms: u64 = rng.gen_range(0..20);
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
            println!("{}", number);
        })
        .await;
}
```

```bash
$ cargo run --release
14
17
18
13
9
2
5
8
16
19
3
4
10
29
0
7
20
15
...
```

The lack of order of the printed numbers shows us that jobs are executed concurrently.

In `async` Rust, Streams and their concurrent combinators replace worker pools in other languages. Worker pools are commonly used to process jobs concurrently, such as HTTP requests, file hashing, and so on. But in Rust, they are an anti-pattern because their APIs often favor imperative programming, mutable variables (to accumulate the result of computation) and thus may introduce subtle bugs.

Indeed, the most common challenge of a worker pool is to collect back the result of the computation applied to the jobs.

There are 3 ways to use Streams to replace worker pools and collect the result in an idiomatic and functional way. Remember to **always put an upper limit on the number of concurrent tasks. Otherwise, you may quickly exhaust the resources of your system and thus affect performance**.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


##### Using `buffer_unordered` and `collect`

Remember `collect`? It can also be used on Streams to convert them to a collection.

**[ch_03/tricoder/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/tricoder/src/main.rs)**
```rust
// Concurrent stream method 1: Using buffer_unordered + collect
let subdomains: Vec<Subdomain> = stream::iter(subdomains.into_iter())
    .map(|subdomain| ports::scan_ports(ports_concurrency, subdomain))
    .buffer_unordered(subdomains_concurrency)
    .collect()
    .await;
```

This is the more functional and idiomatic way to implement a worker pool in Rust. Here, our `subdomains` is the list of jobs to process. It's then transformed into Futures holding port scanning tasks. Those Futures are concurrently executed thanks to `buffer_unordered`. And the Stream is finally converted back to a `Vec` with `.collect().await`.


##### Using an `Arc<Mutex<T>>`

**[ch_03/tricoder/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/tricoder/src/main.rs)**
```rust
// Concurrent stream method 2: Using an Arc<Mutex<T>>
let res: Arc<Mutex<Vec<Subdomain>>> = Arc::new(Mutex::new(Vec::new()));

stream::iter(subdomains.into_iter())
    .for_each_concurrent(subdomains_concurrency, |subdomain| {
        let res = res.clone();
        async move {
            let subdomain = ports::scan_ports(ports_concurrency, subdomain).await;
            res.lock().await.push(subdomain)
        }
    })
    .await;
```



##### Using channels

**[ch_03/tricoder/src/ports.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/tricoder/src/ports.rs)**
```rust
// Concurrent stream method 3: using channels
let (input_tx, input_rx) = mpsc::channel(concurrency);
let (output_tx, output_rx) = mpsc::channel(concurrency);

tokio::spawn(async move {
    for port in MOST_COMMON_PORTS_100 {
        let _ = input_tx.send(*port).await;
    }
});

let input_rx_stream = tokio_stream::wrappers::ReceiverStream::new(input_rx);
input_rx_stream
    .for_each_concurrent(concurrency, |port| {
        let subdomain = subdomain.clone();
        let output_tx = output_tx.clone();
        async move {
            let port = scan_port(&subdomain.domain, port).await;
            if port.is_open {
                let _ = output_tx.send(port).await;
            }
        }
    })
    .await;

// close channel
drop(output_tx);

let output_rx_stream = tokio_stream::wrappers::ReceiverStream::new(output_rx);
let open_ports: Vec<Port> = output_rx_stream.collect().await;
```

Here we voluntarily complexified the example as the two channels (one for queuing jobs in the Stream, one for collecting results) are not necessarily required.

One interesting thing to notice, is the use of a generator:
```rust
tokio::spawn(async move {
    for port in MOST_COMMON_PORTS_100 {
        let _ = input_tx.send(*port).await;
    }
});
```

Why? Because as you don't want unbounded concurrency, you don't want unbounded channels, it may put down your system under pressure. But if the channel is bounded and the downstream system processes jobs slower than the generator, it may block the latter and cause strange issues. This is why we spawn the generator in its own tokio task, so it can live its life in complete independence.

