+++
title = "Technical Writer - Publishing"
date = 2021-06-01T6:00:00Z
type = "page"
url = "/technical-writer/publishing"
access = "paid_members"

+++


<!--
Pandoc
ebook converter
Dockerfile

you don't want to reach a large number of people for your first book
you want to have the biggest impact on a small number of person

* https://news.ycombinator.com/item?id=31394719 (The Tools I Use to Write Books (2018))

https://gumroad.com/

* Famous technical publishers
  * https://www.oreilly.com/work-with-us.html
    * https://www.quora.com/How-rich-is-the-average-author-of-an-OReilly-book-Does-writing-a-book-make-them-a-lot-of-money-The-typical-bio-reveals-the-author-is-relatively-young-male-a-nerd-contributes-to-5-open-source-projects-and-wrote-this-book
  * https://authors.packtpub.com/
    * https://www.rosipov.com/blog/how-much-does-writing-a-book-earn/
    * https://www.quora.com/Did-you-make-money-writing-a-book-for-Packt-publishing-Would-you-write-for-them-again
  * https://nostarch.com/writeforus
  * https://www.manning.com/write-for-us
    * https://www.tunetheweb.com/blog/writing-a-technical-book-for-manning/
    * https://medium.com/modern-fortran/writing-a-technical-book-with-manning-in-2020-6ac3497500c9

* which platform
* podia
* paddle.com



While singer can make money with showcases and merchandising, do you want to sell tee shirts, or focus all your efforts on producing an awesome book?
 -->


# Publishing

There are quite a few publishers specializing into technical topics, the most famous being:

* https://www.oreilly.com
* https://packtpub.com
* https://nostarch.com
* https://www.manning.com



## Why you may want to work with a publisher

Publishers already have huge audiences (their customers), and a working supply chain.

They will save you a lot of marketing efforts.

<!-- By working with a publisher, -->

You can see the terms of each publisher here:
*  https://www.oreilly.com/work-with-us.html
*  https://authors.packtpub.com/
*  https://nostarch.com/writeforus
*  https://www.manning.com/write-for-us



## Why you actually want to self-publish your book


<!-- Why self-publishing:
- revenues
- you can write what you really want


drawbacks:
- no editor, so it takes a lot of time to verify facts

In my opinion, the goal of a technical writer is to maximize both the value provided to their readers, and personal profit.

If you don't like to provide value, then maybe you should sell NFTs or enter dropshipping.

And, if you don't think that your time and knowledge are valuable, then you can write free blog posts



Why not publishers

not your customers
-->

### Opacity

~6 months after I publicly announced the completion of my book, a famous tech publishing company contacted me to see if I was interested in working with them.

We exchanged a few emails, but the experience left me a very bad taste in my mouth: every time I asked specific questions, they were evasive and couldn't answer them.

The only thing that was clear from the exchange was that very few of their authors made enough money to live from their writings and thus had

So, you want me to work with you, but at the end I will be the only one who is not able to put food on the table thanks to the book, all while providing the expertise?

Hahah, no, thank you!

<!-- ### You should respect yourself -->



### It takes way more time

By working with an editor, you'll have to work with their editors and managers, which requires a lot of effort and time.


### They steal your customers

When sharing your work, you are building trust with your readers.

When you self-publish a book, you are building an audience but, more importantly, a customer base.

Everybody in sales knows that it's easier to take care of your customer than to acquire new ones. Thus, if people have already bought your product, there are more chances that they will buy another one again.

But, when you sell your work through a publisher, customers are not yours. The publisher has the email addresses of your readers, not you. So, if you decide to write another book, you will need to start your marketing efforts from zero, again.


### Drawbacks

On the other hand, self-publishing your book has a few drawbacks:

You need to do no everything by yourself (you can still hire contractors if you don't want to).

Marketing and building an audience takes a lot of time when starting from zero.

It's harder to review your own work, so there will certainly be a few mistakes in the first versions of your book.

## Which price?

If you are a nobody (like I was when I published my first book), then the price should be between $30 and $50.


<!-- A strategy that I tested many times with success is to offer discounts: -->

The unintuitive pricing strategy for Early-Access:

You certainly think that your early supporters should get a better price. But this is false.

Your early access customers are the one the most interested buy your book. The ones that are ready to pay the most.

Once you have saturated your niche, it remains only the curious. The one for who you are not solving a painful problem, but reading your book can be nice, not more.

That's why it makes sense to actually sell your book at a higher price during early access and offer some bonuses in return. When I say a higher price, it's like $10 more.

For example, my book was sold $48 during the early-access period with the bonuses and is now $38 without the bonuses.


### The coupon strategy

Another strategy that works well is to put your book at a very high price, $60-$80, for example, and to regularly announce limited-time offers.

Here, you will make most of your sales during the discount period, playing with the Fear Of Missing Out of your customers.

But, this strategy only works if you have an audience that you can regularly contact to push your discounts.


## From Markdown to ebooks


We are going to see 2 ways to turn your manuscript into ebooks for your readers.

The first one is [honkit](https://github.com/honkit/honkit), which is the "quick and dirty" way to get the job done, and the second one is [pandoc](https://pandoc.org) which requires a little bit more configuration, but is way more powerful and will allow you to produce good-looking ebooks.

But first, you need to create a cover for your book.

### Creating a cover

The easiest way to create a cover is to use [Figma](https://www.figma.com) using a 700 (width) x 930 (height) frame.

You can then export it to PNG and PDF: `cover.png` and `cover.pdf`, two files that we will use later to generate the ebooks.

### Using HonKit

`honkit` is a fork of [GitBook](https://github.com/GitbookIO/gitbook) when it was Open Source.

```bash
$ npm i -g honkit
$ mkdir mybook
$ honkit init mybook
$ honkit build
```


You can read more about it on [their website](https://honkit.netlify.app).

I personally don't use it because I find the end-result ugly. But if you need a simple promotional ebook or a quick PDF, it can get the job done quickly.


### Using Pandoc

Instead, I use [Pandoc](https://pandoc.org). Pandoc wraps a complex toolchain to turns many text formats (Markdown, AsciiDoc...) to documents (PDF, EPUB, DOCX...). It uses [LaTeX](https://www.latex-project.org) under the hood, but this is of no interest to us.



#### The pandoc configuration I use

Pandoc is not easy to configure, this is why, to save you countless hours of your precious time, I give you the configuration files and templates that I use to generate my ebooks.


Given the following files tree:
```
.
├── Dockerfile
├── ebooks
├── Makefile
├── README.md
└── src
    ├── 01_first_chapter.md
    ├── 02_second_chapter.md
    ├── assets
    │   ├── cover.pdf
    │   └── cover.png
    └── config
        ├── epub.css
        ├── inline_code.tex
        ├── settings.txt
        └── tango_mod.theme
```

Created with the following commands:

```bash
$ mkdir my_book
$ cd my_book
$ mkdir -p ebooks src/config src/assets
$ cat << EOF > src/01_first_chapter.md

# Hello
This is the first chapter.
EOF
$ cat << EOF > src/02_second_chapter.md
# World

This is the second chapter.
EOF
```

And the following configuration files:

**Dockerfile**
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


**Makefile**
```makefile

BOOK_FILE = output_book_file
BOOK_TITLE = TITLE OF YOUR BOOK
CONFIG_DIR = src/config
CHAPTERS_FILES = $(CONFIG_DIR)/settings.txt src/*.md


COVER = src/assets/cover.png
DATE := $(shell date "+%B %e, %Y")
VERSION = v2021.41
DIST_DIR = ebooks
DOCKER_IMAGE = localhost/writing/ebook


.PHONY: all
all: pdf epub mobi epubcheck


.PHONY: pdf
pdf:
  pandoc $(CHAPTERS_FILES) \
  --resource-path=src \
  --output=$(DIST_DIR)/$(BOOK_FILE)_content.pdf \
  --pdf-engine=xelatex \
  --table-of-contents --toc-depth=2 \
  --number-sections \
  --top-level-division=chapter \
  --include-in-header config/inline_code.tex \
  -V fontsize=12pt \
  -V documentclass=report \
  -V linkcolor:blue \
  --highlight-style $(CONFIG_DIR)/tango_mod.theme \
  -M date="$(VERSION)"
  pdftk src/assets/cover.pdf $(DIST_DIR)/$(BOOK_FILE)_content.pdf cat output $(DIST_DIR)/$(BOOK_FILE).pdf
  rm $(DIST_DIR)/$(BOOK_FILE)_content.pdf


.PHONY: epub
epub:
  pandoc $(CHAPTERS_FILES) \
  --resource-path=src \
  --output=$(DIST_DIR)/$(BOOK_FILE).epub \
  --table-of-contents --toc-depth=2 \
  --top-level-division=chapter \
  --number-sections \
  --listings \
  --standalone \
  --epub-cover-image=$(COVER) \
  --metadata title="$(BOOK_TITLE)" \
  --highlight-style $(CONFIG_DIR)/tango_mod.theme \
  --css $(CONFIG_DIR)/epub.css \
  -M date="$(DATE)"


.PHONY: epubcheck
epubcheck:
  java -jar /usr/bin/epubcheck ebooks/$(BOOK_FILE).epub


.PHONY: mobi
mobi:
  ebook-convert $(DIST_DIR)/$(BOOK_FILE).epub $(DIST_DIR)/$(BOOK_FILE).mobi --cover $(COVER)


.PHONY: docker
docker:
  docker build -t $(DOCKER_IMAGE):latest .
```

**src/config/settings.txt**
```yaml
---
title: TITLE OF YOUR BOOK
subtitle: SUBTITLE OF YOUR BOOK
author: YOUR NAME
subject: "markdown"
keywords: [books,programming,technology]
language: en-US
cover-image: src/assets/cover.png
lof: false
lof-own-page: true
toc-own-page: true
titlepage: false
colorlinks: true
geometry: "left=3cm, top=2cm, right=3cm, bottom=2cm"
linestretch: 1.25
---
```

**src/config/tango_mod.theme**
```json
{
    "text-color": null,
    "background-color": "#f8f8f8",
    "line-number-color": "#aaaaaa",
    "line-number-background-color": null,
    "text-styles": {
        "Other": {
            "text-color": "#8f5902",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Attribute": {
            "text-color": "#c4a000",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "SpecialString": {
            "text-color": "#4e9a06",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Annotation": {
            "text-color": "#8f5902",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Function": {
            "text-color": "#000000",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "String": {
            "text-color": "#4e9a06",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "ControlFlow": {
            "text-color": "#204a87",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Operator": {
            "text-color": "#ce5c00",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Error": {
            "text-color": "#a40000",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "BaseN": {
            "text-color": "#0000cf",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Alert": {
            "text-color": "#ef2929",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Variable": {
            "text-color": "#000000",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Extension": {
            "text-color": null,
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Preprocessor": {
            "text-color": "#8f5902",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Information": {
            "text-color": "#8f5902",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "VerbatimString": {
            "text-color": "#4e9a06",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Warning": {
            "text-color": "#8f5902",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Documentation": {
            "text-color": "#8f5902",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Import": {
            "text-color": null,
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Char": {
            "text-color": "#4e9a06",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "DataType": {
            "text-color": "#204a87",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Float": {
            "text-color": "#0000cf",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Comment": {
            "text-color": "#8f5902",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "CommentVar": {
            "text-color": "#8f5902",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Constant": {
            "text-color": "#000000",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "SpecialChar": {
            "text-color": "#000000",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "DecVal": {
            "text-color": "#0000cf",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        },
        "Keyword": {
            "text-color": "#204a87",
            "background-color": null,
            "bold": false,
            "italic": false,
            "underline": false
        }
    }
}
```

**src/config/epub.css**
```css
/* This defines styles and classes used in the book */
body { margin: 5%; text-align: justify; font-size: medium; }
code { font-family: monospace; }
h1 { text-align: left; }
h2 { text-align: left; }
h3 { text-align: left; }
h4 { text-align: left; }
h5 { text-align: left; }
h6 { text-align: left; }
/* For title, author, and date on the cover page */
h1.title { }
p.author { }
p.date { }
nav#toc ol,
nav#landmarks ol { padding: 0; margin-left: 1em; }
nav#toc ol li,
nav#landmarks ol li { list-style-type: none; margin: 0; padding: 0; }
a.footnote-ref { vertical-align: super; }
em, em em em, em em em em em { font-style: italic;}
em em, em em em em { font-style: normal; }
code{ white-space: pre-wrap; font-family: 'Courier New', Courier, monospace; font-size: 13px;}
span.smallcaps{ font-variant: small-caps; }
span.underline{ text-decoration: underline; }
q { quotes: """ """ "‘" "’"; }
div.column{ display: inline-block; vertical-align: top; width: 50%; }
div.hanging-indent{margin-left: 1.5em; text-indent: -1.5em;}
@media screen { /* Workaround for iBooks issue; see #6242 */
  .sourceCode {
    overflow: visible !important;
    white-space: pre-wrap !important;
  }
}
```

**src/config/inline_code.tex**
```tex

\usepackage{fancyvrb,newverbs,xcolor}

\definecolor{Light}{HTML}{F4F4F4}

\let\oldtexttt\texttt
\renewcommand{\texttt}[1]{
  \colorbox{Light}{\oldtexttt{#1}}
  \small
}

% \renewenvironment{Shaded} {\begin{snugshade}\footnotesize} {\end{snugshade}}

\usepackage{fvextra}
\DefineVerbatimEnvironment{Highlighting}{Verbatim}{breaklines,breakanywhere,commandchars=\\\{\}}
```


Edit `Makefile` and `src/config/settings.txt` with the name of your book and the correct metadata

```bash
$ make docker
```

Then put the cover files of your book (`cover.png` and `cover.pdf`) in the `src/assets` folder.


then:

```bash
$ docker run -ti --rm -v `pwd`:/ebook localhost/writing/ebook
```

And TDAAAAAA! You can find your ebook files in the `ebooks` folder.


## Selling your ebooks

Now you have the actual digital product, it's time to sell it.


### Gumroad

If you have a business that don't need to handle VAT, [Gumroad](https://gumroad.com) is certainly the easiest platform to get started to sell digital products.



### Podia

[Podia](https://podia.com) links with your [Stripe](https://stripe.com) account to handle payments. They are a good and cheaper alternative to gumroad as you pay a fixed monthly subscription. One nice thing for European businesses is that they can handle VAT.

I really like the experience as a customer as it gives access to a portal where I can re-download my past purchases.

One downside of Podia is that you can't configure the VAT rate, and thus are limited to the digital service rate, while most countries have a specific VAT rate for (e)books.

### Paddle

If you are an European business subject to collectin VAT and don't want to have to worry about this sh*t, [Paddle](https://www.paddle.com) may be your best option

One downside of Paddle is that they only send a one-time link by email to download the digital product, so if you make updates to your book, you will ned to find a way to distribute your updates to your readers.

### Stripe links

[Stripe Links](https://stripe.com/payments/payment-links) are not a good fit to sell your book because you can't easily include a link that will limit the download to customers.

### Amazon KDP (Kindle Direct Publishing)

if you want to make money from your book, [Amazon KDP](https://kdp.amazon.com) is not a good solution. They'll take a 70% cut if your book costs more than $9.99 or less than $2.99, and this is before taxes.

Furthermore, like with traditional publishers, customers are not yours. They are Amazon's which means that

The last thing that your should consider is that you need to fill and sign a few taxes forms to sell your book here, which may be hard to understand if you don't have an American accountant.

For all these reasons I've personally decided that I will never publish on Amazon KDP.

### Summary

Use Gumroad or Podia if you don't need to handle European VAT. Use Paddle or Podia if you need to handle European VAT.


## Avoiding piracy

If your book is successful, it will be shared illegally, which means various financial damages for you. You just can't avoid that.

Instead, what you can do is be prepared for that and pro-actively look for pirated versions of your book online.

There are mainly 4 sources of pirated ebooks:
- ZLibrary
- https://1filedownload.com
- Torrents
- Forums and private chats

While it's really hard to monitor private forums and chats, it's, on the other hand, easy to check once a month The Pirate Bay and ZLibrary to see if someone posted a pirate version of your book.



### The DMCA notice

One day, I found a pirated version of my book on the famous ebook pirate website: ZLibrary. I was extremely disappointed by my readers, but that's life, and at least, it had a good review on the website 🤷‍♂️.

I sent the following email to their support, and within a few days or weeks, I don't remember exactly. The pirated book was no longer available on their website.

```
To whom it may concern,

My name is [YOUR NAME] and I am writing to notify you of the copyright infringement and
unlawful use of my copyrighted material that appear on the service for which you are the
designated agent.

The infringing material, which I contend belongs to me, includes the following:
My book: [THE NAME OF YOUR BOOK].

The infringing material appears at the following location:

- [URLS]
- [OF THE PIRATED]
- [BOOK]
- And all the other mirrors


The original material is available at: [LANDING PAGE OF YOUR BOOK]

This letter is the official notification under Section 512(c) of the Digital Millennium Copyright Act of 1998 (" DMCA") and I request the immediate removal of the aforementioned infringing
material from your servers.

I also request that you immediately notify the infringer of this notice and inform them to cease any further posting of the infringing material to your server in the future.

I am providing this notice in good faith and with the firm belief that the use of the described material in the manner complained is not authorized by myself or the law.

If you have any questions, please contact me directly: [YOUR EMAIL ADDRESS]

Sincerely,
[YOUR NAME]

```


## Refunds

Don't be cheap on refunds.

An unsatisfied customer who gets a refund won't make much noise.

On the other hand, an unsatisfied customer who feels that you scammed them may make you lose more money than if you just refunded them by spreading their hate and misinformation on social media.


## Summary

* Self-publishing is not only easier and faster, but you will also make more money.
* Use [HonKit](https://github.com/honkit/honkit) (fast and dirty) or [Pandoc](https://pandoc.org) (harder but cleaner) to turn your Markdown files into ebooks.
* If you spot a pirate version of your book, send a DMCA notice to the website administrators.


<!-- [Next Chapter: Marketing](/technical-writer/marketing) -->
