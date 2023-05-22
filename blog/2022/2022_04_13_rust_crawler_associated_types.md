+++
date = 2022-04-13T02:00:00Z
title = "Building a crawler in Rust: Design and Associated Types"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-crawler-associated-types"

[extra]
lang = "en"

comment ="""
"""
+++

**Building a crawler in Rust**:
* **[Building a crawler in Rust: Design and Associated Types](https://kerkour.com/rust-crawler-associated-types)**
* [Building a crawler in Rust: Synchronization (Atomic Types and Barriers)](https://kerkour.com/rust-crawler-synchronization-atomic-types-barrier)
* [Building a crawler in Rust: Implementing the crawler](https://kerkour.com/rust-crawler-implementation)
* [Building a crawler in Rust: Scraping and Parsing HTML](https://kerkour.com/rust-crawler-scraping-and-parsing-html)
* [Building a crawler in Rust: Crawling a JSON API](https://kerkour.com/rust-crawler-json-api)
* [Building a crawler in Rust: Crawling Javascript Single Page Applications (SPA) with a headless browser](https://kerkour.com/rust-crawler-javascript-single-page-application-headless-browser)


First, a term disambiguation: what is the difference between a scraper and a crawler?

Scraping is the process of turning unstructured web data into structured data.

![Web scraping](/2022/rust-crawler-associated-types/ch05_web_scraper.png)


Crawling is the process of running through a lot of interlinked data (web pages, for example).

In practice, it's most of the time useless to scrape without crawling through multiple pages or to crawl without scraping content, so we can say that each crawler is a scraper, and almost every scraper is a crawler.

Some people prefer to call a scraper a crawler for a specific website and a crawler something that crawls the entire web. Anyway, I think that it's nitpicking, so we won't spend more time debating.

For the rest of this book, we are going to use the term **crawler**.

So, why crawl websites to scrape data?

It's all about **automation**. Yes, you can manually browse the 1000s pages of a website and manually copy/paste the data in a spreadsheet.

Or, you could build a specialized program, the crawler, that will do it for you in a blink.


> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)



### Designing a crawler

![The architecture of a crawler](/2022/rust-crawler-associated-types/ch05_crawler_architecture.png)


A crawler is composed of the following parts:

**Start URLs**: you need a list of seed URLs to start the crawl. For example, the root page of your target's website.


**Spiders**: this is the specialized part of a crawler, tuned for a specific site or task. For example, we could implement a spider to get all the users of a GitHub organization or all the vulnerabilities of a specific product. A spider is itself composed of 2 parts:

- The scraper that fetches the URLs, parses the data, turns it into structured data, and a list of URLs extracted from the document to continue the crawl.
- The processor that precesses the structured data: saving it to a database, for example.


The biggest advantage of splitting the responsibilities of a spider into 2 distinct stages is that they can be run with different concurrency levels depending on your expected workload. For example, you could have a pool with 3 concurrent scrapers not to flood the website you are crawling and trigger bot detection systems, but 100 concurrent processors.

<!-- The control loop -->

<!--

the goal is to have to create the minimum of code for a specific website
or put in another way:
to reuse the maximum of code across crawls

 -->


**A Control loop**: this is the generic part of a crawler. Its job is to dispatch data between the scrapers and the processors and queue URLs.


## Why Rust for crawling

Now you may be wondering, why Rust for crawling? After all, Python and Go already have a solid ecosystem around this problem (respectively [Scrapy](https://scrapy.org/) and [Colly](http://go-colly.org/)).


### Async

The first, and maybe most important reason for using Rust, is its async I/O model: you are guaranteed to have the best performance possible when making network requests.

### Memory-related performance

Making a lot of network requests and parsing data often require creating a lot of short-lived memory objects, which would put a lot of pressure on garbage collectors. As Rust doesn't have a garbage collector, it doesn't have this problem, and the memory usage will be far more deterministic.


### Safety when parsing

Scraping requires parsing. Parsing is one of the most common ways to introduce vulnerabilities ([Parsing JSON is a Minefield](https://seriot.ch/projects/parsing_json.html), [XML parsing vulnerabilities](https://gist.github.com/mgeeky/4f726d3b374f0a34267d4f19c9004870)) or bugs. Rust, on the other hand, with its memory safety and strict error handling, provides better tools to handle the complex task of parsing untrusted data and complex formats.




## Associated types

Now we are all up about what a crawler is and why Rust, let's learn the last few Rust features that we need to build a crawler.

The last important point to know about generics in Rust is: **Associated types**.

You already dealt with associated types when using iterators and Futures.

Remember `Future<Output=String>`, here `String` is an associated type.

We could build a generic spider such as:
```rust
pub trait Spider<I>{
    fn name(&self) -> String;
    fn start_urls(&self) -> Vec<String>;
    async fn scrape(&self, url: &str) -> Result<(Vec<I>, Vec<String>), Error>;
    async fn process(&self, item: I) -> Result<(), Error>;
}
```

But then it would be very inconvenient to use it as each function using it would need to also be generic over `I`:
```rust
fn use_spider<I, S: Spider<I>>(spider: S) {
    // ...
}
```

By using an associated type, we simplify the usage of the trait and communicate more clearly how it works:
```rust
#[async_trait]
pub trait Spider {
    type Item;

    fn name(&self) -> String;
    fn start_urls(&self) -> Vec<String>;
    async fn scrape(&self, url: &str) -> Result<(Vec<Self::Item>, Vec<String>), Error>;
    async fn process(&self, item: Self::Item) -> Result<(), Error>;
}
```

```rust
fn use_spider<S: Spider>(spider: S) {
    // ...
}
```

Like with type parameters, you can add constraints to associated types:

```rust
pub trait Spider {
    type Item: Debug + Clone;

    fn name(&self) -> String;
    fn start_urls(&self) -> Vec<String>;
    async fn scrape(&self, url: &str) -> Result<(Vec<Self::Item>, Vec<String>), Error>;
    async fn process(&self, item: Self::Item) -> Result<(), Error>;
}
```


**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build a crawler in Rust to list vulnerabilities and gather data about our targets.**
