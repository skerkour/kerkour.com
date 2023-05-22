+++
date = 2022-03-08T02:00:00Z
title = "Functional Programming in Rust"
type = "post"
tags = ["rust", "programming", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/rust-functional-programming"

[extra]
lang = "en"

comment ="""
"""
+++

I've been interested in functional programming since a [friend](https://yanael.io) introduced Haskell to me at University.

<!-- At the time I was doing a lot of JavaScript (Node.js) and did my best to use functionnal patterns in my code which helped a lot  -->

While I never successfully learned Haskell as it's too far from what I was using day to day (C and Node.js), a new language perfectly mixing imperative and functional programming so that even the most object-oriented programmers can introduce functional programming patterns in their code and reduce bugs arrived. This is compounded by its extremely advanced type system, which makes it easy to encode all the invariants in code.

You got it, we are talking about Rust.


> This post contains excerpts from my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where you'll learn Rust, offensive security and cryptography.


## Why

The core premise of functional programming is to reduce bugs by being declarative instead of imperative.

Building blocks are expressions and not statements like in imperative programming languages. For example, everything in [Lisp](https://en.wikipedia.org/wiki/Lisp_(programming_language)) is an expression. On the other hand, most C chunks of code are statements. Rust is primarily an expression language: most chunks of code are producing values.

That's why you can replace
```rust
fn add_one(x: u64) -> u64 {
  return x + 1;
}
```

by

```rust
fn add_one(x: u64) -> u64 {
  x + 1
}
```

The second most important aspect of functional programming is the affection for immutable data.

Third, by being declarative, functional programming is said to be closer to Human (mathematical) reasoning and thus easier to reason about.

This last point can be debated because most programmers learn imperative and Object-Oriented programming during their training, and thus functional programming kind of require them to re-learn how to code.


## Immutability

Rust's variables are immutable by default. You have to be explicit when you want a variable to be mutable.

```rust
fn main() {
    let mut v = Vec::new();

    push_forty_two(&mut v)
}

fn push_forty_two(v: &mut Vec<u64>) {
    v.push(42);
}
```

But

```rust
fn main() {
    let v = Vec::new();

    push_forty_two(&mut v)
}

fn push_forty_two(v: &mut Vec<u64>) {
    v.push(42);
}
```

Does not compile

```console
   Compiling playground v0.0.1 (/playground)
error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
 --> src/main.rs:4:20
  |
2 |     let v = Vec::new();
  |         - help: consider changing this to be mutable: `mut v`
3 |
4 |     push_forty_two(&mut v)
  |                    ^^^^^^ cannot borrow as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `playground` due to previous error
```

## Functions are first-class citizens


Thanks to the [Fn](https://doc.rust-lang.org/stable/std/ops/trait.Fn.html), [FnMut](https://doc.rust-lang.org/stable/std/ops/trait.FnMut.html) and [FnOnce](https://doc.rust-lang.org/stable/std/ops/trait.FnOnce.html) traits which allow us to manipulate functions like any other kind of varaibles.

* *Instances of `Fn` can be called repeatedly without mutating state.*
* *Instances of `FnMut` can be called repeatedly and may mutate state.*
* *Instances of `FnOnce` can be called, but might not be callable multiple times. Because of this, if the only thing known about a type is that it implements `FnOnce`, it can only be called once.*

```rust
fn apply<F: Fn(&str)>(x: &[&str], f: F) {
    for elem in x {
      f(&elem)
    }
}

fn main() {
  let v = vec!["hello", "world"];
  apply(&v, |x| println!("{}", x));
}
```
You can learn more about the topic [in the chapter on closures](https://doc.rust-lang.org/stable/book/ch13-01-closures.html) in the *The Rust Programming Language*.




## Iterators and combinators

An `Iterator` is an object that enables developers to traverse collections.

Iterators can be obtained from most of the collections of the standard library.

First, [`into_iter`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) which provides an owned iterator: the collection is moved, and you can no longer use the original variable.

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn vector() {
    let v = vec![
        1, 2, 3,
    ];

    for x in v.into_iter() {
        println!("{}", x);
    }

    // you can't longer use v
}
```

Then, `iter` which provides a borrowed iterator. Here `key` and `value` variables are references (`&String` in this case).
```rust
fn hashmap() {
    let mut h = HashMap::new();
    h.insert(String::from("Hello"), String::from("World"));

    for (key, value) in h.iter() {
        println!("{}: {}", key, value);
    }
}
```

Since version 1.53 (released on June 17, 2021), iterators can also be obtained from arrays:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn array() {
    let a =[
        1, 2, 3,
    ];

    for x in a.iter() {
        println!("{}", x);
    }
}
```


### Consuming iterators


Iterators are lazy: they won't do anything if they are not consumed.

As we have just seen, Iterators can be consumed with `for x in` loops. But this is not where they are the most used. Idiomatic Rust favors functional programming. It's a better fit for its ownership model.


[for_each](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each) is the functional equivalent of `for .. in ..` loops:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn for_each() {
    let v = vec!["Hello", "World", "!"].into_iter();

    v.for_each(|word| {
        println!("{}", word);
    });
}
```


[collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) can be used to transform an iterator into a collection:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn collect() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into_iter();

    let _: Vec<u64> = x.collect();
}
```


Conversely, you can obtain a `HashMap` (or a `BTreeMap`, or other collections, see [https://doc.rust-lang.org/std/iter/trait.FromIterator.html#implementors](https://doc.rust-lang.org/std/iter/trait.FromIterator.html#implementors), using `from_iter`:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn from_iter() {
    let x = vec![(1, 2), (3, 4), (5, 6)].into_iter();

    let _: HashMap<u64, u64> = HashMap::from_iter(x);
}
```


[reduce](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce) accumulates over an iterator by applying a closure:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn reduce() {
    let values = vec![1, 2, 3, 4, 5].into_iter();

    let _sum = values.reduce(|acc, x| acc + x);
}
```

Here `_sum` = 1 + 2 + 3 + 4 + 5 = 15



[fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) is like `reduce` but can return an accumulator of a different type than the items of the iterator:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn fold() {
    let values = vec!["Hello", "World", "!"].into_iter();

    let _sentence = values.fold(String::new(), |acc, x| acc + x);
}
```

Here `_sentence` is a `String`, while the items of the iterator are of type `&str`.


### Parallel iterators


Usually, multithreading is dreaded by developers because of the high probability of introducing the bugs we have just seen.

But in Rust this is another story. Other than for launching long-running background jobs or workers, it's **rare to directly use the thread API from the standard library**.

Instead, we use [rayon](https://github.com/rayon-rs/rayon), *a data-parallelism library for Rust*.

Why a data-parallelism library? Because thread synchronization is hard. It's better to design our programs in a functional way that doesn't require threads to be synchronized.

**[ch_02/tricoder/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_02/tricoder/src/main.rs)**
```rust
// ...
use rayon::prelude::*;

fn main() -> Result<()> {
    // ...
    let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
        .unwrap()
        .into_par_iter()
        .map(ports::scan_ports)
        .collect();
    // ...
}
```

Aaaand... That's all. Really. We replaced `into_iter()` by `into_par_iter()` (which means "into parallel iterator".

### Combinators


Combinators are a very interesting topic. Almost all the definitions you'll find on the internet will make your head explode 🤯 because they raise more questions than they answer.

Thus, here is my empiric definition: Combinators are methods that ease the manipulation of some type `T`. They favor a functional (method chaining) style of code.

```rust
let sum: u64 = vec![1, 2, 3].into_iter().map(|x| x * x).sum();
```


This section will be pure how-to and real-world patterns about how combinators make your code easier to read or refactor.



First, one of the most famous, and available in almost all languages: [filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter):

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn filter() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();

    let _positive_numbers: Vec<i32> = v.filter(|x: &i32| x.is_positive()).collect();
}
```



[inspect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect) can be used to... inspect the values flowing through an iterator:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn inspect() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();

    let _positive_numbers: Vec<i32> = v
        .inspect(|x| println!("Before filter: {}", x))
        .filter(|x: &i32| x.is_positive())
        .inspect(|x| println!("After filter: {}", x))
        .collect();
}
```


[map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) is used to convert an the items of an iterator from one type to another:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn map() {
    let v = vec!["Hello", "World", "!"].into_iter();

    let w: Vec<String> = v.map(String::from).collect();
}
```

Here from `&str` to `String`.


[filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) is kind of like chainng `map` and `filter`. It has the advantage of dealing with `Option` instead of `bool`:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn filter_map() {
    let v = vec!["Hello", "World", "!"].into_iter();

    let w: Vec<String> = v
        .filter_map(|x| {
            if x.len() > 2 {
                Some(String::from(x))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(w, vec!["Hello".to_string(), "World".to_string()]);
}
```

[chain](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain) merges two iterators:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn chain() {
    let x = vec![1, 2, 3, 4, 5].into_iter();
    let y = vec![6, 7, 8, 9, 10].into_iter();

    let z: Vec<u64> = x.chain(y).collect();
    assert_eq!(z.len(), 10);
}
```


And many more...


## Algebraic data types and pattern matching

Enums are certainly the favorite Rust's feature of new Rustaceans because they are the foundations of `Result` and `Option`. They allow us to express all the invariants of the domain and check at compile time that all cases are covered.

You can then use the `match` keyword to do pattern matching against enums.

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


But that's not all. Pattern matching in Rust can be used to match against many other expressions:

```rust
match x {
    42 => println!("Good!"),
    _ => println!("Bad!"),
}

let boolean = true;
// Match is an expression too
let binary = match boolean {
    false => 0,
    true => 1,
};


let x = Some(42u64);

match x {
    Some(1) => println!("1"),
    Some(42) => println!("42"),
    None => println!("none"),
};
```

## Streams

Streams are iterators for the `async` world.

**You should use them when you want to apply asynchronous operations on a sequence of items of the same type.**

It can be a network socket, a file, a long-lived HTTP request.

Anything that is too large and thus should be split in smaller chunks, or that may arrive later, but we don't know when, or that is simply a collection (a `Vec` or an `HashMap` for example) to which we need to apply `async` operations to.

Even if not directly related to Rust, I recommend the site [reactivex.io](http://reactivex.io) to learn more about the elegance and limitations of Streams.


Thank to the [StreamExt](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html) trait, you will be able to use the same combinators with Streams than with iterators, such as [filter](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.filter), [fold](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.fold), [for_each](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.for_each), [map](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.map) and so on.

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

**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) to learn Rust, Cybersecurity and Cryptography.**
