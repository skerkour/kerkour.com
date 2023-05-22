+++
date = 2022-03-02T03:00:00Z
title = "Mental models for learning Rust"
type = "post"
tags = ["rust", "programming", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/rust-mental-models"

[extra]
lang = "en"

comment ="""
"""
+++


Let us not beat around the bush: Rust is not easy to learn.

I think it took me nearly 1 year of full-time programming in Rust to become proficient and no longer have to read the documentation every 5 lines of code. It's a looong journey but absolutely worth it.


It requires you to re-think all the mental models you learned while using other programming languages.

This is why I thought it could be interesting to share how I adapted my programming habits when working with Rust along the years.


> This post contains excerpts from my book [Black Hat Rust](/black-hat-rust) about Security, Rust and Cryptography.


## Embrace the compiler

The compiler will make you hard times when starting Rust. You will hate it. You will swear. You will wish to disable it and send it to hell. Don't.

The compiler should be seen as an always available and friendly code-reviewer. So it's not something preventing your code from compiling. Instead, it's a friend that tells you that your code is defective and even offers suggestions on how to fix it.

I have witnessed a great improvement over the years of the messages displayed by the compiler, and I have no doubts that if today the compiler produces an obscure message for an edge case, it will improve in the future.


## Just In Time learning

Rust is a vast language that you won't be able to master in a few weeks. And that's totally fine. You don't have to know everything to get started.

I've spent a lot of time reading about all the computer science behind Rust before even writing my first program. **This was the wrong approach**. There is too much to read about all the features of Rust, and you certainly won't use them all (and you shouldn't! For example, please **never ever** use [non_ascii_idents](https://rust-lang.github.io/rfcs/2457-non-ascii-idents.html) it will only bring chaos and pain!). All this stuff is really interesting and produced by very smart people, but it prevents you from getting things done.

Instead, embrace the unknown and make your first programs. Fail. Learn. Repeat.


## Keep it simple

**Don't try to be too clever!** If you are fighting with the limits of the language (which is already huge), it may mean that you are doing something wrong. Stop what you are doing, take a break, and think about how you can do things differently. It happens to me almost every day.

Also, keep in mind that the more you are playing with the limits of the type system, the more your code will create hard-to-understand errors by the compiler. So, make you and your co-workers a favor: **KISS (Keep It Simple, Stupid)**.

**Favor getting things done rather than the perfect design that will never ship**. It's far better to re-work an imperfect solution than to never ship a perfect system.



## You pay the costs upfront

Programming in Rust may sometimes appear to be slower than in Python, Java, or Go. This is because, in order to compile, the Rust compiler requires a level of correctness far superior to other languages. Thus, in the whole lifetime of a project, Rust will save you a **lot** of time. All the energy you spend crafting a correct program in Rust, is 1x-10x the time (and money and mental health!) you save when you **won't** have to spend hours and hours debugging weird bugs.

The first programs I shipped in production were in TypeScript (Node.js) and Go. Due to the lax compilers and type systems of these languages, you have to add complex instrumentation to your code and external services to detect errors at runtime. In Rust, I've never had to use this. Simple logging (as we will see in chapter 4) is all I ever needed to track bugs in my programs. Aside from that, as far as I remember, I've never experienced a crash in a production system in Rust. This is because Rust forces you to "pay the costs upfront": you have to handle every error and be very intentional about what you are doing.

Here is another testimony from ["jhgg"](https://news.ycombinator.com/item?id=26228798), Senior Staff Engineer at Discord:
*"We are going hard on Rust in 2021 after some very successful projects in 2019 and 2020. our engineers have ramped up on the language - and we have good support internally (both in terms of tools, but also knowledge) to see its success. Once you've passed the learning curve - imo, Rust is far easier and more productive to write than go - especially if you know how to leverage the type system to build idiomatic code and apis that are very hard to use incorrectly. Every piece of rust code we have shipped to production so far has gone perfectly thanks to the really powerful compile time checks and guarantees of the language. I can't say the same for our experiences with go. Our reasons go well beyond "oh the gc in go has given us problems" but more like "go as a language has willingly ignored and has rejected advances in programming languages". You can pry those algebraic data types, enums, borrow checker, and compile time memory management/safety, etc... from my cold dead hands. [...]"*



<!-- ### Ownership and memory -->



## Functional programming

Rust is (in my opinion) the perfect mix between an imperative and a functional language to get things done. It means that if you are coming from a purely imperative programming language, you will have to unlearn some things and embrace the functional paradigm.

Favor iterators (chapter 3) over `for` loops. Favor immutable data over mutable references, and don't worry, the compiler will do a great job optimizing your code.



## A few more tips

Use the automated tools to assist you in the maintenance of your projects: `rustfmt`, `clippy`, `cargo update`, [`cargo outdated`](https://github.com/kbknapp/cargo-outdated) and [`cargo-audit`](https://github.com/RustSec/rustsec/tree/main/cargo-audit).


**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where, from theory to practice, you will learn Rust, Security, and Cryptography.**
