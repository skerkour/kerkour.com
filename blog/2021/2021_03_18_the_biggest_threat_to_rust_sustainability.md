+++
date = 2021-03-18T07:00:00Z
title = "The biggest threat to Rust's sustainability"
type = "post"
tags = ["rust", "programming"]
authors = ["Sylvain Kerkour"]
url = "/the-biggest-threat-to-rust-sustainability"

[extra]
lang = "en"
+++

**Its fast-paced development cycles.**

> For more data points, please [go here](https://github.com/rust-lang/rust/blob/master/RELEASES.md), search for 'Compatibility Notes' and 'Language'.

I love Rust. I can build web servers, create web apps with WebAssembly, use it for embedded development, craft shellcodes, and above all, it reduced the number of bugs in my programs by order of magnitude. As an indie developer, it means that I can sleep far better without the fear of my servers burning ([I mean, not literally](https://www.ovh.com/world/news/press/cpl1787.fire-our-strasbourg-site)).

Unfortunately, there is **one** thing that makes me anxious about its future: the 6-week development cycle. It's, I believe, one of the causes of an unhealthy problem: feature bloat. It's also the cause, in my opinion, of another problem: the immaturity of the ecosystem.

**The compound effect of small additions is complexity. Uncontrolled complexity is fatal to any project.**

**I feel like a hamster in his wheel**: Will it ever end? As a rust developer, I need to remain up to date with the latest developments in order not to produce code that is obsolete as soon as it's released.

**Learning materials quickly become obsolete**: Programming in Rust today is entirely different than what was programming in Rust 1.5 years ago (mainly due to [async-await hitting stable](https://blog.rust-lang.org/2019/11/07/Async-await-stable.html)). Rust's learning curve is already high enough to discourage people from learning it, the obsolescence of tutorials and guides does not help.

**The signal/noise ratio is low**: Even if there are dedicated posts on the [official Rust blog](https://blog.rust-lang.org) to highlight the most notable features, it's hard to keep track of all the important features as they are lost among other less-relevant things.

**It feels unfocused**: I'm a fan of the [Pareto principle](https://en.wikipedia.org/wiki/Pareto_principle):  80% of consequences come from 20% of the causes. It brings calm and helps to identify busywork. Are all Rust's features high-value, or are some only the cause of incidental complexity? Who is in charge of saying no?

What I would like instead? 4 or 6 months release cycles and a more conservative approach to adding features. For example, instead of a `nightly` channel, I want a `playground` channel where a lot of features are experimented, and very few are promoted to `stable`.

A programming language is a platform. You can't build a thriving ecosystem on top of an unstable platform. We need, as a community, to find a solution to bring calm and focus to Rust, and then we will be able to stabilize and build a great ecosystem of libraries on top of that platform.

Short release cycles are suitable for products, as you can remove features or UI elements, and thus complexity can decrease over time. Removing parts of a platform will break backward compatibility. Thus platforms complexity with strict backward compatibility policies may only increase over time.

We don't want Rust to become the new C++, don't we?


<!-- Want to learn more from real-world Rust experience? I'm writing a book (available in early access) where I share everything I learned. Here is a coupon to save 10€ on the book: [https://academy.kerkour.com/black-hat-rust?coupon=BLOG](https://academy.kerkour.com/black-hat-rust?coupon=BLOG) -->


