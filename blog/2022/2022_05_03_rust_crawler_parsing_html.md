+++
date = 2022-05-03T02:00:00Z
title = "Building a crawler in Rust: Scraping and Parsing HTML"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-crawler-scraping-and-parsing-html"

[extra]
lang = "en"

comment ="""
"""
+++


**Building a crawler in Rust**:
* [Building a crawler in Rust: Design and Associated Types](https://kerkour.com/rust-crawler-associated-types)
* [Building a crawler in Rust: Synchronization (Atomic Types and Barriers)](https://kerkour.com/rust-crawler-synchronization-atomic-types-barrier)
* [Building a crawler in Rust: Implementing the crawler](https://kerkour.com/rust-crawler-implementation)
* **[Building a crawler in Rust: Scraping and Parsing HTML](https://kerkour.com/rust-crawler-scraping-and-parsing-html)**
* [Building a crawler in Rust: Crawling a JSON API](https://kerkour.com/rust-crawler-json-api)
* [Building a crawler in Rust: Crawling Javascript Single Page Applications (SPA) with a headless browser](https://kerkour.com/rust-crawler-javascript-single-page-application-headless-browser)

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


Now that we have a fast concurrent crawler, it's time to actually parse the HTML and extract structured data (remember that this [process is called scraping](https://kerkour.com/rust-crawler-associated-types)).


The plain HTML website that we will crawl is [CVE Details](https://www.cvedetails.com/): *the ultimate security vulnerabilities datasource*.

It's a website providing an easy way to search for vulnerabilities with a [CVE ID](https://cve.mitre.org/).

We will use this page as the start URL: [https://www.cvedetails.com/vulnerability-list/vulnerabilities.html](https://www.cvedetails.com/vulnerability-list/vulnerabilities.html) which, when you look at the bottom of the page, provides the links to all the other pages listing the vulnerabilities.



### Extracting structured data

The first step is to identify what data we want. In this case, it's all the information of a CVE entry:
**[ch_05/crawler/src/spiders/cvedetails.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/src/spiders/cvedetails.rs)**
```rust
#[derive(Debug, Clone)]
pub struct Cve {
    name: String,
    url: String,
    cwe_id: Option<String>,
    cwe_url: Option<String>,
    vulnerability_type: String,
    publish_date: String,
    update_date: String,
    score: f32,
    access: String,
    complexity: String,
    authentication: String,
    confidentiality: String,
    integrity: String,
    availability: String,
}
```

Then, with a browser and the developers tools, we inspect the page to search the relevant HTML classes and ids that will allow us to extract that data:
**[ch_05/crawler/src/spiders/cvedetails.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/src/spiders/cvedetails.rs)**
```rust
async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error> {
    log::info!("visiting: {}", url);

    let http_res = self.http_client.get(url).send().await?.text().await?;
    let mut items = Vec::new();

    let document = Document::from(http_res.as_str());

    let rows = document.select(Attr("id", "vulnslisttable").descendant(Class("srrowns")));
    for row in rows {
        let mut columns = row.select(Name("td"));
        let _ = columns.next(); // # column
        let cve_link = columns.next().unwrap().select(Name("a")).next().unwrap();
        let cve_name = cve_link.text().trim().to_string();
        let cve_url = self.normalize_url(cve_link.attr("href").unwrap());


        let _ = columns.next(); // # of exploits column

        let access = columns.next().unwrap().text().trim().to_string();
        let complexity = columns.next().unwrap().text().trim().to_string();
        let authentication = columns.next().unwrap().text().trim().to_string();
        let confidentiality = columns.next().unwrap().text().trim().to_string();
        let integrity = columns.next().unwrap().text().trim().to_string();
        let availability = columns.next().unwrap().text().trim().to_string();
```

```rust
        let cve = Cve {
            name: cve_name,
            url: cve_url,
            cwe_id: cwe.as_ref().map(|cwe| cwe.0.clone()),
            cwe_url: cwe.as_ref().map(|cwe| cwe.1.clone()),
            vulnerability_type,
            publish_date,
            update_date,
            score,
            access,
            complexity,
            authentication,
            confidentiality,
            integrity,
            availability,
        };
        items.push(cve);
    }
}
```


### Extracting links

**[ch_05/crawler/src/spiders/cvedetails.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/src/spiders/cvedetails.rs)**
```rust
let next_pages_links = document
    .select(Attr("id", "pagingb").descendant(Name("a")))
    .filter_map(|n| n.attr("href"))
    .map(|url| self.normalize_url(url))
    .collect::<Vec<String>>();
```


To run this spider, go to the git repository accompanying this book, in **[ch_05/crawler](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/)**, and run:
```bash
$ cargo run -- run --spider cvedetails
```



**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build a crawler in Rust to scrape vulnerabilities and gather data about our targets.**
