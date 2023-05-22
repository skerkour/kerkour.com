+++
date = 2021-12-07T06:00:00Z
title = "The tools and services I used to write, edit and self-publish my book"
type = "post"
tags = ["hacking", "security", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/book-self-publishing-pandoc"

[extra]
lang = "en"

comment ="""


Twitter: i've received a few questions about the tools and services I used to self-publish my book.
I've detailed everything in a blog post :)


HN: Also, I've received some questions about the tools I used to write, edit and publish the book, so I've detailed in a blog post.



Show HN: I wrote a book about Rust and Security

Hi HN,

Since February I've been working hard to write a book about Security and Rust, the 2 topics I'm working on full-time since 2017 and 2019 respectively.

Why Rust and security?

The world of software is plagued by too many programming languages with too many footguns. You have to choose between fast and unsafe (C, C++…) or slow but mostly safe (Python, Java…).

What if we could have language that is fast, memory safe, provide low-level controls and high-level abstractions? A language that could help us craft everything, from web servers to shellcodes, passing by scanners and Remote Access Tools. All of that while being cross-platform. Sounds too good to be true?

It’s not! Rust is the programming language that meets all these requirements. Of course, there are some pitfalls and a few things to know, but everything is covered in the book.


I've created a special coupon for the launch week that is available until Wednesday, December 8 at midnight: https://academy.kerkour.com/black-hat-rust?coupon=BLACK-HAT-WEEK ;)


Also, I've received a few questions about the tools and services I've used to self-publish the book so I wrote a detailed write-up :)

"""
+++



[Last week](https://kerkour.com/black-hat-rust-week-2021/), I officially announced that my Book [Black Hat Rust](https://kerkour.com/black-hat-rust) is out 🍾

Since, I received a few questions about the tools and services I used to self-publish it, so here is my end-of-the-year gift 🎁 to you: A detailed write-up of all the tools, services, and scripts I've used to self-publish my book. What worked, and what didn't.


As always, you can find the code on GitHub: [github.com/skerkour/black-hat-rust](https://github.com/skerkour/black-hat-rust/tree/main/extra/publishing) (please don't forget to star the repo 🙏).


## Writing


The entire manuscript is written in [Markdown](https://en.wikipedia.org/wiki/Markdown) (`.md` files).

Why use Markdown and not an office suite such as Google Docs, LibreOffice, Microsoft Word, or Apple Pages?

First, because it's hard to collaborate with these tools across different devices, Operating Systems, and people.

Second, because these tools are designed to produce office documents, not books. Programming books have the particularity of containing code, and it's impossible to produce good-looking code sections with office word processors.

Third, because I can use Git to backup and sync my Markdown files. More importantly, I can **work offline**, which is, as everyone knows, a prerequisite to enter [the flow](https://en.wikipedia.org/wiki/Flow_(psychology)).

Last, because I don't like the idea of imprisoning my work in the walled garden of tech giants. Markdown is a universal format and works with hundreds to thousands of different tools to make anything you want.

Do you want to create a PDF file from your Markdown notes? No problem. Do you want to create DOCX, EPUB, and HTML files? No problem either, there are tools for that. More about that below.


## Illustrations

All the illustrations are made on [Figma](https://www.figma.com), which is really great as I can use it from any device and share projects with other people.

I **always** export illustrations both in vector (`.svg`) and raster (`.png`, `.jpg`) formats because this is a cloud service, and like all cloud services, may go down or lose my data. Thus, by exporting the `.svg` files, I have the original source files as backups.


## Editing

As a non-native English speaker, I make tons and tons of mistakes. This is why I use [Grammarly](https://www.grammarly.com) to spot typos and simple grammatical errors.

The service is faaaar from perfect, but, at least, it will prevent you from passing for a clown and is well worth the price. It also sometimes helps to find the right words instead of poorly translated ones.

I don't use their invasive browser extension and prefer instead to copy/paste my text into their web application and correct it there.



## Generating the EBooks

I use [pandoc](https://pandoc.org/) to turn my markdown files and illustrations into PDF and EPUB files.

How to say it.

Pandoc is both great and awful.

It's great because it does the job. But, it uses under the hood a technology called [LaTeX](https://www.latex-project.org/) that is far from intuitive. Fortunately, I didn't really have to touch it other than for tweaking the template of the book generated by pandoc.

I couldn't find a way to add a cover to the PDF ebook directly from `pandoc`, so I used the `pdftk` command-line tool to merge 2 PDFs: One that is the cover exported from Figma and the other that is the content of the book generated by pandoc.

Pandoc itself can't generate MOBI files (for Kindle devices). For that I use the `ebook-convert` command-line tool that can be installed alongside the [calibre ebook manager](https://calibre-ebook.com).

Finally, I use `epubcheck` to verify that the EPUB file doesn't contain any error.


### My Docker image

LaTeX is a real nightmare to install and maintain. And saying that is a compliment, I've lost countless hours fighting with the dependencies!

Here is a Dockerfile that will save you a lot of time:


```dockerfile
FROM ubuntu:latest

RUN apt update
RUN apt upgrade -y

# Create ebook user
ENV USER=ebook
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


# Install dependencies
ENV DEBIAN_FRONTEND noninteractive
RUN apt install -y vim calibre pdftk epubcheck binutils make wget imagemagick

RUN apt install -y pandoc libpar-packer-perl perl-doc zlib1g zlib1g-dev expat \
    texlive-latex-base texlive-latex-extra texlive-xetex texlive librsvg2-bin \
    texlive-fonts-recommended texlive-fonts-extra texlive-xetex texlive-latex-recommended


USER ebook:ebook

WORKDIR /ebook

CMD ["make", "all"]
```




You can find all the other configuration files such as the `Makefile` on GitHub: [github.com/skerkour/black-hat-rust](https://github.com/skerkour/black-hat-rust/tree/main/extra/publishing) (please don't forget to star the repo 🙏).



## Publishing

Before writing Black Hat Rust, I thought that selling digital goods on the internet was just a matter of subscribing to Shopify. It turns out that not at all!

First, because it seems that you can't sell digital products on Shopify without third-party add-ons that make the whole operation complex.

Second, because as a European business, I have to deal with the complete sh*t that European VAT is. I need to apply the correct rate depending on the country of the buyer and depending on if the buyer is an individual or a business with a VAT number. All of this should be handled at the time of the sale.

And if you thought that it was too easy, let me just say that the VAT rate of some countries changed during the pandemic... Don't ask me why Europe has some innovation problems 🤷‍♂️

Fortunately, I've found a little gem that was specifically designed to sell digital goods: [Podia](https://www.podia.com). It handles VAT and integrates directly with Stripe for secure payments processing. Then you can connect Stripe to most accounting software.

The pricing is fair, the interface is clean, everyone is happy.


### Amazon KDP

Because some people intereseted by the book asked, I've tried to publish the book on Amazon through their program [Amazon Kindle Direct Publishing (KDP)](https://kdp.amazon.com). But again, it was not as simple as I thought.

The first thing is that it would add me a lot of US tax forms to fill and nobody enjoy that. If it was only that, I could have surpassed my paperwork phobia.

But, the second, and most important thing, is the cut Amazon takes. You thought that the 30% cut Apple takes on the App Store is huge? Amazon takes 70% (for a book that costs more than $9.99 or less than $2.99). And that’s before taxes. While I appreciate the buying experience on Amazon's marketplace, I respect my work and myself. I’m sorry Amazon, I didn't work so hard on this book to give you the biggest piece of the cake just to move a few bits here and there 🖕.



## Some Closing Thoughts

Writing and publishing a book is a great experience! A big thanks to all the nice people who helped me make this book a reality 🙏

<!-- On the other hand the existing tools are a pain to use. -->

A nice workflow improvement would be to use an iPad pro with the [iA Writer](https://ia.net/writer) + [Working Copy](https://workingcopyapp.com/) apps for offline and deep focused work, all while keeping the ability to push files to Git. That being said, [I don't like single-purpose devices](https://kerkour.com/2021/fight-climate-change-as-a-technologist/), so it won't happen.

**Interested in learning Rust and Security? Get [Black Hat Rust here](https://academy.kerkour.com/black-hat-rust?coupon=BLOG)**.

## The code is on GitHub

As always, you can find the code on GitHub: [github.com/skerkour/black-hat-rust](https://github.com/skerkour/black-hat-rust/tree/main/extra/publishing) (please don't forget to star the repo 🙏).
