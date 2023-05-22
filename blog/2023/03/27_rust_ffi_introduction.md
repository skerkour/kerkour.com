+++
date = 2023-03-27T07:00:00Z
title = "Calling C Code from Rust with FFI"
type = "post"
tags = ["programming", "rust"]
authors = ["Sylvain Kerkour"]
url = "/call-c-code-from-rust-with-ffi"
draft = true

[extra]
lang = "en"
+++

As Rust gains popularity for its safety, performance, and concurrency features, developers are increasingly looking for ways to leverage its strengths in combination with existing C libraries. Fortunately, Rust's Foreign Function Interface (FFI) provides a seamless and efficient way to achieve this interoperability. In this blog post, we'll explore the power of Rust's FFI and walk you through a step-by-step guide to interoperate with C code. We'll cover the most important ideas on the topic, complete with code examples and anecdotes.

## What is Rust's FFI


Rust's FFI allows you to call functions written in other languages, such as C, directly from your Rust code. This is particularly useful when you want to use a battle-tested library written in C, or when you need to integrate your Rust code into a larger C codebase.

To demonstrate Rust's FFI capabilities, let's start with a simple example. Suppose you have a C library that contains the following function, which calculates the factorial of a given integer:

```c
// factorial.c
#include <stdint.h>

uint64_t factorial(uint64_t n) {
    if (n == 0) {
        return 1;
    }
    return n * factorial(n - 1);
}
```

Our goal is to call this factorial function from Rust code. To achieve this, we'll follow these steps:

1.1. Create a C-compatible header file

First, create a header file that declares the C function we want to call from Rust. In our case, the header file should contain the prototype of the factorial function:


```c
// factorial.h
#include <stdint.h>

uint64_t factorial(uint64_t n);
```

1.2. Compile the C library

Next, compile the C library as a shared library. On Linux, you can use the following command:

```shell
$ gcc -shared -o libfactorial.so factorial.c
```

On macOS, use the following command:

```shell
$ gcc -shared -o libfactorial.dylib factorial.c
```

This will create a shared library file named `libfactorial.so` (or `libfactorial.dylib` on macOS) that contains the compiled factorial function.

## Create a Rust wrapper for the C function

Now, let's create a Rust file that imports the `factorial` function from the shared library using the `extern` keyword. We'll also provide a type signature for the imported function, ensuring that it matches the C function's signature:


```rust
// factorial_wrapper.rs
extern "C" {
    pub fn factorial(n: u64) -> u64;
}
```

1.4. Use the C function in your Rust code

Finally, we can use the imported `factorial` function in our Rust code. To do this, we'll need to link our Rust program with the shared library we created earlier. We'll use the `println!` macro to output the result of calling the `factorial` function:

```rust
// main.rs
mod factorial_wrapper;

fn main() {
    let n = 10;
    let result = unsafe { factorial_wrapper::factorial(n) };
    println!("The factorial of {} is {}", n, result);
}
```


In the `Cargo.toml` file, add the following lines to import the required dependencies and specify the linker arguments:

```toml
[dependencies]
libc = "0.2"

[build]
rustc-link-search = ["."]
rustc-link-lib = ["factorial"]
```

Now, run your Rust program:

```shell
$ cargo run
```


If everything is set up correctly, you should see the following output:
```
The factorial of 10 is 3628800
```

Congratulations! You've successfully called a C function from your Rust code using Rust's FFI.

## Passing Complex Data Types Between Rust and C

Now that we've covered the basics, let's dive into more advanced examples involving complex data types. We'll discuss how to pass strings and structs between Rust and C.

2.1. Passing Strings

When working with strings, Rust and C have different representations. Rust uses the `String` and `&str` types, while C uses null-terminated character arrays. To pass a Rust string to a C function, we'll need to convert it to a C-compatible format. Let's consider the following C function that prints a greeting message:


```c
// greeting.c
#include <stdio.h>

void print_greeting(const char *name) {
    printf("Hello, %s!\n", name);
}
```


To call this function from Rust, first create a header file with the function prototype:

```c
// greeting.h
void print_greeting(const char *name);
```

Now, create a Rust wrapper for the C function:

```rust
// greeting_wrapper.rs
extern "C" {
    pub fn print_greeting(name: *const libc::c_char);
}
```


In your Rust code, convert the Rust string to a C-compatible format using the `CString` type from the `std::ffi` module:

```rust
// main.rs
use std::ffi::CString;
mod greeting_wrapper;

fn main() {
    let name = "Sylvain";
    let c_name = CString::new(name).unwrap();
    unsafe { greeting_wrapper::print_greeting(c_name.as_ptr()) };
}
```

Compile and run your Rust program, and you should see the following output:

```
Hello, Sylvain!
```


## 2.2. Passing Structs

To pass a struct between Rust and C, ensure that both representations have the same memory layout. Let's consider a simple example involving a 2D point struct:

```c
// point.c
#include <stdio.h>

typedef struct Point {
    double x;
    double y;
} Point;

void print_point(const Point *point) {
    printf("Point: (%f, %f)\n", point->x, point->y);
}
```

Create a header file with the struct definition and function prototype:

```c
// point.h
typedef struct Point {
    double x;
    double y;
} Point;

void print_point(const Point *point);
```

Now, create a Rust wrapper for the C function and a corresponding Rust struct with the same memory layout:


```rust
// point_wrapper.rs
#[repr(C)]
pub struct Point {
    x: f64,
    y: f64,
}

extern "C" {
    pub fn print_point(point: *const Point);
}
```

In your Rust code, create an instance of the Point struct and pass it to the C function:
```rust
// main.rs
mod point_wrapper;

fn main() {
    let point = point_wrapper::Point { x: 3.0, y: 4.0 };
    unsafe { point_wrapper::print_point(&point) };
}
```

Compile and run your Rust program, and you should see the following output:

```
Point: (3.000000, 4.000000)
```


## Conclusion

In this blog post, we explored the power of Rust's FFI and provided a step-by-step guide to interoperate with C code. We discussed the basics of calling C functions from Rust, passing complex data types like strings and structs between the two languages, and handling errors and panics.

By leveraging Rust's FFI, you can tap into the vast ecosystem of C libraries and improve the interoperability of your Rust projects. As you continue to explore Rust's FFI capabilities, remember to handle errors carefully, ensure memory safety, and always be mindful of the differences between Rust and C. Happy coding!
