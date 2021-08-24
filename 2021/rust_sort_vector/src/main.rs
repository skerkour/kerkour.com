use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    // sort
    let mut v = vec![3, 2, 90, 78, 64, 32, 1, -10, 10, 10000];
    v.sort();
    println!("{:?}", v);

    // sort_by
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

    // par_sort
    let mut v = vec![3, 2, 90, 78, 64, 32, 1, -10, 10, 10000];
    v.par_sort();
    println!("{:?}", v);
}
