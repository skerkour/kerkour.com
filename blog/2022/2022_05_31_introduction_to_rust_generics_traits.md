+++
date = 2022-05-31T00:15:00Z
title = "Introduction to Rust generics [1/2]: Traits"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-generics-traits"

[extra]
lang = "en"

comment ="""
"""
+++

**Introduction to Rust generics**:
* **[Traits](https://kerkour.com/rust-generics-traits)**
* [Trait Objects (Static vs Dynamic dispatch)](https://kerkour.com/rust-generics-trait-objects)


Imagine that you want to add a camera to your computer which is lacking one. You buy a webcam and connect it via a USB port. Now imagine that you want to add storage to the same computer. You buy an external hard drive and also connect it via a similar USB port.

This is the power of generics applied to the world of physical gadgets. A USB port is a **generic** port, and an accessory that connects to it is a **module**. You don't have device-specific ports, such as a specific port for a specific webcam vendor, another port for another vendor, another one for one vendor of USB external drives, and so on... You can connect almost any USB device to any USB port and have it working (minus software drivers compatibility...). Your PC vendors don't have to plan for any module you may want to connect to your computer. They just have to follow the generic and universal USB specification.

The same applies to code. A function can perform a specific task against a specific type, and a generic function can perform a specific task on *some* (more on that later) types.


> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


`add` can only add two `i64` variables.

```rust
fn add(x: i64, y: i64) -> i64 {
    return x + y;
}
```


Here, `add` can add two variables of any type.

```rust
fn add<T>(x: T, y: T) -> T {
    return x + y;
}
```

**But this code is not valid**: it makes no sense to add two planes (for example). And the compiler don't even know how to add two planes! This is where **constraints** come into play.

```rust
use std::ops::Add;

fn add<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}
```

Here, `add` can add any types that implement the [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html) trait. By the way, this is how we do operator overloading in Rust: by implementing traits from the [`std::ops`](https://doc.rust-lang.org/stable/std/ops/) module.



## Generics

Generic programming's goal is to improve code reusability and reduce bugs by allowing functions, structures, and traits to have their types *defined later*.

In practice, it means that an algorithm can be used with multiple different types, provided that they fulfill the **constraints**. As a result, if you find a bug in your generic algorithm, you only have to fix it once. If you had to implement the algorithm 4 times for 4 different but similar types (let say `int32`, `int64`, `float32`, `float64`), not only you spent 4x more time to implement it, but you will also spend 4x more time fixing the same bug in all the implementations (granted you didn't introduce other bugs due to fatigue).


In Rust, functions, traits (more on that below), and data types can be generic:
```rust
use std::fmt::Display;

// a generic function, whose type parameter T is constrained
fn generic_display<T: Display>(item: T) {
    println!("{}", item);
}

// a generic struct
struct Point<T> {
    x: T,
    y: T,
}

// another generic struct
struct Point2<T>(T, T)

// a generic enum
enum Option<T> {
    Some(T),
    None
}


fn main() {
    let a: &str = "42";
    let b: i64 = 42;

    generic_display(a);
    generic_display(b);

    let (x, y) = (4i64, 2i64);

    let point: Point<i64> = Point {
        x,
        y
    };

    // generic_display(point) <- not possible. Point does not implement Display
}
```

Generics are what allow Rust to be so expressive. Without them, it would not be possible to have generic collections such as [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), [`HashMap`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html), or [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html).


```rust
use std::collections::HashMap;

struct Contact {
    name: String,
    email: String,
}

fn main() {
    // imagine a list of imported contacts with duplicates
    let imported_contacts = vec![
        Contact {
            name: "John".to_string(),
            email: "john@smith.com".to_string(),
        },
        Contact {
            name: "steve".to_string(),
            email: "steve@jobs.com".to_string(),
        },
        Contact {
            name: "John".to_string(),
            email: "john@smith.com".to_string(),
        },
        // ...
    ];

    let unique_contacts: HashMap<String, Contact> = imported_contacts
            .into_iter()
            .map(|contact| (contact.email.clone(), contact))
            .collect();
}
```

Thanks to the power of generics, we can reuse `HashMap` from the standard library and quickly deduplicate our data!


Imagine having to implement those collections for **all the types** in your programs?


## Traits

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


Traits are the Rust's equivalent of interfaces in other languages (with some differences).

As defining a term by its synonym is not really useful, let see what does it mean in code:

```rust
pub trait Dog {
    fn bark(&self) -> String;
}

pub struct Labrador{}

impl Dog for Labrador {
    fn bark(&self) -> String {
        "wouf".to_string()
    }
}

pub struct Husky{}

impl Dog for Husky {
    fn bark(&self) -> String {
        "Wuuuuuu".to_string()
    }
}

fn main() {
    let labrador = Labrador{};
    println!("{}", labrador.bark());

    let husky = Husky{};
    println!("{}", husky.bark());
}

// Output:

// wouf
// Wuuuuuu
```


By defining a `Dog` interface, all types that implement this trait in our program will be considered as being a `Dog`.

This is why we say that traits (and interfaces) allow programmers to define **shared behavior**: behaviors that are shared by multiple types.



### Default Implementations

It's possible to provide default implementations for trait methods:

```rust
pub trait Hello {
    fn hello(&self) -> String {
        String::from("World")
    }
}

pub struct Sylvain {}

impl Hello for Sylvain {
    fn hello(&self) -> String {
        String::from("Sylvain")
    }
}

pub struct Anonymous {}

impl Hello for Anonymous {}

fn main() {
    let sylvain = Sylvain{};
    let anonymous = Anonymous{};

    println!("Sylvain: {}", sylvain.hello());
    println!("Anonymous: {}", anonymous.hello());
}
// Output:

// Sylvain: Sylvain
// Anonymous: World
```


### Traits composition

Traits can be composed to require more advanced constraints:

```rust
pub trait Module {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

pub trait SubdomainModule {
    fn enumerate(&self, domain: &str) -> Result<Vec<String>, Error>;
}

fn enumerate_subdomains<M: Module + SubdomainModule>(module: M, target: &str) -> Vec<String> {
    // ...
}
```

<!--

In traditional [Object Oriented Programming (OOP)](https://en.wikipedia.org/wiki/Object-oriented_programming) style, shared behavior between classes is expressed through inheritance, for example, in C++:
```c++
// Shared behavior
class Module {
    public:
        string name();
        string description();
}

// Derived classes
class SubdomainModule: public Module {
    public:
        enumerate(string);
};


class HttpModule: public Module {
    public:
        scan(string);
};
```

On the other hand, Rust favors composition:
```rust
// Shared behavior
pub trait Module {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

#[async_trait]
pub trait SubdomainModule: Module {
    async fn enumerate(&self, domain: &str) -> Result<Vec<String>, Error>;
}


#[async_trait]
pub trait HttpModule: Module {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error>;
}
``` -->



<!--
> Coming soon

Now, back to our scanner. We want to add modules in order to add features and scanning capabilities. -->
<!-- TODO code example de function module -->

<!-- so we will put all those modules in a list -->

<!-- TODO code generic list-->

<!-- aaaaaannnd -->

<!-- TODO it does not work -->


<!-- It does not work!

Indeed, -->


### Async Traits

As of today, `async` functions in traits are not natively supported by Rust. Fortunately, [David Tolnay](https://github.com/dtolnay) got our back covered (one more time): we can use the [async-trait](https://github.com/dtolnay/async-trait) crate.

```rust
#[async_trait]
pub trait HttpModule: Module {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error>;
}
```

### Generic traits

Traits can also have generic parameters:

```rust
use std::fmt::Display;

trait Printer<S: Display> {
    fn print(&self, to_print: S) {
        println!("{}", to_print);
    }
}

struct ActualPrinter{}

impl<S: Display> Printer<S> for ActualPrinter {}

fn main() {
    let s = "Hello";
    let n: i64 = 42;
    let printer = ActualPrinter{};

    printer.print(s);
    printer.print(n);
}

// output:

// Hello
// 42
```

And even better, you can implement a generic trait for a generic type:
```rust
use std::fmt::Display;

trait Printer<S: Display> {
    fn print(&self, to_print: S) {
        println!("{}", to_print);
    }
}

// implements Printer<S: Display> for any type T
impl<S: Display, T> Printer<S> for T {}

fn main() {
    let s = "Hello";
    let printer: i64 = 42;

    printer.print(s);
}

// Output:

// Hello
```

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)



### The `derive` attribute

When you have a lot of traits to implement for your types, it can quickly become tedious and may complexify your code.

Fortunately, Rust has something for us: the `derive` [attribute](https://doc.rust-lang.org/rust-by-example/attribute.html).

By using the `derive` attribute, we are actually feeding our types to a [Derive macro](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros) which is a kind of [procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html#procedural-macros).

They take code as input (in this case, our type), and create more code as output. At compile-time.


This is especially useful for data deserialization: Just by implementing the  [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits from the [`serde`](https://docs.rs/serde) crate, the (almost) universally used serialization library in the Rust world, we can then serialize and deserialize our types to a lot of data formats: [JSON](https://github.com/serde-rs/json), [YAML](https://github.com/dtolnay/serde-yaml), [TOML](https://github.com/alexcrichton/toml-rs), [BSON](https://github.com/mongodb/bson-rust) and so on...


```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Point {
    x: u64,
    y: u64,
}
```

Without much effort, we just implemented the [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html), [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits for our `struct Point`.


One thing to note is that all the subfields of your `struct` need to implement the traits:
```rust
use serde::{Serialize, Deserialize};

// Not possible:
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Point<T> {
    x: T,
    y: T,
}

// instead, do this:
use serde::{Serialize, Deserialize};
use core::fmt::Debug; // Import the Debug trait

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Point<T: Debug + Clone + Serialize + Deserialize> {
    x: T,
    y: T,
}
```
