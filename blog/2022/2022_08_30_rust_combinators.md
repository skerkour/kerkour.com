+++
date = 2022-08-30T13:30:00Z
title = "Learning Rust: Combinators"
type = "post"
tags = ["hacking", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-combinators"

[extra]
lang = "en"
+++

<!--
You have decided that it's time to finally enjoy programming and that Rust, [Stack Overflow's most loved language for 7 years in a row](https://survey.stackoverflow.co/2022/#technology-most-loved-dreaded-and-wanted) might be the way?


Then you start reading some Rust code and quickly feel overwhelmed by the syntax and what looks like a huge number of features.

It's normal, I've been there. Rust is a programming language with an already huge surface and [it keeps growing](https://kerkour.com/the-biggest-threat-to-rust-sustainability), that's why I believe that the best way to learn Rust to day is to practice selective ignorance and focsu on the [20% that brings 80% of the results](https://en.wikipedia.org/wiki/Pareto_principle).


## Features to learn



## What to ignore

### writing macros


But today is your lucky day because I



The

and, maybe more importantly, which features to avoid.


Now it's time to practice. I wrote a projects based book where youll learn rust, security and crpytography by coding real projects such as:



Ready to dive? -->


Combinators are a very interesting to make your code cleaner and more functional. Almost all the definitions you'll find on the internet will make your head explode 🤯 because they raise more questions than they answer.

Thus, here is my empiric definition: Combinators are methods that ease the manipulation of some type `T`. They favor a functional (method chaining) style of code.

```rust
let sum: u64 = vec![1, 2, 3].into_iter().map(|x| x * x).sum();
```


This section will be pure how-to and real-world patterns about how combinators make your code easier to read or refactor.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)
> [Get 64% off until **Sunday, September 4** with the **BACK2HACKING**](https://kerkour.com/black-hat-rust-discount-august-2022) coupon


### Iterators

Let start with iterators because this is certainly the situation where combinators are the most used.

#### Obtaining an iterator

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

#### Consuming iterators


Iterators are lazy: they won't do anything if they are not consumed.

As we have just seen, Iterators can be consumed with `for x in` loops. But this is not where they are the most used. Idiomatic Rust favor functional programming. It's a better fit for its ownership model.


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


Conversely, you can obtain an `HashMap` (or a `BTreeMap`, or other collections, see [https://doc.rust-lang.org/std/iter/trait.FromIterator.html#implementors](https://doc.rust-lang.org/std/iter/trait.FromIterator.html#implementors), using `from_iter`:

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



[fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) is like `reduce` but can return an accumulator of different type than the items of the iterator:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn fold() {
    let values = vec!["Hello", "World", "!"].into_iter();

    let _sentence = values.fold(String::new(), |acc, x| acc + x);
}
```

Here `_sentence` is a `String`, while the items of the iterator are of type `&str`.



#### Combinators

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


[filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) is kind of like chaining `map` and `filter`. It has the advantage of dealing with `Option` instead of `bool`:

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

[flatten](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten) can be used to flatten collections of collections:

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn flatten() {
    let x = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]].into_iter();

    let z: Vec<u64> = x.flatten().collect();
    assert_eq!(z.len(), 10);
}
```

Now `z = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]`;

<!-- [flat_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)
```rust
// .flat_map()
``` -->


##### Composing combinators

This is where combinators shine: they make your code more elegant and (most of the time) easier to read because closer to how Humans think than how computers work.


**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
#[test]
fn combinators() {
    let a = vec![
        "1",
        "2",
        "-1",
        "4",
        "-4",
        "100",
        "invalid",
        "Not a number",
        "",
    ];

    let _only_positive_numbers: Vec<i64> = a
        .into_iter()
        .filter_map(|x| x.parse::<i64>().ok())
        .filter(|x| x > &0)
        .collect();
}
```

For example, the code snippet above replaces a big loop with complex logic, and instead, in a few lines, we do the following:

* Try to parse an array of collection of strings into numbers
* filter out invalid results
* filter numbers less than 0
* collect everything in a new vector

It has the advantage of working with immutable data and thus reduces the probability of bugs.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)
> [Get 64% off until **Sunday, September 4** with the **BACK2HACKING**](https://kerkour.com/black-hat-rust-discount-august-2022) coupon


### `Option`


Use a default value: [unwrap_or](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or)
```rust
fn option_unwrap_or() {
    let _port = std::env::var("PORT").ok().unwrap_or(String::from("8080"));
}
```

Use a default `Option` value: [or](https://doc.rust-lang.org/std/option/enum.Option.html#method.or)
```rust
// config.port is an Option<String>
let _port = config.port.or(std::env::var("PORT").ok());
// _port is an Option<String>
```

Call a function if `Option` is `Some`: [and_then](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then)
```rust
fn port_to_address() -> Option<String> {
    // ...
}

let _address = std::env::var("PORT").ok().and_then(port_to_address);
```

Call a function if `Option` is `None`: [or_else](https://doc.rust-lang.org/std/option/enum.Option.html#method.or_else)
```rust
fn get_default_port() -> Option<String> {
    // ...
}

let _port = std::env::var("PORT").ok().or_else(get_default_port);
```


<!-- Get a (mutable, or not) reference: [as_ref](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref) and [as_mut](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_mut)
```rust

// .as_ref() & .as-mut()
``` -->

And the two extremely useful function for the `Option` type:
[is_some](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some) and [is_none](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none)

`is_some` returns `true` is an `Option` is `Some` (contains a value):
```rust
let a: Option<u32> = Some(1);

if a.is_some() {
    println!("will be printed");
}

let b: Option<u32> = None;

if b.is_some() {
    println!("will NOT be printed");
}
```


`is_none` returns `true` is an `Option` is `None` (does **not** contain a value):
```rust
let a: Option<u32> = Some(1);

if a.is_none() {
    println!("will NOT be printed");
}


let b: Option<u32> = None;

if b.is_none() {
    println!("will be printed");
}
```




You can find the other (and in my experience, less commonly used) combinators for the `Option` type online: [https://doc.rust-lang.org/std/option/enum.Option.html](https://doc.rust-lang.org/std/option/enum.Option.html).


### `Result`


Convert a `Result` to an `Option` with [`ok`](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok):

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn result_ok() {
    let _port: Option<String> = std::env::var("PORT").ok();
}
```


Use a default `Result` if `Result` is `Err` with [`or`](https://doc.rust-lang.org/std/result/enum.Result.html#method.or):

**[ch_03/snippets/combinators/src/main.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_03/snippets/combinators/src/main.rs)**
```rust
fn result_or() {
    let _port: Result<String, std::env::VarError> =
        std::env::var("PORT").or(Ok(String::from("8080")));
}
```


[`map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err) converts a `Result<T, E>` to a `Result<T, F>` by calling a function:

```rust
fn convert_error(err: ErrorType1) -> ErrorType2 {
    // ...
}


let _port: Result<String, ErrorType2> = std::env::var("PORT").map_err(convert_error);
```



Call a function if `Results` is `Ok`: [and_then](https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then).

```rust
fn port_to_address() -> Option<String> {
    // ...
}

let _address = std::env::var("PORT").and_then(port_to_address);
```

Call a function and default value: [map_or](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or)
```rust
let http_port = std::env::var("PORT")
    .map_or(Ok(String::from("8080")), |env_val| env_val.parse::<u16>())?;
```


Chain a function if `Result` is `Ok`: [map](https://doc.rust-lang.org/std/result/enum.Result.html#method.map)

```rust
let master_key = std::env::var("MASTER_KEY")
    .map_err(|_| env_not_found("MASTER_KEY"))
    .map(base64::decode)??;
```


<!-- Get a (mutable, or not) reference: [as_ref](https://doc.rust-lang.org/std/result/enum.Result.html#method.as_ref) and [as_mut](https://doc.rust-lang.org/std/result/enum.Result.html#method.as_mut)
```rust

// .as_ref() & .as_mut()
``` -->


And the last two extremely useful functions for the `Result` type:
[is_ok](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok) and [is_err](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_err)


`is_ok` returns `true` is an `Result` is `Ok`:

```rust
if std::env::var("DOES_EXIST").is_ok() {
    println!("will be printed");
}

if std::env::var("DOES_NOT_EXIST").is_ok() {
    println!("will NOT be printed");
}
```

`is_err` returns `true` is an `Result` is `Err`:

```rust
if std::env::var("DOES_NOT_EXIST").is_err() {
    println!("will be printed");
}

if std::env::var("DOES_EXIST").is_err() {
    println!("will NOT be printed");
}
```


You can find the other (and in my experience, less commonly used) combinators for the `Result` type online: [https://doc.rust-lang.org/std/result/enum.Result.html](https://doc.rust-lang.org/std/result/enum.Result.html).


> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)
> [Get 64% off until **Sunday, September 4** with the **BACK2HACKING**](https://kerkour.com/black-hat-rust-discount-august-2022) coupon

### When to use `.unwrap()` and `.expect()`

`unwrap` and `expect` can be used on both `Option` and `Result`. They have the potential to crash your program, so use them with parsimony.

I see 2 situations where it's legitimate to use them:

* Either when doing exploration, and quick script-like programs, to not bother with handling all the edge cases.
* When you are sure they will never crash, **but**, they should be accompanied by a comment explaining why it's safe to use them and why they won't crash the program.
