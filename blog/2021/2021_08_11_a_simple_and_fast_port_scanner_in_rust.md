+++
date = 2021-08-11T16:05:00Z
title = "A fast port scanner in 100 lines of Rust"
type = "post"
tags = ["hacking", "rust", "programming", "tutorial", "security"]
authors = ["Sylvain Kerkour"]
url = "/rust-fast-port-scanner"

[extra]
lang = "en"

comment ="""
a fast and simple port scanner in rust

first we need to extracts most common ports feom nmap

then parse cli args

and finally we concurrently scan ports
"""
+++


To write a fast port scanner, a programming language requires:

* **A Good I/O model**, not to eat all the resources of the system.
* **High-level abstractions and a good packaging system** to isolate low-level code and reuse it easily.
* **To be type and memory safe**, because who wants offensive tools with vulnerabilities?
* **And, ideally, to be compiled**, because most of the time, it's worth trading a little bit of compile time for extreme runtime speed.


Guess what? These are precisely Rust's selling points. So let see how to build a high-speed port scanner in 100 lines of Rust.

A port scanner is basically composed of 3 parts:
* **A list of ports to scan**
* **A port scanning algorithm** (see this list [on nmap's website](https://nmap.org/book/man-port-scanning-techniques.html))
* **A concurrency primitive**, to scan port concurrently


## A Generic port list

Scanning all the *65535* ports is often wasteful and useless. Thus, we first need to extract the list of the most common open ports in the wild. Fortunately, the nmap project already has such a list:

```rust
// from awk '$2~/tcp$/' /usr/share/nmap/nmap-services | sort -r -k3 | head -n 1000 | tr -s ' ' | cut -d '/' -f1 | sed 's/\S*\s*\(\S*\).*/\1,/'
pub const MOST_COMMON_PORTS_1002: &[u16] = &[
    5601, 9300, 80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111,
    995, 993, 5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179,
    1026, 2000, 8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666, 646,
    5000, 5631, 631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513, 990, 5357,
    427, 49156, 543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070, 5190, 3000, 5432,
    1900, 3986, 13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899, 9100, 119, 37, 1000,
    3001, 5001, 82, 10010, 1030, 9090, 2107, 1024, 2103, 6004, 1801, 5050, 19, 8031, 1041, 255,
    // ...
];
```

Then, we need to be able to return either this list or all the 65535 ports. In Rust, the way to achieve this is with an [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html#).

But, as Iterator is a trait, we need to use a [Trait Object](https://doc.rust-lang.org/book/ch17-02-trait-objects.html) to be able to return an Iterator of 2 different types.

```rust
fn get_ports(full: bool) -> Box<dyn Iterator<Item = u16>> {
    if full {
        Box::new((1..=u16::MAX).into_iter())
    } else {
        Box::new(ports::MOST_COMMON_PORTS_1002.to_owned().into_iter())
    }
}
```


## Scanning a single port

To scan a port, we will use the **TCP connect** technique, as it's the easiest one to implement and requires no special privilege or raw socket. [The 20% which brings us 80% of the results.](https://en.wikipedia.org/wiki/Pareto_principle)

```rust
async fn scan_port(target: IpAddr, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => println!("{}", port),
        _ => {}
    }
}
```



## Extreme concurrency

If you are a recurrent reader of this blog you should have guessed the perfect concurrency primitive for the task (if not, you can <a href="https://kerkour.com/subscribe/" target="_blank">subscribe here</a>): A [Stream](https://docs.rs/futures/0.3.16/futures/stream/trait.StreamExt.html#method.for_each_concurrent) :)


```rust
async fn scan(target: IpAddr, full: bool, concurrency: usize, timeout: u64) {
    let ports = stream::iter(get_ports(full));

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
        .await;
}
```

## Parsing CLI arguments

And finally, we need to parse the CLI arguments and all the configuration boilerplate to run our scanner:


```rust
use clap::{App, Arg};
use futures::{stream, StreamExt};
use std::{
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    time::Duration,
};
use tokio::net::TcpStream;

mod ports;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli_matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("target")
                .help("The target to scan")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("concurrency")
                .help("Concurrency")
                .long("concurrency")
                .short("c")
                .default_value("1002"),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Display detailed information")
                .long("verbose")
                .short("v"),
        )
        .arg(
            Arg::with_name("full")
                .help("Scan all 65535 ports")
                .long("full"),
        )
        .arg(
            Arg::with_name("timeout")
                .help("Connection timeout")
                .long("timeout")
                .short("t")
                .default_value("3"),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    let full = cli_matches.is_present("full");
    let verbose = cli_matches.is_present("verbose");
    let concurrency = cli_matches
        .value_of("concurrency")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1002);
    let timeout = cli_matches
        .value_of("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(3);
    let target = cli_matches.value_of("target").unwrap();

    if verbose {
        let ports = if full {
            String::from("all the 65535 ports")
        } else {
            String::from("the most common 1002 ports")
        };
        println!(
            "Scanning {} of {}. Concurrency: {:?}. Timeout: {:?}",
            &ports, target, concurrency, timeout
        );
    }

    let socket_addresses: Vec<SocketAddr> = format!("{}:0", target).to_socket_addrs()?.collect();

    if socket_addresses.is_empty() {
        return Err(anyhow::anyhow!("Socket_addresses list is empty"));
    }

    scan(socket_addresses[0].ip(), full, concurrency, timeout).await;

    Ok(())
}
```


```shell
$ cargo run --release -- kerkour.com
80
8080
8443
443
```


## Conclusion


Even if Rust is a strongly typed and compiled language, we just built a complex program as easily as if it's were in a scripting language, but way faster and way safer.

The next steps? Implementing [more port scanning strategies](https://nmap.org/book/man-port-scanning-techniques.html).


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_fast_port_scanner) (please don't forget to star the repo 🙏)
