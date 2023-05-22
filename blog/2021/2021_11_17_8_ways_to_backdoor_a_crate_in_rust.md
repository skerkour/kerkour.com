+++
date = 2021-11-17T06:00:00Z
title = "Backdooring Rust crates for fun and profit"
type = "post"
tags = ["hacking", "security", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-crate-backdoor"

[extra]
lang = "en"

comment ="""
https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros

"""
+++





Supply chains attacks are all the rage these days, whether to deliver RATs, cryptocurrencies miners, or credential stealers.

In Rust, packages are called crates and are (most of the time) hosted on a central repository: [https://crates.io](https://crates.io/) for better discoverability.

We are going to study 8 techniques to achieve Remote Code Execution (RCE) on developers', CI/CD, or users' machines. I voluntarily ignored perniciously backdoored algorithms such as cryptographic primitives or obfuscated code because this is a whole different topic.

**The goal of this post is to raise awareness among developers about how easy it's to carry these kinds of attacks and how pernicious they can be.**

Of course, an attacker can combine these techniques to make them more effective and stealthy.

> Interested in Security and Rust? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


**Contents:**
- [Typosquatting](#typosquatting)
- [Misleading name](#misleading-name)
- [Transitive dependencies](#transitive-dependencies)
- ["x.x.1" Update](#xx1-update)
- [Malicious update](#malicious-update)
- [Run code before main](#run-code-before-main)
- [Malicious macros](#malicious-macros)
- [build.rs](#buildrs)
- [Some Closing Thoughts](#some-closing-thoughts)
- [The code is on GitHub](#the-code-is-on-github)




## Typosquatting


By naming a crate in a very similar way to a popular one, we can expect that a non-zero number of developers will make a typo in the name, either when searching on `crates.io` or when installing the crate.

As an example, I just published the crate [num_cpu](https://crates.io/crates/num_cpu) which targets the [num_cpus](https://crates.io/crates/num_cpus) crate with almost 43,000,000 downloads.

When you look at both crates on `crates.io`, it's very hard to tell which one is legitimate and which one is malicious.


![num_cpu on crates.io](https://kerkour.com/2021/rust-crate-backdoor/num_cpu.png)

Actually, my [num_cpu](https://crates.io/crates/num_cpu) crate has been downloaded 24 times in less than 24 hours, but I'm not sure if it's by bots or real persons (I didn't embed any active payload to avoid headaches for anyone involved).



How to know if a crate is legitimate or not?

It's hard! You can look at the **Owners** section or the total number of downloads.

But still, this is not perfect: I could have made up my `crates.io` profile in order to look like a famous developer.


## Misleading name

All crates on `crates.io` live under a global namespace, which means no organizational scoping.

Thus, organizations, projects, and developers rely on prefixes to make their packages discoverable and group them. `tokio-stream` or `actix-http`, for example.

Problem: Anyone can upload a package with a given prefix. For example, I just uploaded the crate `tokio-backdoor`. While It's hard to have a more explicit name, imagine if I would have named this crate `tokio-workerpool` or `tokio-future`.


By using misleading metadata such as the `README`, the `repository`, and `tags`, an attacker can make this crate appear like an official one.


How to detect these scams?

Again, it's hard!



## Transitive dependencies

By burying a backdoored dependency deep in the dependency tree, an attacker can conceal a backdoored crate.

The chance of a code review of **all** the transitive dependencies is approximately 0.


For example, let say I want to backdoor a popular crate. I can make a Pull Request with a new dependency, let say `tokio-helpers`. The trick is that it's not `tokio-helpers` that is backdoored, it's a dependency of a dependency of a ... of `tokio-helpers`.


## "x.x.1" Update


By issuing an `x.x.1` update, an attacker can compromise all the maintainers relying on `cargo update` to update their dependencies. From `1.12.0` to `1.12.1` or `0.5.13` to `0.5.14` for example.

Due to how semantic versioning works, a maintainer relying on `cargo update` to keep their dependencies up to date is going to install the compromised version.


This technique does not necessarily require the cooperation of the crate author. An attacker only needs a `crates.io` token, which could have been stolen from a previous compromise.


How to protect?

By pinning an exact version of a dependency, `tokio = "=1.0.0"` for example, but then you lose the bug fixes.



## Malicious update

A variant of the previous technique is to use the `--allow-dirty` flag of the `cargo publish` command.

By doing that, in conjunction with a `x.x.1` update, for example, an attacker can publish a crate on `crates.io` without having to commit the code in a public repository.

Where it becomes vicious is that it's totally possible to make Git tags and `crates.io` versions match while the code is different! There are absolutely no guarantees that the code on crates.io matches the code on GitHub, even if the tags and version numbers match!

How to protect?

A method to protect is to vendor your dependencies (with `cargo vendor`) and carefully audit the diffs for each update.


## Run code before main

One of the principles of Rust is [no life before main](https://doc.rust-lang.org/1.4.0/complement-design-faq.html#there-is-no-life-before-or-after-main-(no-static-ctors/dtors)), yet it's still possible to run code before main by abusing how executables work.

Put in another way, **it's possible to run code without calling it**.

It can be done by using the `.init_array` section on Linux or FreeBSD, `__DATA,__mod_init_func` section on macOS / iOS and the `.ctors` or `.CRT$XCU` sections on Windows.

Here is an example extracted from the [`startup`](https://github.com/thomcc/startup/blob/main/src/lib.rs) crate:
```rust
#[macro_export]
macro_rules! on_startup {
    ($($tokens:tt)*) => {
        const _: () = {
            // pulled out and scoped to be unable to see the other defs because
            // of the issues around item-level hygene.
            extern "C" fn __init_function() {
                // Note: currently pointless, since even when loaded at runtime
                // via dlopen, panicing before main makes the stdlib abort.
                // However, if that ever changes in the future, we want to guard
                // against unwinding over an `extern "C"` boundary, so we force
                // a double-panic, which will trigger an abort (rather than have
                // any UB).
                let _guard = $crate::_private::PanicOnDrop;
                // Note: ensure we still forget the guard even if `$tokens` has
                // an explicit `return` in it somewhere.
                let _ = (|| -> () { $($tokens)* })();
                $crate::_private::forget(_guard);
            }
            {
                #[used]
                #[cfg_attr(
                    any(target_os = "macos", target_os = "ios", target_os = "tvos"),
                    link_section = "__DATA,__mod_init_func",
                )]
                // These definitely support .init_array
                #[cfg_attr(
                    any(
                        target_os = "linux",
                        target_os = "android",
                        target_os = "freebsd",
                        target_os = "netbsd",
                    ),
                    link_section = ".init_array"
                )]
                // Assume all other unixs support .ctors
                #[cfg_attr(all(
                    any(unix, all(target_os = "windows", target_env = "gnu")),
                    not(any(
                        target_os = "macos", target_os = "ios",
                        target_os = "tvos", target_os = "linux",
                        target_os = "android", target_os = "freebsd",
                        target_os = "netbsd",
                    ))
                ), link_section = ".ctors")]
                #[cfg_attr(all(windows, not(target_env = "gnu")), link_section = ".CRT$XCU")]
                static __CTOR: extern "C" fn() = __init_function;
            };
        };
    };
}
```

Then, we can backdoor a crate like this:

**[lib.rs](https://github.com/skerkour/black-hat-rust/blob/main/extra/backdoors/init/backdoored_crate/src/lib.rs)**
```rust
pub fn do_something() {
    println!("do something...");
}

startup::on_startup! {
    println!("Warning! You just ran a malicious package. Please read https://kerkour.com/rust-crate-backdoor for more information.");
}
```

Any crate using the backdoored crate is compromised, even if it's a dependency of a dependency of a ...:

**[main.rs](https://github.com/skerkour/black-hat-rust/blob/main/extra/backdoors/init/app/src/main.rs)**
```rust
fn main() {
    backdoored_crate::do_something();
}
```



## Malicious macros

Rust's macros is code that runs at compile or `cargo check` time. Can it be abused?

It turns out that **yes**! The ability to run code at compile time means that any of your dependencies can download malware or exfiltrate files from your computer.

This risk is amplified by the fact that `rust-analyzer` also expands the macros when loading a project, thus **a machine can be compromised just by opening with a code editor (with the `rust-analyzer` plugin) a folder of a crate whose one of its dependencies is backdoored .**

Whether it be a direct or an indirect dependency!

These attacks are particularly juicy for attackers because develpoers' and CI/CD machines (the targets of these attacks) often hold credentials that they can use to pivot or spread more malware.


Here are two examples of malicious macros.

First, an [Attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros):

**[lib.rs](https://github.com/skerkour/black-hat-rust/blob/main/extra/backdoors/bad_macro/malicious_macro/src/lib.rs)**
```rust
use proc_macro::TokenStream;
use std::path::Path;

fn write_warning(file: &str) {
    let home = std::env::var("HOME").unwrap();
    let home = Path::new(&home);
    let warning_file = home.join(file);

    let message = "Warning! You just ran a malicious package. Please read https://kerkour.com/rust-crate-backdoor for more information.";
    let _ = std::fs::write(warning_file, message);
}

#[proc_macro_derive(Evil)]
pub fn evil_derive(_item: TokenStream) -> TokenStream {
    write_warning("WARNING_DERIVE");

    "".parse().unwrap()
}
```

Which once used by a crate is enough to compromise it and all its dependents.

**[lib.rs](https://github.com/skerkour/black-hat-rust/blob/main/extra/backdoors/bad_macro/backdoored_lib/src/lib.rs)**
```rust
use malicious_macro::Evil;

#[derive(Evil)]
pub struct RandomStruct {}
```


Then, a [function-like procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros):

**[lib.rs](https://github.com/skerkour/black-hat-rust/blob/main/extra/backdoors/bad_macro/malicious_macro/src/lib.rs)**
```rust
#[proc_macro]
pub fn evil(_item: TokenStream) -> TokenStream {
    write_warning("WARNING_MACRO");

    "".parse().unwrap()
}
```

Again, if any of your (transitive or not) dependencies call this macro, it's enough for a compromise **at compile-time**.

**[lib.rs](https://github.com/skerkour/black-hat-rust/blob/main/extra/backdoors/bad_macro/backdoored_lib/src/lib.rs)**
```rust
pub fn do_something() {
    println!("do something...");
}

malicious_macro::evil!();
```

**main.rs**
```rust
fn main() {
    lib::do_something();
}
```


{{< subscribe_form >}}


## build.rs

Like malicious macros, `build.rs` is run by `cargo check` and `rust-analyzer`. Thus, opening with a code editor the folder of a crate that has one of its dependencies backdoored is enough to compromise a machine.

~~While it's possible to audit the code of a crate on `https://docs.rs` on clicking on a `[src]` button, it turns that I couldn't find a way to inspect `build.rs` files. Thus, combined with a malicious update, it's the almost perfect backdoor.~~

It's [actually possible](https://github.com/skerkour/black-hat-rust/issues/29) to inspect `build.rs` files on [docs.rs](https://docs.rs) by using the source view: `https://docs.rs/crate/[CRATE]/[VERSION]/source/`. Thanks [Joshua](https://github.com/jyn514) 🙏


**[build.rs](https://github.com/skerkour/black-hat-rust/blob/main/extra/backdoors/buildrs/backdoored_lib/build.rs)**
```rust
use std::path::Path;

fn main() {
    let home = std::env::var("HOME").unwrap();
    let home = Path::new(&home);
    let warning_file = home.join("WARNING_BUILD");

    let message = "Warning! You just ran a malicious package. Please read https://kerkour.com/rust-crate-backdoor for more information.";
    let _ = std::fs::write(warning_file, message);
}
```


This technique is less stealth than malicious macros as `build.rs` files are displayed during the compilation process and are easier to spot.


## Some Closing Thoughts


As Rust is designed for sensitive applications where reliability is important such as embedded, networking or blockchain-like projects, it can raise concerns.

Also, while favoring small and reusable software packages may be philosophically appealing, it has serious practical implications.


Finally, let's be honest, who has the resources to carefully audit each one of their dependencies (including the transitive ones), for each update?


I see 3 main axes to reduce the impact and the risks associated with these kinds of attacks.

Firstly, a bigger standard library would reduce the need for external dependencies and thus reduce the risk of compromise.


Secondly, Rust supports [`git` dependencies](https://kerkour.com/rust-import-crate-from-git/).  Using Git dependencies pinned to a commit can prevent some of the techniques mentioned above.


Thirdly, using cloud developer environments such as [GitHub Codespaces](https://github.com/features/codespaces) or [Gitpod](https://www.gitpod.io/). By working in sandboxed environments for each project, one can significantly reduce the impact of a compromise.


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/black-hat-rust](https://github.com/skerkour/black-hat-rust/tree/main/extra/backdoors) (please don't forget to star the repo 🙏).
