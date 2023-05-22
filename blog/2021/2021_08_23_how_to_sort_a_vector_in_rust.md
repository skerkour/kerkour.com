+++
date = 2021-08-23T14:00:00Z
title = "How to sort a vector in Rust"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-how-to-sort-a-vector"

[extra]
lang = "en"

comment ="""
# https://stackoverflow.com/questions/26836488/how-to-sort-a-vector-in-rust


 you forgot to mention sort_unstable() (faster if you don't care about preserving order of equal items), par_sort() and par_sort_unstable() (the fastest)
"""
+++


## sort

The simplest method (no pun intended) to sort a vector in Rust is to use [`sort`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort).

```rust
fn main() {
    let mut v = vec![3, 2, 90, 78, 64, 32, 1, -10, 10, 10000];
    v.sort();
    println!("{:?}", v);
}
```

```shell
[-10, 1, 2, 3, 10, 32, 64, 78, 90, 10000]
```

Please note that this method requires your type to implement the [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html) trait.

## sort_by


Alternatively, if your type does not implement [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html), you can use the [`sort_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by) method.



```rust
use std::cmp::Ordering;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let mut v = vec![
        Point { x: 3, y: 2 },
        Point { x: 90, y: 78 },
        Point { x: 64, y: 32 },
        Point { x: 1, y: -10 },
        Point { x: 10, y: 10000 },
    ];
    v.sort_by(|a, b| {
        if a.x < b.x {
            Ordering::Less
        } else if a.x == b.x {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    println!("{:?}", v);
}
```

```shell
[Point { x: 1, y: -10 }, Point { x: 3, y: 2 }, Point { x: 10, y: 10000 }, Point { x: 64, y: 32 }, Point { x: 90, y: 78 }]
```


## par_sort

If you want to sort items in parallel, you can also use [`par_sort`](https://docs.rs/rayon/latest/rayon/slice/trait.ParallelSliceMut.html#method.par_sort) and [`par_sort_by`](https://docs.rs/rayon/latest/rayon/slice/trait.ParallelSliceMut.html#method.par_sort_by) from the [`rayon`](https://crates.io/crates/rayon) crate.

```rust
use rayon::prelude::*;

fn main() {
    let mut v = vec![3, 2, 90, 78, 64, 32, 1, -10, 10, 10000];
    v.par_sort();
    println!("{:?}", v);
}
```



## sort_unstable

Finally, there are the [`sort_unstable`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable), [`sort_unstable_by`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable_by), [`par_sort_unstable_by`](https://docs.rs/rayon/latest/rayon/slice/trait.ParallelSliceMut.html#method.par_sort_unstable_by), and [`par_sort_unstable`](https://docs.rs/rayon/latest/rayon/slice/trait.ParallelSliceMut.html#method.par_sort_unstable) (the fastest), if you need absolute performance.


```rust
fn main() {
    let mut v = vec![3, 2, 90, 78, 64, 32, 1, -10, 10, 10000];
    v.sort_unstable();
    println!("{:?}", v);
}
```


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_sort_vector) (please don't forget to star the repo 🙏)
