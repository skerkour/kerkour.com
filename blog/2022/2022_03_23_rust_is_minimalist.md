+++
date = 2022-03-23T02:00:00Z
title = "Rust is minimalist"
type = "post"
tags = ["rust", "programming", "hacking", "minimalism"]
authors = ["Sylvain Kerkour"]
url = "/rust-is-minimalist"

[extra]
lang = "en"

comment ="""
"""
+++


<!-- Go vs Rust

je pense que Rust est plus minimalist car il est beaucoup plus polyvent et donc on a pas besoin de connaitre d'autre technos


aux premiers abords, Rust n'est pas du tout minimaliste. Designe par un commite, il a beaucoup de features qui font plus de mal que de bien (lien vers unicode identifiers...)

Designed by a committe


Cependant, sa polyvalence fera de vous un programmeur plus minimalist. -->

[Rust is designed by a committee](https://www.rust-lang.org/governance), by choice.

If you ever have managed a project, you should smell the unfocused monstrosity coming from 100 KM away.

And yet, after many years, I've come to the conclusion that in Rust's case, it's a huge asset instead of a liability.

Generics, [algebraic data types](https://doc.rust-lang.org/book/ch06-00-enums.html), macros, [cooperative scheduling](https://kerkour.com/cooperative-vs-preemptive-scheduling), associated type or even [non-ASCII identifiers](https://rust-lang.github.io/rfcs/2457-non-ascii-idents.html)...

It's unquestionable that Rust is packed with a lot of features, some useful, some useless, and, at first glance, may appear far from minimalist.

<!--

 and that in the long term, it may [cause its desmise](https://kerkour.com/the-biggest-threat-to-rust-sustainability) due to incidental complexity.
 -->


## But...

When I take a few steps back and start thinking about the bigger picture, I realize that, today, Rust is the first language that can **reliably** do everything well.

![Rust vs Status Quo](https://kerkour.com/2022/rust-is-minimalist/rust_is_minimalist.png)

That's why today I say that Rust is minimalist: by being able to focus on a single language and ecosystem, developers can boost their effectiveness.


Python, Node.JS, Ruby, PHP, Java, R, C++...

Most companies have polyglot codebases that are costing them big $$ and they are not even aware of the underlying reasons.

**Compound effect**: When you have 5 different languages... you inevitably implement the same things 5 times.

With a single language, you can have a single library of software packages that can be used across all the teams of the organization.


**Hiring**: What costs more: Hiring people for 5 different technologies or allowing people to move freely internally because the tech stack is the same everywhere?

**Tooling**: Related to talents, every programming languages and frameworks have their own workflows and specific tools. It takes time to train people and to update the tools when they are in use.

**Reliability**: When sh*t happens (it's not a matter of "if" but "when"), do you prefer to have everyone able to fix it, or to wait for the only experts on the topic to come back from their holidays?

Ans this is even before talking about correctness and performance.


Rust may not be a silver bullet, but now I ask you, what is best for an organization: spending resources on training and implementing the same things in 5 different programming languages, or focusing their efforts on a single, polyvalent language that can be used across a large panel of domains?
