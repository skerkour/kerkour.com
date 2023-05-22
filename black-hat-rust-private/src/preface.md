# Preface{.unnumbered}

Before we start, I would like to tell you that I regularly publish content that is complementary to this book in my newsletter.

Every week I share updates about my projects and everything I learn about how to (ab)use technology for fun & profit: Programming, Hacking & Entrepreneurship. You can subscribe by **Email or RSS**: [https://kerkour.com/subscribe](https://kerkour.com/subscribe).


Without further ado, let's hack!

<!--

Why this book?
Car le Rust book est bon pour apprendre le language, mais il manque de bonnes practices.
Rust est un language pretty vast, and so on peut vite faire n'importe quoi si on ne met pas en places quelques regles.

Car le monde du software est extremement insecure, et selon moi il n'y a pas d'autres moyens que de penser comme un attacker pour creer du software secure.


 -->

After high school, my plan for life was to become a private detective, maybe because I read too many Sherlock Holmes books. In France, the easiest way to become one is (was?) to go to law university and then to attend a specialized school.

I was not ready.

I quickly realized that studying law was not for me: reality is travestied to fit whatever narrative politics or professor wanted us to believe. No deep knowledge is taught here, only numbers, dates, how to look nice and sound smart. It was deeply frustrating for the young man I was, with an insatiable curiosity. I wanted to understand how the world works, not human conventions. For example, how do these machines we call computers that we are frantically typing on all day long work under the hood?


So I started by installing Linux (no, I won't enter the GNU/Linux war) on my Asus EeePC, a small netbook with only 1GB of RAM, because Windows was too slow, and started to learn to develop C++ programs with Qt, thanks to online tutorials. I coded my own text and my own chat systems. But my curiosity was not fulfilled.

One day, I inadvertently fell on the book that changed my life: "Hacking: The Art of Exploitation, 2nd Edition", by *Jon Erickson*.


This book not only made me curious about how to **make** things, but, more importantly, how to **break** things. It made me realize that you can't build reliable things without understanding how to break them, and by extension, where their weaknesses are.

While the book remains great to learn low-level programming and how to exploit simple memory safety bugs, today, hacking requires new skills: web exploitation, network and system programming, and, above all, how to code in a modern programming language.

Welcome to the fascinating world of Rust and offensive security.

While the [Rust Book](https://doc.rust-lang.org/book/) does an excellent job teaching **What is** Rust, I felt that a book about **Why** and **How** to Rust was missing. That means that some concepts will not be covered in-depth in this book. Instead, we are going to see how to effectively use them in practice.

In this book, we will shake the preconceived ideas (Rust is too complex for the real world, Rust is not productive...) and see how to architect and create real-world Rust projects applied to offensive security. We will see how polyvalent Rust is, which enables its users to replace the plethora of programming languages (Python, Ruby, C, C++...) plaguing the offensive security world with a unique language that offers high-level abstractions, high performance, and low-level control when needed.

<!-- In this book we will see the things that will get you 80-90% of the result. By that I mean -->

We will always start with some theory, deep knowledge that pass through ages, technologies and trends. This knowledge is independent of any programming language and will help you to get the right mindset required for offensive security.

I designed this book for people who either want to understand how attackers think in order to better defend themselves or for people who want to enter the world of offensive security and eventually make a living off it.


The goal of this book is to save you time in your path to action, by distilling knowledge and presenting it in applied code projects.

It's important to understand that *Black Hat Rust* is not meant to be a big encyclopedia containing all the knowledge of the world. Instead, it was designed as a guide to help you getting started and pave the way to **action**. Knowledge is often a prerequisite, but it's **action** that is shaping the world, and sometimes knowledge is a blocker for action (see [analysis paralysis](https://en.wikipedia.org/wiki/Analysis_paralysis)). As we will see, some of the most primitive offensive techniques are still the most effective. Thus some very specific topics, such as how to bypass modern OSes protection mechanisms won't be covered because there already is extensive literature on these topics, and they have little value in a book about Rust. That being said, I did my best to list the best resources to further your learning journey.

It took me approximately 1 year to become efficient in Rust, but it's only when I started to write (and rewrite) a lot of code that I made real progress.

Rust is an extremely vast language, but in reality, you will (and should) use only a subset of its features: you don't need to learn them all ahead of time. Some, that we will study in this book, are fundamentals. Others are not and may have an adversarial effect on the quality of your code by making it harder to read and maintain.

My intention with this book is not only to make you discover the fabulous world of offensive security, to convince you that Rust is the long-awaited one-size-fits-all programming language meeting all the needs of offensive security, but also to save you a lot of time by guiding you to what really matters when learning Rust and offensive security. But remember, knowledge is not enough. Knowledge doesn't move mountains. Actions do.

<!-- My intention is that the amount of time (and thus money) this book will save you, will make the purchase an absolute bargain. For example the scanner we will build in chapter 4 can be used with little to no modifications in a bug bounty program to make your first hundreds dollars with a subdomain takeover or similar vulnerability. -->

Thus, the book is only one half of the story. The other half is the accompanying code repository: [https://github.com/skerkour/black-hat-rust](https://github.com/skerkour/black-hat-rust/). **It's impossible to learn without practice, so I invite you to read the code, modify it and make it yours!**

If at any time you feel lost or don't understand a chunk of Rust code, don't hesitate to refer to the [Rust Language Cheat Sheet](https://cheats.rs), [The Rust Book](https://doc.rust-lang.org/book/), and the [Rust Language Reference](https://doc.rust-lang.org/stable/reference/).


Also, the book is code-heavy. I recommend reading it with a web browser aside, in order to explore and play with the code on GitHub: [https://github.com/skerkour/black-hat-rust/](https://github.com/skerkour/black-hat-rust/).



<!-- [The Rustonomicon](https://doc.rust-lang.org/nightly/nomicon/)  -->
