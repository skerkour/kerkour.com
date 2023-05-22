## linter code with clippy


##


## Jörn Bexxx <bxx@posteo.de>


```
Hi Sylvain,

I finished the book! I liked it and it was interesting to see how someone in
infosec works with Rust. I think the focus on this topic makes this book really
unique.

I also like to give a bit of criticism, so that you can improve these things in
your next book.

1. There is code in the Book that is not Rust. For example PHP. It would
be really good if you marked those code sections accordingly. It seems
that people who are new to Rust are also your target audience and this
could really confuse people when they mix up PHP with Rust ;-)

2. Infosec encompassed a lot of different topics like cryptography,
social engineering, low level technical details, ... The book tries to
give short introductions into such topics, but I personally think you
rarely hit the right amount of explanations: In most cases you provide
too many unnecessary details and you do not explain how these details
fit together in the bigger picture. Of course, all these details can be
useful in other contexts. But learning is most effective, when the
reader can place the new knowledge into one big meaningful picture. And
the picture is Blackhat Rust. Not cryptography or social engineering or
something else. I know it is cool to share knowledge and you certainly
want to give your readers enough for the money they paid. But I think
this book would benefit a lot more from being more focussed on just the
topic blackhat Rust even if it means that the book is shorter. A more
focussed book creates a more pleasant reading and learning experience.

You can always add appendixes if you think there's a topic that needs
more explanations when the topic doesn't fit into the main text.

3. Comments about the text:
   Section 6.29.1. The word "fastidious" might be a wrong choice.
   Section 8.10 "whit"
   Section 11.4 "On the other hand, two different messages should
   never produce the same hash". Hash collisions are probably out of
   scope for the book, but I think you need to be a bit more precise.
   Maybe add "with very high probability" at the end?


Thanks again for the book and best wishes for your future writing
adventures!

Jörn
```


## https://github.com/tylerhjones

```
Chapter 2 skips over the models file, and several external libraries. It might be important to

    explain adding these external libraries and include the import lines demonstrating how to import / add external libraries.
    local crate references in imports, how to import files in a project.
    project hierarchy and structures. (at this point ch2. i wouldnt expect a real tree structure, but maybe address the models file and its common practices)
    adding deps with cargo
    explain the rayon crate
    explain the serde crate
    explain the reqwest crate

Very beginner things, but it is chapter 2. I would expect these things to be explained in a book which advertises learning rust along the way.


either this, or mention in the chapter that you expect the reader to supplement the examples by reading the repo source and that the book does not include all information; ellipsis things like imports, crates, etc.
```
