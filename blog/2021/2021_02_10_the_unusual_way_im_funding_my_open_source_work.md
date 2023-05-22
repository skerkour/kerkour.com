+++
date = 2021-02-10T12:42:42+01:00
title = "The unusual way I'm funding my open source work"
type = "post"
tags = ["bloom", "hacking", "open-source", "black-hat-rust", "entrepreneurship", "nomad"]
authors = ["Sylvain Kerkour"]
url = "/the-unusual-way-im-funding-my-open-source-work"

[extra]
lang = "en"
+++

From the Linux kernel to Firefox and Wordpress, Open Source is changing the world for the better. But how to achieve financial sustainability when you produce something that can legally be copied, by design, at zero cost?


<!-- {{< bhr_banner >}} -->

## Revenue models for Open Source

A [lot](https://news.ycombinator.com/item?id=23218943) has been written about [achieving profitability](https://plausible.io/blog/open-source-funding) for open source projects, still it’s not a solved problem. Here are the most common ways for open source projects to generate profit:

* Hosting: where you offer a fully-managed version of your project. It’s certainly the easiest way to scale revenues but it’s not appropriate for all products.
* Paying support: popularized by Red Hat and other Linux distributions, this model is hard to scale and create bizarre incentives.
* Patronage: patrons are helping artists make a living since forever. Unfortunately as you are depending on the goodwill of your patrons, it’s an extremely precarious way of life without a huge audience.
* [VC](https://en.wikipedia.org/wiki/Venture_capital): According to my empirical study it’s extremely hard to create a long term business whose interests are aligned with those of its users going this way.
* Having a real job™ beside: but is your project worth sacrificing all your free time for it?


## Bloom

Early in 2019, I innocently started working on [Bloom](https://github.com/skerkour/bloom) with one mission: putting an end to digital feudalism (Why? How? What? 👉 [You can read the launch post here](https://kerkour.com/bloom-a-free-and-open-source-google)).
The idea resonated with a lot of people and I received more positive feedback than I would ever have imagined.

But the path to success was still not clear: I launched the project as a self-hostable web application with a plan to create native applications with end-to-end encryption later. It quickly turned into a full-time R&D project as end-to-end encryption is not yet a totally solved problem (especially its key distribution part) and neither is native applications distribution (App stores being controlled by two companies that are more and more user hostile).

<img src="https://kerkour.com/2021/valley_of_death.jpg" />


So, how to survive the valley of death and stay alive enough time to reach product-market fit and turn Bloom into a sustainable open source project?



## Black Hat Rust

<a href="https://github.com/skerkour">
    <img src="https://kerkour.com/2021/github_contributions.png" />
</a>

While developing Bloom, I have written and deleted more than a hundred thousand lines of code of Rust. From achieving great development speed to designing and implementing an en-to-end encrypted synchronization protocol, passing by handling millions of HTTP requests a day, I’ve learned a few things along the way. <br />
Why not turn these thousand hours of experience into a profit which will allow me to arrange free hours during the week to work on Bloom? <br />
Some friends suggested that I launch an Udemy course, but I was not a fan of the idea because I find the ratio of information / time of these video courses too low compared to a good old book.

So I decided to write a book about Rust. As I had experience with offensive security (developing an automated vulnerability scanner, reverse engineering applications and helping plenty of companies to secure their assets) I narrowed the scope of the book to offensive security programming with Rust. <br />
Rust is not only a great fit for offensive programming, but the only one-size-fits-all programming language able to meet all the needs of this field: from shellcodes and exploits to servers, RATs and phishing pages in WebAssembly, Rust is polyvalent enough to shine in all those tasks. <br />
*Black Hat Rust - Deep dive into offensive security with the Rust programming language* was born!

[I announced the (extremely) early-access edition of the book on Reddit](https://www.reddit.com/r/rust/comments/lcow5j/black_hat_rust_im_writing_a_book_about_offensive) last week and received a lot of positive feedback. <br />
Idea validated. Funding secured. 🥳 <br />
Even if the hardest part is yet to come, I just want to say a big **THANK YOU** to all the people who are making this possible. <br />
Since this Monday I have started to work 4 days a week on the book and 2 days on Bloom (yes, as an indie hacker my workweek is 6 days long 🤷‍♂️).

If you find the idea compelling and want to be part of the adventure, you can get the book here: [https://kerkour.com/black-hat-rust](https://kerkour.com/black-hat-rust).

Sylvain ✌️


