+++
date = 2022-05-11T02:00:00Z
title = "Programming languages are platforms, not products"
type = "post"
tags = ["programming", "rust"]
authors = ["Sylvain Kerkour"]
url = "/programming-languages-are-platforms"

[extra]
lang = "en"

comment ="""
"""
+++


A few weeks ago, I wrote: *"["I believe that Rust moves too fast](https://kerkour.com/what-a-better-rust-would-look-like)"* and *"a programming language is a platform"*.

Some people raised good objections, so I thought it would be good to write a follow-up to explain my thoughts.

Nowadays, everyone wants to be agile, move fast, and break things where the goal is to ship more features than the competition, at the expense of stability and robustness.

While this model may be good for some kinds of products, for others, it's a huge liability.

Let's see why.


## Platform vs Products

Products are intended to be consumed by end-users. Users love to see new features and improvements in their products: A better camera for their phone, a new filter in their social media app (but not too much, otherwise they become confused).

On the other hand, platforms are intended to be used by builders that will create other platforms or products. You don't want to change your credit card every month because of "security upgrades" of the underlying payment infrastructure, you don't want to change your car every year because it relies [on a network technology that is moving too fast](https://www.zdnet.com/home-and-office/networking/3g-is-shutting-down-here-are-the-gadgets-that-still-rely-on-it-do-you-have-one/), and you don't want to change all your cables, adapters and chargers every week because the USB standard is evolving too fast.


Thus, **products need to move fast and can sometimes break things, while platforms need to move slowly and keep backward compatibility**.


## Programming languages are platforms

So, are programming languages products or platforms?

In his great post *[How often does Rust change?](https://steveklabnik.com/writing/how-often-does-rust-change)*, [Stevek Klabnik](https://steveklabnik.com/writing/how-often-does-rust-change) presents the following graph, which shows the changes in the Rust programming language:

![How often does Rust change](https://kerkour.com/2022/programming-languages-are-platforms/rust_changes.png)

It seems reasonable at first glance.

The thing is, if you want to maintain backward compatibility, changes in a programming language compound over time, they are not distinct.

Thus, I think that the previous graph is hiding the truth. The real graph would be more like that:

![Features compound over time](https://kerkour.com/2022/programming-languages-are-platforms/compounded.jpg)



What does it mean in practice?

First, like communication channels increase exponentially as team members increase, complexity balloons when the number of features increases due to how they interact with each other.

![Features and complexity](https://kerkour.com/2022/programming-languages-are-platforms/features_complexity.jpg)


Second, learning materials become obsolete faster: books, internal wikis, and online tutorials. It's frustrating to learn something that is no longer true due to a new version of the language, or an old best practice that is no longer one. The corollary is that developers also need to spend more time updating their skills and following the trends of the day which has a real cost in terms of mental energy and $$.


Third, developers need to update their toolchains and code more often, which is never taken into account when planning roadmaps... Nobody like to update all their Docker images and local toolchains `FROM rust:1.x` to `FROM rust:1.y` every 6 weeks while they already have 10 Jira tickets to complete.


This is why programming languages are platforms: programmers use them to build other platforms or products, and when they move too fast, everybody loses.


What is the worst-case scenario? That different teams or developers collaborating on a same project use completely different features of the same language and are no longer able to understand and maintain each others' code.

Producing software is already complex enough. We don't need our toolchains to increase this complexity.


## How fast a programming language should evolve?

It's a tradeoff between **how often** and **how much**.

I think that Rust [moves too fast](https://blog.rust-lang.org/), but I also think that C++ moves too slowly with big releases every few years with so many new features that most teams I've heard about are still using C++11 because it's good enough.

I believe that the Go model of a new release every 6 months with mostly minor changes is reasonable. Developers can anticipate and prepare for new features, and when a new version is released, they most of the time don't need to learn anything new nor update their code.
