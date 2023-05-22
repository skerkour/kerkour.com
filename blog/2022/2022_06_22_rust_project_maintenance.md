+++
date = 2022-06-22T01:00:00Z
title = "Automated maintenance and dependencies security for Rust projects"
type = "post"
tags = ["hacking", "programming", "rust"]
authors = ["Sylvain Kerkour"]
url = "/rust-projects-maintenance-and-supply-chain-security"

[extra]
lang = "en"

comment ="""
"""
+++


It's an open secret that most of the time and costs spent on any serious software project are from maintenance. Rust is moving fast, and its ecosystem too, so it's necessary to automate your projects' maintenance.

The good news is that, in my experience, due to its strong typing, Rust project maintenance is easier than in other languages: errors such as API changes will be caught at compile time.

For that, the community has built a few tools which will save you a lot of time to let you keep your projects up to date.


> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


#### Rustup

Update your local toolchain with `rustup`:

```bash
$ rustup self update
$ rustup update
```


#### Rust fmt

`rustfmt` is a code formatter that allows codebases to have a consistent coding style and avoid nitpicking during code reviews.

It can be configured using a `.rustfmt.toml` file: [https://rust-lang.github.io/rustfmt](https://rust-lang.github.io/rustfmt).

You can use it by calling:
```bash
$ cargo fmt
```

In your projects.


#### Clippy

`clippy` is a [linter](https://en.wikipedia.org/wiki/Lint_(software)) for Rust. It will detect code patterns that may lead to errors or are identified by the community as bad style.

It helps your codebase to be consistent and reduce time spent during code reviews discussing tiny details.

It can be installed with:
```bash
$ rustup component add clippy
```

And used with:
```bash
$ cargo clippy
```


#### Cargo update

```bash
$ cargo update
```

Is a command that will automatically update your dependencies according to the [semver](https://semver.org/) declaration in your `Cargo.toml`.


#### Cargo outdated

[`cargo-outdated`](https://github.com/kbknapp/cargo-outdated) is a program that helps you to identify your outdated dependencies that can't be automatically updated with `cargo update`

It can be installed as follows:
```bash
$ cargo install -f cargo-outdated
```

The usage is as simple as running
```bash
$ cargo outdated
```
In your projects.


#### Cargo audit

Sometimes, you may not be able to always keep your dependencies to the last version and need to use an old version (due to dependency by another of your dependency...) of a crate. As a professional, you still want to be sure that none of your outdated dependencies contains any known vulnerability.

[`cargo-audit`](https://github.com/RustSec/rustsec/tree/main/cargo-audit) is the tool for the job. It can be installed with:
```bash
$ cargo install -f cargo-audit
```

Like other helpers, it's very simple to use:
```
$ cargo audit
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 317 security advisories (from /usr/local/cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (144 crate dependencies)
```

