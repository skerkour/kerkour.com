+++
date = 2022-03-30T02:00:00Z
title = "When not to use Rust?"
type = "post"
tags = ["rust", "programming"]
authors = ["Sylvain Kerkour"]
url = "/why-not-rust"

[extra]
lang = "en"

comment ="""
"""
+++

I sincerely believe that Rust is a huge step forward in terms of software reliability and performance, which directly translate to $$ and time saved. It solves a lot of problems that I face every day as a developer, such as immutability and good abstractions.

But like all technologies, it has drawbacks that may not make it the best choice for your project. Today I want to explore what I think are bad use cases for Rust.


## Fast prototyping & Hackathons

Let's be clear Rust favors reliability over speed of development. Thus if your time is limited to 1 or 2 days, you have better things to do than manually managing memory and handling each and every edge cases.

That being said, for projects lasting more than a few weeks, the reliability will make you save a lot of time and money due to fewer things to monitor and debug.


## Solo developers

Rust's ecosystem of libraries (crates) is still young. If you are a solo developer, you may want to outsource as many things as possible and thus Rust may not be the best fit. You may prefer languages with a huge ecosystem of ready-to-use libraries.

On the other hand, Rust's reliability alleviates solo developers from requiring extensive monitoring and debugging.


## When integrating with many SaaS services

If you are developing a service that integrates with many 3rd party APIs you may want to use another language that has a lot of official [SDKs](https://en.wikipedia.org/wiki/Software_development_kit), such as TypeSript or Python.

That being said, now that [AWS has an official SDK](https://github.com/awslabs/aws-sdk-rust), I expect more companies to follow the path and provide a Rust SDK.


## If you favor done over perfect

Let's be honest, not all software needs the reliability and performance offered by Rust. Sometimes, a few errors are acceptable and Rust is only going to slow you down. In these situations you will prefer more boring languages such as Go.
