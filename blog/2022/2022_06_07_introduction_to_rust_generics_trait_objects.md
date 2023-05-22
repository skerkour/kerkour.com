+++
date = 2022-06-07T02:00:00Z
title = "Introduction to Rust generics [2/2]: Trait Objects (Static vs Dynamic dispatch)"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-generics-trait-objects"

[extra]
lang = "en"

comment ="""
"""
+++

**Introduction to Rust generics**:
* [Traits](https://kerkour.com/rust-generics-traits)
* **[Trait Objects (Static vs Dynamic dispatch)](https://kerkour.com/rust-generics-trait-objects)**


> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


Now you may be wondering: How to create a collection that can contain different concrete types that satisfy a given trait? For example:
```rust
trait UsbModule {
    // ...
}

struct UsbCamera {
     // ...
}

impl UsbModule for UsbCamera {
    // ..
}

impl UsbCamera {
    // ...
}

struct UsbMicrophone{
     // ...
}

impl UsbModule for UsbMicrophone {
    // ..
}

impl UsbMicrophone {
    // ...
}

let peripheral_devices: Vec<UsbModule> = vec![
    UsbCamera::new(),
    UsbMicrophone::new(),
];
```

Unfortunately, this is not as simple in Rust. As the modules may have a different size in memory, the compiler doesn't allow us to create such a collection. All the elements of the vector don't have the same shape.


**Traits objects** solve precisely this problem: when you want to use different concrete types (of varying shape) adhering to a contract (the trait), at runtime.


Instead of using the objects directly, we are going to use pointers to the objects in our collection. This time, the compiler will accept our code, as every pointer has the same size.

How to do this in practice? We will see below when adding modules to our scanner.


### Static vs Dynamic dispatch

So, what is the technical difference between a generic parameter and a trait object?

When you use a generic parameter (here for the `process` function):
**[ch_04/snippets/dispatch/src/statik.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_04/snippets/dispatch/src/statik.rs)**
```rust
trait Processor {
    fn compute(&self, x: i64, y: i64) -> i64;
}

struct Risc {}

impl Processor for Risc {
    fn compute(&self, x: i64, y: i64) -> i64 {
        x + y
    }
}

struct Cisc {}

impl Processor for Cisc {
    fn compute(&self, x: i64, y: i64) -> i64 {
        x * y
    }
}

fn process<P: Processor>(processor: &P, x: i64) {
    let result = processor.compute(x, 42);
    println!("{}", result);
}

pub fn main() {
    let processor1 = Cisc {};
    let processor2 = Risc {};

    process(&processor1, 1);
    process(&processor2, 2);
}
```


The compiler generates a **specialized version for each type you call the function with** and then replaces the call sites with calls to these specialized functions.

This is known as **monomorphization**.

For example the code above is roughly equivalent to:
```rust
fn process_Risc(processor: &Risc, x: i64) {
    let result = processor.compute(x, 42);
    println!("{}", result);
}

fn process_Cisc(processor: &Cisc, x: i64) {
    let result = processor.compute(x, 42);
    println!("{}", result);
}
```

It's the same thing as if you were implementing these functions yourself. This is known as **static dispatch**. The type selection is made statically at compile time. It provides the best runtime performance.


On the other hand, when you use a trait object:
**[ch_04/snippets/dispatch/src/dynamic.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_04/snippets/dispatch/src/dynamic.rs)**
```rust
trait Processor {
    fn compute(&self, x: i64, y: i64) -> i64;
}

struct Risc {}

impl Processor for Risc {
    fn compute(&self, x: i64, y: i64) -> i64 {
        x + y
    }
}

struct Cisc {}

impl Processor for Cisc {
    fn compute(&self, x: i64, y: i64) -> i64 {
        x * y
    }
}

fn process(processor: &dyn Processor, x: i64) {
    let result = processor.compute(x, 42);
    println!("{}", result);
}

pub fn main() {
    let processors: Vec<Box<dyn Processor>> = vec![
        Box::new(Cisc {}),
        Box::new(Risc {}),
    ];

    for processor in processors {
        process(&*processor, 1);
    }
}
```

The compiler will generate only 1 `process` function. It's at runtime that your program will detect which kind of `Processor` is the `processor` variable and thus which `compute` method to call. This is known **dynamic dispatch**. The type selection is made dynamically at runtime.

The syntax for trait objects `&dyn Processor` may appear a little bit heavy, especially when coming from less verbose languages. I personally love it! In one look, we can see that the function accepts a trait object, thanks to `dyn Processor`.

The reference `&` is required because Rust needs to know the exact size for each variable.

As the structures implementing the `Processor` trait may vary in size, the only solution is then to pass a reference. It could also have been a (smart) pointer such as `Box`, `Rc` or `Arc`.

**The point is that the `processor` variable needs to have a size known at compile time.**

Note that in this specific example, we do `&*processor` because we first need to dereference the `Box` in order to pass the reference to the `process` function. This is the equivalent of `process(&(*processor), 1)`.

When compiling dynamically dispatched functions, Rust will create under the hood what is called a [vtable](https://en.wikipedia.org/wiki/Virtual_method_table), and use this vtable at runtime to choose which function to call.

<!-- > Coming soon: VTable -->

<!-- explain VTable
https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit#slide=id.p
https://llogiq.github.io/2020/03/14/ootb.html
https://alschwalm.com/blog/static/2017/03/07/exploring-dynamic-dispatch-in-rust/
https://github.com/usagi/rust-memory-container-cs
https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
-->

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


### Some Closing Thoughts

Use static dispatch when you need absolute performance and trait objects when you need more flexibility or collections of objects sharing the same behavior.
