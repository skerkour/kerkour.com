+++
date = 2022-10-04T06:30:00Z
title = "When to vendor (or not) your dependencies in Rust"
type = "post"
tags = ["hacking", "rust", "programming"]
authors = ["Sylvain Kerkour"]
url = "/rust-vendor-dependencies"

[extra]
lang = "en"
+++

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


Vendoring dependencies is the act of bundling all your dependencies with your code in your repositories.

Why would someone want to do that?

A first reason is for offline builds: when your dependencies are in your repository, you no longer depend on the availability of the dependencies registry ([crates.io](https://creates.io) or Git in the case of Rust), thus if for some reason the registry goes down, our you no longer have internet, you will still be able to build your program.

A second reason is privacy. Indeed, depending on an external registry induces a lot of privacy concerns for all the people and machines (your CI/CD pipeline, for example) that will build your code. Each time someone or something wants to build the project and doesn't have the dependencies locally cached, it has to contact the package registry, leaking its IP address, among other things. Depending on the location of those registries and the law they have to obey, they may block some countries.

A third reason is for audits. Indeed, when you vendor your dependencies, the updates of the dependencies now appear in git diff, and thus fit well in a code-review process. Dependencies updates can be reviewed like any other chunk of code.


But, vendoring dependencies has the disadvantage of increasing the size of your code repository by many Megabytes. And once a Git repository tracks a file, it's very hard to remove it from the history. For example, the `vendor` folder for the [end-to-end encrypted Remote Access Tool](https://github.com/skerkour/black-hat-rust/tree/main/ch_11) that we build in [chapter 11](https://kerkour.com/black-hat-rust/11) weights 305 MB!


An alternative is to use a [private registry](https://doc.rust-lang.org/cargo/reference/registries.html#running-a-registry), but it comes with a lot of maintenance and may only be a viable solution for larger teams.

In Rust, you can vendor your dependencies using the `cargo vendor` command.

**Want to learn more about Rust, applied Cryptography and Security? Take a look at my book [Black Hat Rust](https://kerkour.com/black-hat-rust).**
