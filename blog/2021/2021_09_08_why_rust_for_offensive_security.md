+++
date = 2021-09-08T13:00:00Z
title = "Why Rust for offensive security"
type = "post"
tags = ["hacking", "rust", "programming", "security"]
authors = ["Sylvain Kerkour"]
url = "/why-rust-for-offensive-security"

[extra]
lang = "en"

comment ="""

# Attention



# Interet
# Desire
# Action


Imaginez que notre armee soit faite de tanks de carton. Ce serait une situation plutot inconfortable non?

while it spunds absurb, thianis the sad state of hacking today.
you have to choose between fast and low level (c, c++) or slow and high level (python, ruby)
reusable

Et pourtant, c'est l'etat actuel de la securite offensive

why rust for offensive security


imagine un tank en carton. Il peut tirer, mais si on lui tire dessus...


Mais ce n'est pas tout...


quand on parcours les repos sur github d'offensive security, que ce soit des outils open source, ou des leaks (mettres des liens) On se rend compte tres vite que quelques chose ne va pas.

on observe une multitude de langages de l'assembleur au ruby et php, en passant par beaucoip de c, cpp et python.

cela mene a tres tres peu de transfert de skills, de failles de securite (!) et tres tres peu de reusabilite

mais c'etait avant rust. rust est LE languge to rule them all,here is why

seco de partie:
repondre aux objections evemtuelles

lifetimes are ugly and a big distractions

you dont need lifetimes, use smart pointers
"""
+++

**Imagine: all the tanks of your army are made of cardboard.** Now imagine that not only your tanks but also all your airforce is composed of paper planes and your navy of paper vessels. It would be a pretty bad situation, don't you think?

**While it sounds absurd, this is the sad state of hacking today.**

![A paper plane](https://kerkour.com/2021/why-rust-for-offensive-security/paperplane.jpg)

<div class="center">
  <i>A fighter jet, according to the Cybersecurity industry</i>
</div>

**Assembly, C, C++, Python, Java, Ruby...**

You have to choose between low-level, fast, but unsafe, or high-level, mostly safe but slow.

Can someone be an expert in all these languages? I don't think so. And the countless bugs and vulnerabilities in offensive tools prove I'm right.

What if, instead, we could have a unique language.

A language that once mastered, would fill all the needs of the field:
- Shellcodes
- Cross-platform Remote Access Tools (RATs)
- Reusable and embeddable exploits
- Scanners
- Phishing toolkits
- Embedded programming
- Web servers
- ...

What if we had a single language that is low-level enough while providing high-level abstractions, is exceptionally fast, easy to cross-compile, all of that while being memory safe, highly reusable and extremely reliable.

No more weird toolchains, strange binary packagers, vulnerable network code, injectable phishing forms...

You got it, **Rust is the language to rule them all**.

Due to momentum, Rust isn't widely adopted by the security industry yet, but once the tech leads and independent hackers understand this reality, the change will happen really fast.

This is why I dedicated the past months to write a book about the topic: [Black Hat Rust - Applied offensive security with the Rust programming language](https://kerkour.com/black-hat-rust).


While the [Rust Book](https://doc.rust-lang.org/book/) does a great job explaining **What** is Rust, Black Hat Rust is about **Why** and **How to** Rust.

Some say that Rust is ugly or too hard write. **This is false!** [You can write Rust code without lifetime annotations](https://kerkour.com/rust-avoid-lifetimes/). Actually, clean Rust code looks very similar to TypeScript while empowering the developers a thousand times more. And **I can prove it**: All the code accompanying the book is available on GitHub: [https://github.com/skerkour/black-hat-rust](https://github.com/skerkour/black-hat-rust)


In the book, we learn how to:
- Build fast and reusable network scanners
- Craft cross-platform shellcodes
- Code a Phishing toolkit with WebAssembly
- Create a cross-platform Remote Access Tool
- Design and implement an end-to-end encryption protocol
- And many other tips and tricks

But, more importantly, **I share what I learned through years of experience and thousands of lines of code, so you don't have to make the same costly (in time) mistakes as I did**.

Indeed, the book is designed to save you a lot of time in your Rust and offensive security learning journey. So if you understand the value of your time, and understand that, in order to write secure code you have to think like an attacker, this book is for you!


You can get the book here: [https://kerkour.com/black-hat-rust](https://kerkour.com/black-hat-rust).
