+++
date = 2022-01-25T06:00:00Z
title = "Async Rust: Cooperative vs Preemptive scheduling"
type = "post"
tags = ["rust", "programming", "tutorial", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/cooperative-vs-preemptive-scheduling"

[extra]
lang = "en"

comment ="""

title = "async-await: Cooperative vs Preemptive scheduling"


dire que async-std est preemtive (a verifier)


*This is part of my book "Black Hat Rust" available in early-access. Here is a coupon to save 10€ on the book: [https://academy.kerkour.com/black-hat-rust?coupon=BLOG](https://academy.kerkour.com/black-hat-rust?coupon=BLOG).*



Imagine you arrive at a restaurant with only one cook and 9 tables are occupied. Will you wait for the 9 other tables to finish their full meal (starter, main dish, dessert) before being able to order? Of course not!

The cook can fry a dish, at the same time he prepares another plate, all while a third plate is cooling after a passage in the oven. The cook is multitasking.

And this is precisely the point of *async-await*: **multitasking**.

## Concurrency

Concurrency is defined by [Clay Breshears](https://www.oreilly.com/library/view/the-art-of/9780596802424/) as:
> A system is said to be concurrent if it can support two or more tasks in progress at the same time. A system is said to be parallel if it can support two or more tasks executing simultaneously.

In the example above, the tasks are frying, cooling, preparing... The single cook is concurrent, and the kitchen can be parallelized by adding more cooks.


The nice thing is that these two characteristics are not mutually exclusive: in order to squeeze the last bits of performances, it's possible to set up multiple concurrent systems in parallel. For example, we could add more concurrent cooks in our kitchen to increase the number of plates that can be served per minute.


## Runtimes

A runtime (also known as a scheduler) is a pre-made piece of code that allows systems to be concurrent. Its job is to spawn and schedule tasks. A runtime can be single-threaded or multithreaded, which means that tasks are either executed on a single thread or on multiple.

## Preemptive scheduling

When you write Go code, you don't even notice the presence of the scheduler. You write your code as if all your operation are equal, and you don't worry about an operation blocking your system or the other operations happening at the same time in your other goroutines.


```go
func DownloadFile(url string, filepath string) error {
	// Create the file
	file, err := os.Create(filepath)
	if err != nil {
		return err
	}
	defer file.Close()

	// Get the data
	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	// Write the body to file
	_, err = io.Copy(file, resp.Body)
	if err != nil {
		return err
	}

	return nil
}
```

The scheduler detects by itself blocking operations and will schedule other tasks.


## Cooperative scheduling

A cooperative scheduler, on the other hand, doesn't take initiatives by itself. It relies on the programmer to indicate to it when it's good to switch to another task. This is the reason to exist of the `async` and `await` keywords in Rust: to allow the programmer to tell the compiler and the runtime that it's time to switch to another task because the current task will take some time to finish.

A good rule to remember is no more than [10 to 100 microseconds](https://ryhl.io/blog/async-what-is-blocking/) between two `.await` calls. Otherwise, the scheduler is blocked, and the performance of your system may greatly suffer.

## Enter tokio

`tokio` is certainly Rust's most famous and mature runtime and powers extremely high-throughput applications such as some parts of AWS or OneSignal.


## Blocking tasks

What if I want to run a compute-intensive task in an async application?

### spawn_blocking


### Threadpool

This is, for example, the approach taken by the `actix-web` framework, where each `web::block` calls dispatch the task to a [threadpool](https://docs.rs/actix-web/3.3.2/src/actix_web/web.rs.html#284).


## Other Rust runtimes

### Async-std

[`async-std`'s runtime](https://async.rs/blog/stop-worrying-about-blocking-the-new-async-std-runtime/) is inspired by Go's one and thus automatically detect operations that take too much time to perform and spawn them in a different thread. So even if the Rust language is cooperative, the runtime is (kind of) preemptive.


### smoll


## The future of async-await

### Async traits

[Async fn in traits are hard](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/) and this is why it's not possible to do the following today in stable Rust:
```rust
trait UserRepository {
    async fn find_user(&self) -> User;
}
```

As of now, the temporary workaround is to use the [`async-trait`](https://github.com/dtolnay/async-trait) crate:
```rust
#[async_trait]
trait UserRepository {
    async fn find_user(&self) -> User;
}
```

### Stream

The [Stream](https://doc.rust-lang.org/nightly/std/stream/trait.Stream.html) trait has not yet reached the [stable standard library](https://github.com/rust-lang/rust/issues/79024), but is available today in the [`futures`](https://docs.rs/futures/) crate.


### Interchangeable runtimes

A problem today is that runtime can't be easily interchanged because they don't rely on compatible traits. It means that it's not easy to run a web server designed for the `tokio` runtime into the `async-std` runtime.

So if we don't want to perpetuate the current fragmentation of the ecosystem, a good direction is to make all runtimes compatible.


*This is part of my book "Black Hat Rust" available in early-access. Here is a coupon to save 10€ on the book: [https://academy.kerkour.com/black-hat-rust?coupon=BLOG](https://academy.kerkour.com/black-hat-rust?coupon=BLOG).*




Imagine you arrive at a restaurant with only one cook and 9 tables are occupied. Will you wait the 9 other tables to finish their full meal (starter, main dish, dessert) before being able to order? Of course not!

The cook can fry a dish, at the same time he prepares another plate, all while a third plate is cooling after a passage in the oven. The cook is multitasking.

And this is exactly the point of async-await: **multitasking**.

## Concurrency

Concurrency is defined by [Clay Breshears](https://www.oreilly.com/library/view/the-art-of/9780596802424/) as:
> A system is said to be concurrent if it can support two or more tasks in progress at the same time. A system is said to be parallel if it can support two or more tasks executing simultaneously.

In the example above, the tasks are frying, cooling, preparing... The single cook is concurrent, and the kitchen can be parallelized by adding more cooks.


The nice thing is that these two caracteristics are not mutually exclusive: in order to squeeze the last bits of performances it's possible to setup multiple concurrent systems in parallel. For example, we could add more concurrent cooks in our kitchen to increase the number of plates that can be served per minute.


## Runtimes

A runtime (also known as a scheduler) is a pre-made piece of code that allows systems to be concurrent. Its job is to spawn and schedule tasks. A runtime can be single-threaded, or multithreaded, which means that tasks are either executed on a single thread, or on multiple.


## Preemptive scheduling

When you write Go code you don't even notice the presence of the scheduler. You write your code as if all your operation are equal and you don't worry about an operation blocking your system or the other operations happening at the same time in your other goroutines.


```go
func DownloadFile(url string, filepath string) error {
	// Create the file
	file, err := os.Create(filepath)
	if err != nil {
		return err
	}
	defer file.Close()

	// Get the data
	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	// Write the body to file
	_, err = io.Copy(file, resp.Body)
	if err != nil {
		return err
	}

	return nil
}
```

It's because the scheduler detects by itself blocking operations and will schedule other tasks


## Cooperative scheduling

A cooperative scheduler, on the other hand, don't take initiatives by itself. It relies on the programmer to indicate to it when it's good to switch to another task. This is the reason to exist of the `async` and `await` keywords in Rust: to allow the programmer to tell the compiler and the runtime that it's time to switch to another task because the current task will take some time to finish.

A good rule to remember, is no more than [10 to 100 microseconds](https://ryhl.io/blog/async-what-is-blocking/) between two `.await` calls. Otherwise the scheduler is blocked and the performance of your system may greatly suffer.

## Enter tokio

`tokio` is certainly Rust's most famous and mature runtime and powers extremely high-trougput applications such as some parts of AWS or OneSignal.


## Blocking tasks

What if I want to run a compute intenseive task in an async application?

### spawn_blocking


### Threadpool

This is for example the approach taken by the `actix-web` framework, where each `web::block` calls dispatch the task to a [threadpool](https://docs.rs/actix-web/3.3.2/src/actix_web/web.rs.html#284).



## Other Rust runtimes

### Async-std

[`async-std`'s runtime](https://async.rs/blog/stop-worrying-about-blocking-the-new-async-std-runtime/) is inspired by Go's one and thus automatically detect operations that take too much time to perform, and spawn them in a different thread. So even if the Rust language is cooperative, the runtime is (kind ok) preemptive.


### smoll


## The future of async-await

### Async traits

[Async fn in traits are hard](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/) and this is why it's not possible to do the following today in stable Rust:
```rust
trait UserRepository {
    async fn find_user(&self) -> User;
}
```

As of now, the temporary workaround is to use the [`async-trait`](https://github.com/dtolnay/async-trait) crate:
```rust
#[async_trait]
trait UserRepository {
    async fn find_user(&self) -> User;
}
```

### Stream

The [Stream](https://doc.rust-lang.org/nightly/std/stream/trait.Stream.html) traitd has not yet reached the [stable standard library](https://github.com/rust-lang/rust/issues/79024), but is available today in the [`futures`](https://docs.rs/futures/) crate.

### Interchangeable runtimes

A problem today, is that runtime can't be easily interchanged, because they don't rely on compatible traits. It means that it's not easy to run a web server designed for the `tokio` runtime into the `async-std` runtime.

So if we don't want to perpetuate the current fragmentation of the ecosystem, a good direction is make all runtimes compatible.


*This is part of my book "Black Hat Rust" available in early access. Here is a coupon to save 10€ on the book: [https://academy.kerkour.com/black-hat-rust?coupon=BLOG](https://academy.kerkour.com/black-hat-rust?coupon=BLOG).*

"""
+++


Threads were designed to parallelize compute-intensive tasks. However, these days, a lot of applications (such as a network scanner) are I/O (Input / Output) intensive.

Thus, threads have two significant problems:

* They use a *lot* of memory (compared to others solutions).
* Launches and context switches have a cost that can be felt when a lot (in the ten of thousands) threads are running.


In practice, it means that by using threads, our apps would spend a lot of time waiting for network requests to complete and use way more resources than necessary.


Please welcome `async-await`.

> This post contains excerpts from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)

## The problem with Threads

From a programmer's perspective, `async`/`await` provides the same things as threads: concurrency, better hardware utilization, improved speed, but with dramatically better performance and lower resource usage for I/O bound workloads.

What is an *I/O bound workload*? Those are tasks that spend most of their time waiting for network or disk operations to complete instead of being limited by the computing power of the processor.

Threads were designed a long time ago, when most of the computing was not network (web) related stuff, and thus are not suitable for too many concurrent I/O tasks.

| operation | async | thread |
| ----- | ------ | ----- |
| Creation | 0.3 microseconds | 17 microseconds |
| Context switch | 0.2 microseconds | 1.7 microseconds |

As we can see with these measurements [made by Jim Blandy](https://github.com/jimblandy/context-switch), context switching is roughly 8.5 times faster with async than with Linux threads and use approximately 20 times less memory.


In the programming language world, there are mainly 2 ways to deal with I/O tasks: **preemptive scheduling** and **cooperative scheduling**.

## Preemptive Scheduling


**Preemptive scheduling** is when the scheduling of the tasks is out of the control of the developer, entirely managed by a **runtime**. Whether the programmer is launching a sync or an async task, there is no difference in the code.

For example, the [Go](https://golang.org) programming relies on preemptive scheduling.

It has the advantage of being easier to learn: for the developers, there is no difference between sync and async code. Also, it is almost impossible to misuse: the runtime takes care of everything.

Here is an example of making an HTTP request in Go:
```go
resp, err := http.Get("https://kerkour.com")
```

Just by looking at this snippet, we can't tell if `http.Get` is I/O intensive or compute intensive.


The disadvantages are:

* Speed, which is limited by the cleverness of the runtime.
* Hard to debug bugs: If the runtime has a bug, it may be extremely hard to find it out, as the runtime is treated as dark magic by developers.


## Cooperative Scheduling

On the other hand, with **cooperative scheduling**, the developer is responsible for telling the runtime when a task is expected to spend some time waiting for I/O. Waiting, you said? Yes, you get it. It's the exact purpose of the `await` keyword. It's an indication for the runtime (and compiler) that the task will take some time waiting for an operation to complete, and thus the computing resources can be used for another task in the meantime.

It has the advantage of being **extremely fast**. Basically, the developer and the runtime are working together, in harmony, to make the most of the computing power at disposition.

The principal disadvantage of cooperative scheduling is that it's easier to misuse: if a `await` is forgotten (fortunately, the Rust compiler issues warnings), or if the event loop is blocked (what is an event loop? continue reading to learn about it) for more than a few micro-seconds, it can have a disastrous impact on the performance of the system.

The corollary is that an `async` program should deal with extreme care with compute-intensive operations.

Here is an example of making an HTTP request in Rust:
```rust
let res = reqwest::get("https://www.rust-lang.org").await?;
```

The `.await` keyword tells us that the `reqwest::get` function is expected to take some time to complete.



## Runtimes

What is a runtime? How do they work under the hood? **Subscribe below** not to miss next week's post, where we will dig the inner working of runtimes.


**Want to learn more about Rust and Security? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust).**
