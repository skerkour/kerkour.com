+++
date = 2022-09-27T06:30:00Z
title = "Multithreading in Rust"
type = "post"
tags = ["hacking", "rust", "programming"]
authors = ["Sylvain Kerkour"]
url = "/multithreading-in-rust"

[extra]
lang = "en"
+++

In this post, we are going to speed up a [port scanner](https://en.wikipedia.org/wiki/Port_scanner) in Rust by using multiple threads instead of only one and see how easily it can be achieved thanks to Rust's type system.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


Once you have discovered which servers are publicly available, you need to find out what services are publicly available on these servers.

Scanning ports is the topic of entire books. Depending on what you want: be more stealthy, be faster, have more reliable results, and so on.

There are a lot of different techniques, so in order not to skyrocket the complexity of our program, we will use the simplest technique: trying to open a TCP socket. This technique is known as *TCP connect* because it consists of trying to establish a connection to a TCP port.

A socket is kind of an internet pipe. For example, when you want to connect to a website, your browser opens a socket to the website's server, and then all the data passes through this socket. When a socket is open, it means that the server is ready to accept connections. On the other hand, if the server refuses to accept the connections, it means that no service is listening on the given port.

In this situation, it's important to use a timeout. Otherwise, our scanner can be stuck (almost) indefinitely when scanning ports blocked by firewalls.

**[ch_02/tricoder/src/ports.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_02/tricoder/src/ports.rs)**
```rust
use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, Subdomain},
};
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};
use rayon::prelude::*;


pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.len() == 0 {
        return subdomain;
    }

    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_iter()
        .map(|port| scan_port(socket_addresses[0], *port))
        .filter(|port| port.is_open) // filter closed ports
        .collect();
    subdomain
}


fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    socket_address.set_port(port);

    let is_open = TcpStream::connect_timeout(&socket_address, timeout).is_ok();

    Port {
        port: port,
        is_open,
    }
}
```

But we have a problem. Firing all our requests in a sequential way is extremely slow: if all ports are closed, we are going to wait `Number_of_scanned_ports * timeout` seconds.



## Multithreading

Fortunately for us, there exists an API to speed-up programs: threads.

Threads are primitives provided by the Operating System (OS) that enable programmers to use the hardware cores and threads of the CPU. In Rust, a thread can be started using the `std::thread::spawn` function.

![Single vs Multi threaded](https://kerkour.com/black-hat-rust/assets/ch02_single_vs_multi_threaded.png)

Each CPU thread can be seen as an independent worker: the workload can be split among the workers.

This is especially important as today, due to the law of physics, processors have a hard time scaling up in terms of operations per second (GHz). Instead, vendors increase the number of cores and threads. Developers have to adapt and design their programs to split the workload between the available threads instead of trying to do all the operations on a single thread, as they may sooner or later reach the limit of the processor.

With threads, we can split a big task into smaller sub-tasks that can be executed in parallel.

In our situation, we will dispatch a task per port to scan. Thus, if we have 100 ports to scan, we will create 100 tasks.

Instead of running all those tasks in sequence like we previously did, we are going to run them on multiple threads.

If we have 10 threads, with a 3 seconds timeout, it may take up to 30 seconds (`10 * 3`) to scan all the ports for a single host. If we increase this number to 100 threads, then we will be able to scan 100 ports in only 3 seconds.


> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)

## Fearless concurrency in Rust

Before going further, I recommend you to read [my other post about how Rust's ownership system prevents data races](https://kerkour.com/rust-fearless-concurrency) and allows easier and safer concurrency.

**TL;DR**:

### The three causes of data races

* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There's no mechanism being used to synchronize access to the data

### The three rules of ownership

* Each value in Rust has a variable that's called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

### The two rules of references

* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.


These rules are **extremely** important and are the foundations of Rust's memory and concurrency safety.

If you need more details about ownership, take some time to read the [dedicated chapter online](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).



## Adding multithreading to our scanner

Now we have seen what multithreading is in theory. Let's see how to do it in idiomatic Rust.

Usually, multithreading is dreaded by developers because of the high probability of introducing the bugs we have just seen.

But in Rust this is another story. Other than for launching long-running background jobs or workers, it's **rare to directly use the thread API from the standard library**.

Instead, we use [rayon](https://github.com/rayon-rs/rayon), *a data-parallelism library for Rust*.

Why a data-parallelism library? Because thread synchronization is hard. It's better to design our programs in a functional way that doesn't require threads to be synchronized.


**[ch_02/tricoder/src/ports.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_02/tricoder/src/ports.rs)**
```rust
// ...
use rayon::prelude::*;

fn main() -> Result<()> {
    // ...
    // we use a custom threadpool to improve speed
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    // pool.install is required to use our custom threadpool, instead of rayon's default one
    pool.install(|| {
        // ...
    });
    // ...
}

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.len() == 0 {
        return subdomain;
    }

    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter() // <- HERE IS THE IMPORTANT BIT
        .map(|port| scan_port(socket_addresses[0], *port))
        .filter(|port| port.is_open) // filter closed ports
        .collect();
    subdomain
}
```


Aaaand... That's all. Really. We replaced `into_iter()` by `into_par_iter()` (which means "into parallel iterator". What is an iterator? More on that in chapter 3), and now our scanner will scan all the different subdomains on dedicated threads.


### Behind the scenes

This two-lines change hides a lot of things. That's the power of Rust's type system.


#### Prelude

```rust
use rayon::prelude::*;
```

The use of `crate::prelude::*` is a common pattern in Rust when crates have a lot of important traits or structs and want to ease their import.

In the case of `rayon`, as of version `1.5.0`, `use rayon::prelude::*;` is the equivalent of:
```rust
use rayon::iter::FromParallelIterator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelDrainFull;
use rayon::iter::ParallelDrainRange;
use rayon::iter::ParallelExtend;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;
use rayon::slice::ParallelSliceMut;
use rayon::str::ParallelString;
```


#### Threadpool

In the background, the `rayon` crate started a thread pool and dispatched our tasks `scan_ports` and `scan_port` to it.

The nice thing with `rayon` is that the thread pool is hidden from us, and the library encourages us to design algorithms where data is not shared between tasks (and thus threads). Also, the parallel iterator has the same methods available as traditional iterators.



## Alternatives

Another commonly used crate for multithreading is [`threadpool`](https://docs.rs/threadpool) but it is a little bit lower level as we have to build the thread pool and dispatch the tasks ourselves. Here is an example:

**[ch_02/snippets/threadpool/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_02/snippets/threadpool/src/main.rs)**
```rust
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

fn main() {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(1).expect("sending data back from the threadpool");
        });
    }

    println!("result: {}", rx.iter().take(n_jobs).fold(0, |a, b| a + b));
}
```

If you don't have a very specific requirement, I don't recommend you to use this crate. Instead, favor `rayon`'s functional programming way.


Indeed, by using `threadpool` instead of `rayon` **you are responsible** for the synchronization and communication between your threads which is the source of a lot of bugs.

It can be achieved by using a `channel` like in the example above where we "share memory by communicating".

Or with a [`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) which allow us to "communicate by sharing memory". A Mutex combined with an [`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) smart pointer allow us to share memory (variables) between threads.


## Async-Await

Before leaving you, I want to tell you a secret.

I didn't tell you the whole story: multithreading is not the only way to increase a program's speed, especially in our case, where most of the time is spent doing I/O operations (TCP connections).

Please welcome `async-await`.

Threads have problems: they were designed to parallelize compute-intensive tasks. However, our current use-case is I/O (Input / Output) intensive: our scanner launches a lot of network requests and doesn't actually compute much.

In our situation, it means that threads have two significant problems:

* They use a *lot* (compared to others solutions) of memory
* Launches and context switches have a cost that can be felt when a lot (in the ten of thousands) threads are running.


In practice, it means that our scanner will spend a lot of time waiting for network requests to complete and use way more resources than necessary.

How to use `async-await` instead of threads? Let's find out in Chapter 3.

**Want to learn more about Rust, applied Cryptography and Security? Take a look at my book [Black Hat Rust](https://kerkour.com/black-hat-rust).**
