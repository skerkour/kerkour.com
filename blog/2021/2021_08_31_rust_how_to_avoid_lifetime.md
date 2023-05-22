+++
date = 2021-08-31T14:00:00Z
title = "Smart pointers: The secret to writing clean Rust code"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-avoid-lifetimes"

[extra]
lang = "en"

comment ="""
"""
+++



Lifetime annotations are one of the things that distract the most new rustaceans.


From my experience writing and digging into Rust codebases, lifetime (annotations) induce a cognitive load that distracts from getting the actual work done. They not only make your code harder to read but also harder to use, especially when combined with generics.


<!--
Lifetime annotations are one of the things that distract the most new rustaceans.


I believe they are a pure <s>evil</s> waste of time. Let's be honest, lifetime (annotations) are ugly and induce a cognitive load that distracts from getting the actual things done. They not only make your code harder to read but also harder to use.

I hate them with a passion, especially when combined with generics.
 -->

```rust
// Haha is a struct to wrap a monad generator to provide a facade for any kind of generic iterator. Because.
struct Haha<'y, 'o, L, O>
  where for<'oO> L: FnOnce(&'oO O) -> &'o O,
  O: Trait<L, 'o, L>,
  O::Item : Clone + Debug + 'static {
    x: L,
}
```

Yeaah suure, please don't mind that somebody, someday, will have to read and understand your code.


But lifetimes annotations are avoidable and **should be avoided**. So here is my strategy to avoid turning Rust code into some kind of monstrosity that nobody will ever want to touch and slowly die of disregard.


## Why are lifetime annotations needed in the first place?

Lifetime annotations are needed to tell the compiler that we are manipulating some kind of long-lived reference and let him assert that we are not going to screw ourselves.


## Lifetime Elision

The simplest and most basic trick is to omit the lifetime annotation.

```rust
fn do_something(x: &u64) {
  println!("{}", x);
}
```


It's most of the time easy to elide input lifetimes, but beware that to omit output lifetime annotations, you have to follow [these 3 rules](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/lifetimes.html#lifetime-elision):

* *Each elided lifetime in a function's arguments becomes a distinct lifetime parameter.*
* *If there is exactly one input lifetime, elided or not, that lifetime is assigned to all elided lifetimes in the return values of that function.*
* *If there are multiple input lifetimes, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.*

Otherwise, it is an error to elide an output lifetime.

```rust
fn do_something(x: &u64)-> &u64 {
    println!("{}", x);
    x
}

// is equivalent to
fn do_something_else<'a>(x: &'a u64)-> &'a u64 {
    println!("{}", x);
    x
}
```

## Smart pointers


Now, not everything is as simple as an `HelloWorld` and you may need some kind of long-lived reference that you can use at multiple places of your codebase (a Database connection for example, or an HTTP client with an internal connection pool).

The solution for long-lived, shared (or not), mutable (or not) references is to use [smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html).


The only downside is that smart pointers, in Rust, are a little bit verbose (but still way less ugly than lifetime annotations).


### Rc


```rust
use std::rc::Rc;

fn main() {
    let pointer = Rc::new(1);

    {
        let second_pointer = pointer.clone(); // or Rc::clone(&pointer)
        println!("{}", *second_pointer);
    }

    println!("{}", *pointer);
}
```

To obtain a mutable, shared pointer, you can use use the [interior mutability pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#refcellt-and-the-interior-mutability-pattern):

```rust
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

fn main() {
    let shared_string = Rc::new(RefCell::new("Hello".to_string()));

    {
        let mut hello_world: RefMut<String> = shared_string.borrow_mut();
        hello_world.push_str(" World");
    }

    println!("{}", shared_string.take());
}
```


### Arc

Unfortunately, `Rc<RefCell<T>>` cannot be used across threads or in an `async` context. This is where [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) comes into play, which implements [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html) and [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html) and thus is safe to share across threads.

```rust
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let pointer = Arc::new(5);

    let second_pointer = pointer.clone(); // or Arc::clone(&pointer)
    thread::spawn(move || {
        println!("{}", *second_pointer); // 5
    });

    thread::sleep(time::Duration::from_secs(1));

    println!("{}", *pointer); // 5
}
```


For mutable shared variables, you can use `Arc<Mutex<T>>`:
```rust
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let pointer = Arc::new(Mutex::new(5));

    let second_pointer = pointer.clone(); // or Arc::clone(&pointer)
    thread::spawn(move || {
        let mut mutable_pointer = second_pointer.lock().unwrap();
        *mutable_pointer = 1;
    });

    thread::sleep(time::Duration::from_secs(1));

    let one = pointer.lock().unwrap();
    println!("{}", one); // 1
}
```


Smart pointers are particularly useful when embedded into structures:
```rust
struct MyService {
  db: Arc<DB>,
  mailer: Arc<dyn drivers::Mailer>,
  storage: Arc<dyn drivers::Storage>,
  other_service: Arc<other::Service>,
}
```

## When to use lifetimes annotations

In my opinion, lifetimes annotations should never surface in any public API. It's okay to use them if you need absolute performance <u>AND</u> minimal resources usage <u>AND</u> are doing embedded development, but you should keep them hidden in your code, and they should never surface in the public API.
