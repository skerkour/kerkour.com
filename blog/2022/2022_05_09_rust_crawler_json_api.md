+++
date = 2022-05-09T02:00:00Z
title = "Building a crawler in Rust: Crawling a JSON API"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-crawler-json-api"

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
* **[Building a crawler in Rust: Crawling a JSON API](https://kerkour.com/rust-crawler-json-api)**
* [Building a crawler in Rust: Crawling Javascript Single Page Applications (SPA) with a headless browser](https://kerkour.com/rust-crawler-javascript-single-page-application-headless-browser)


Crawling a JSON API is, on the other hand, pretty straightforward, as the data is already (in theory) structured. The only difficulty is to find the next pages to crawl.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


Here, we are going to scrape all the users of a GitHub organization. Why it's useful? Because if you gain access to one of these accounts (by finding a leaked token or some other means), or gain access to some of the repositories of the organization.

**[ch_05/crawler/src/spiders/github.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/src/spiders/github.rs)**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubItem {
    login: String,
    id: u64,
    node_id: String,
    html_url: String,
    avatar_url: String,
}
```


As our crawler won't make tons of requests, we don't need to use a token to authenticate to Github's API, but we need to set up some headers. Otherwise, the server would block our requests.

Finally, we also need a regexp, as a *quick and dirty* way to find next page to crawl:
```rust
pub struct GitHubSpider {
    http_client: Client,
    page_regex: Regex,
    expected_number_of_results: usize,
}

impl GitHubSpider {
    pub fn new() -> Self {
        let http_timeout = Duration::from_secs(6);
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Accept",
            header::HeaderValue::from_static("application/vnd.github.v3+json"),
        );

        let http_client = Client::builder()
            .timeout(http_timeout)
            .default_headers(headers)
            .user_agent(
                "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0",
            )
            .build()
            .expect("spiders/github: Building HTTP client");

        // will match https://...?page=XXX
        let page_regex =
            Regex::new(".*page=([0-9]*).*").expect("spiders/github: Compiling page regex");

        GitHubSpider {
            http_client,
            page_regex,
            expected_number_of_results: 100,
        }
    }
}
```

Extracting the item is just a matter of parsing the JSON, which is easy thanks to `reqwest`, which provides the `json` method.

Here, the trick is to find the next URL to visit. For that, we use the regex compiled above and capture the current page number. For example, in `...&page=2` we capture `2`.

Then we parse this String into a number, increment this number, and replace the original URL with the new number. Thus the new URL would be `...&page=3`.

If the API doesn't return the expected number of results (which is configured with the `per_page` query parameter), then it means that we are at the last page of the results, so there is no more page to crawl.

**[ch_05/crawler/src/spiders/github.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/src/spiders/github.rs)**
```rust
async fn scrape(&self, url: String) -> Result<(Vec<GitHubItem>, Vec<String>), Error> {
    let items: Vec<GitHubItem> = self.http_client.get(&url).send().await?.json().await?;

    let next_pages_links = if items.len() == self.expected_number_of_results {
        let captures = self.page_regex.captures(&url).unwrap();
        let old_page_number = captures.get(1).unwrap().as_str().to_string();
        let mut new_page_number = old_page_number
            .parse::<usize>()
            .map_err(|_| Error::Internal("spider/github: parsing page number".to_string()))?;
        new_page_number += 1;

        let next_url = url.replace(
            format!("&page={}", old_page_number).as_str(),
            format!("&page={}", new_page_number).as_str(),
        );
        vec![next_url]
    } else {
        Vec::new()
    };

    Ok((items, next_pages_links))
}
```

To run this spider, go to the git repository accompanying this book, in **[ch_05/crawler/](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/)**, and run:
```bash
$ cargo run -- run --spider github
```
