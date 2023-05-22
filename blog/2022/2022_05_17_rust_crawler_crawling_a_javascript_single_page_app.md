+++
date = 2022-05-17T02:00:00Z
title = "Building a crawler in Rust: Scraping Javascript Single Page Applications (SPA) with a headless browser"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-crawler-javascript-single-page-application-headless-browser"

[extra]
lang = "en"

comment ="""
"""
+++

**Building a crawler in Rust**:
* [Building a crawler in Rust: Design and Associated Types](https://kerkour.com/rust-crawler-associated-types)
* [Building a crawler in Rust: Synchronization (Atomic Types and Barriers)](https://kerkour.com/rust-crawler-synchronization-atomic-types-barrier)
* [Building a crawler in Rust: Implementing the crawler](https://kerkour.com/rust-crawler-implementation)
* [Building a crawler in Rust: Scraping and Parsing HTML](https://kerkour.com/rust-crawler-scraping-and-parsing-html)
* [Building a crawler in Rust: Crawling a JSON API](https://kerkour.com/rust-crawler-json-api)
* **[Building a crawler in Rust: Scraping Javascript Single Page Applications (SPA) with a headless browser](https://kerkour.com/rust-crawler-javascript-single-page-application-headless-browser)**



Nowadays, more and more websites generate elements of the pages client-side, using JavaScript. In order to get this data, we need a **headless browser**: it's a browser that can be operated remotely and programmatically.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


For that, we will use [chromedriver](https://chromedriver.chromium.org/downloads).

On a Debian-style machine, it can be installed with:
```bash
$ sudo apt install chromium-browser chromium-chromedriver
```


Because the headless browser client methods require a mutable reference (`&mut self`), we need to wrap it with a mutex to be able to use it safely in our pool of scrapers.

**[ch_05/crawler/src/spiders/quotes.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/src/spiders/quotes.rs)**
```rust
impl QuotesSpider {
    pub async fn new() -> Result<Self, Error> {
        let mut caps = serde_json::map::Map::new();
        let chrome_opts = serde_json::json!({ "args": ["--headless", "--disable-gpu"] });
        caps.insert("goog:chromeOptions".to_string(), chrome_opts);
        let webdriver_client = ClientBuilder::rustls()
            .capabilities(caps)
            .connect("http://localhost:4444")
            .await?;

        Ok(QuotesSpider {
            webdriver_client: Mutex::new(webdriver_client),
        })
    }
}
```

Fetching a web page with our headless browser can be achieved in two steps:

- first, we go to the URL
- then, we fetch the source

**[ch_05/crawler/src/spiders/quotes.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/src/spiders/quotes.rs)**
```rust
async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error> {
    let mut items = Vec::new();
    let html = {
        let mut webdriver = self.webdriver_client.lock().await;
        webdriver.goto(&url).await?;
        webdriver.source().await?
    };
```

Once we have the rendered source of the page, we can scrape it like any other HTML page:
```rust
let document = Document::from(html.as_str());

let quotes = document.select(Class("quote"));
for quote in quotes {
    let mut spans = quote.select(Name("span"));
    let quote_span = spans.next().unwrap();
    let quote_str = quote_span.text().trim().to_string();

    let author = spans
        .next()
        .unwrap()
        .select(Class("author"))
        .next()
        .unwrap()
        .text()
        .trim()
        .to_string();

    items.push(QuotesItem {
        quote: quote_str,
        author,
    });
}
```


```rust
let next_pages_link = document
    .select(
        Class("pager")
            .descendant(Class("next"))
            .descendant(Name("a")),
    )
    .filter_map(|n| n.attr("href"))
    .map(|url| self.normalize_url(url))
    .collect::<Vec<String>>();

Ok((items, next_pages_link))
```


To run this spider, you first need to launch `chromedriver` in a separate shell:
```bash
$ chromedriver --port=4444 --disable-dev-shm-usage
```

Then, in another shell, go to the git repository accompanying this book, in **[ch_05/crawler/](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/)**, and run:
```bash
$ cargo run -- run --spider quotes
```
