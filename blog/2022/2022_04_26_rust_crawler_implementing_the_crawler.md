+++
date = 2022-04-26T02:00:00Z
title = "Building a crawler in Rust: Implementing the crawler"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-crawler-implementation"

[extra]
lang = "en"

comment ="""
"""
+++


**Building a crawler in Rust**:
* [Building a crawler in Rust: Design and Associated Types](https://kerkour.com/rust-crawler-associated-types)
* [Building a crawler in Rust: Synchronization (Atomic Types and Barriers)](https://kerkour.com/rust-crawler-synchronization-atomic-types-barrier)
* **[Building a crawler in Rust: Implementing the crawler](https://kerkour.com/rust-crawler-implementation)**
* [Building a crawler in Rust: Scraping and Parsing HTML](https://kerkour.com/rust-crawler-scraping-and-parsing-html)
* [Building a crawler in Rust: Crawling a JSON API](https://kerkour.com/rust-crawler-json-api)
* [Building a crawler in Rust: Crawling Javascript Single Page Applications (SPA) with a headless browser](https://kerkour.com/rust-crawler-javascript-single-page-application-headless-browser)

Now that we have a [clear idea of the design of the crawler](https://kerkour.com/rust-crawler-associated-types) and of the [Rust's features we are going to use](https://kerkour.com/rust-crawler-synchronization-atomic-types-barrier), let's start the actual implementation.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


In the following section, we are going to build a generic crawler and three different spiders:

- a spider for an HTML-only website
- a spider for a JSON API
- and a spider for a website using JavaScript to render elements so we are going to need to use a headless browser



## The spider trait

**[ch_05/crawler/src/spiders/mod.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/src/spiders/mod.rs)**
```rust
#[async_trait]
pub trait Spider: Send + Sync {
    type Item;

    fn name(&self) -> String;
    fn start_urls(&self) -> Vec<String>;
    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error>;
    async fn process(&self, item: Self::Item) -> Result<(), Error>;
}
```


## Implementing the crawler


**[ch_05/crawler/src/crawler.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_05/crawler/src/crawler.rs)**
```rust
    pub async fn run<T: Send + 'static>(&self, spider: Arc<dyn Spider<Item = T>>) {
        let mut visited_urls = HashSet::<String>::new();
        let crawling_concurrency = self.crawling_concurrency;
        let crawling_queue_capacity = crawling_concurrency * 400;
        let processing_concurrency = self.processing_concurrency;
        let processing_queue_capacity = processing_concurrency * 10;
        let active_spiders = Arc::new(AtomicUsize::new(0));

        let (urls_to_visit_tx, urls_to_visit_rx) = mpsc::channel(crawling_queue_capacity);
        let (items_tx, items_rx) = mpsc::channel(processing_queue_capacity);
        let (new_urls_tx, mut new_urls_rx) = mpsc::channel(crawling_queue_capacity);
        let barrier = Arc::new(Barrier::new(3));
```


```rust
        for url in spider.start_urls() {
            visited_urls.insert(url.clone());
            let _ = urls_to_visit_tx.send(url).await;
        }

        self.launch_processors(
            processing_concurrency,
            spider.clone(),
            items_rx,
            barrier.clone(),
        );

        self.launch_scrapers(
            crawling_concurrency,
            spider.clone(),
            urls_to_visit_rx,
            new_urls_tx.clone(),
            items_tx,
            active_spiders.clone(),
            self.delay,
            barrier.clone(),
        );
```

And finally, the control loop, where we queue new URLs that have not already have been visited and check if we need to stop the crawler.

By dropping `urls_to_visit_tx`, we close the channels, and thus stop the scrappers, once they have all finished processing the remaining URLs in the channel.


```rust
        loop {
            if let Some((visited_url, new_urls)) = new_urls_rx.try_recv().ok() {
                visited_urls.insert(visited_url);

                for url in new_urls {
                    if !visited_urls.contains(&url) {
                        visited_urls.insert(url.clone());
                        log::debug!("queueing: {}", url);
                        let _ = urls_to_visit_tx.send(url).await;
                    }
                }
            }

            if new_urls_tx.capacity() == crawling_queue_capacity // new_urls channel is empty
            && urls_to_visit_tx.capacity() == crawling_queue_capacity // urls_to_visit channel is empty
            && active_spiders.load(Ordering::SeqCst) == 0
            {
                // no more work, we leave
                break;
            }

            sleep(Duration::from_millis(5)).await;
        }

        log::info!("crawler: control loop exited");

        // we drop the transmitter in order to close the stream
        drop(urls_to_visit_tx);

        // and then we wait for the streams to complete
        barrier.wait().await;
    }
```


Executing the processors concurrently is just a matter of spawning a new task, with a stream and `for_each_concurrent`. Once the stream is stopped, we "notify" the `barrier`.
```rust
    fn launch_processors<T: Send + 'static>(
        &self,
        concurrency: usize,
        spider: Arc<dyn Spider<Item = T>>,
        items: mpsc::Receiver<T>,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(items)
                .for_each_concurrent(concurrency, |item| async {
                    let _ = spider.process(item).await;
                })
                .await;

            barrier.wait().await;
        });
    }
```

Finally, launching scrapers, like processors, requires a new task, with a stream and `for_each_concurrent`.

The logic here is a little bit more complex:

- we first increment `active_spiders`
- then, we scrape the URL and extract the data and the next URLs to visit
- we then send these items to the processors
- we also send the newly found URLs to the control loop
- and we sleep for the configured delay, not to flood the server
- finally, we decrement `active_spiders`

By dropping `items_tx`, we are closing the `items` channel, and thus stopping the processors once the channel is empty.

```rust
    fn launch_scrapers<T: Send + 'static>(
        &self,
        concurrency: usize,
        spider: Arc<dyn Spider<Item = T>>,
        urls_to_vist: mpsc::Receiver<String>,
        new_urls: mpsc::Sender<(String, Vec<String>)>,
        items_tx: mpsc::Sender<T>,
        active_spiders: Arc<AtomicUsize>,
        delay: Duration,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(urls_to_vist)
                .for_each_concurrent(concurrency, |queued_url| {
                    let queued_url = queued_url.clone();
                    async {
                        active_spiders.fetch_add(1, Ordering::SeqCst);
                        let mut urls = Vec::new();
                        let res = spider
                            .scrape(queued_url.clone())
                            .await
                            .map_err(|err| {
                                log::error!("{}", err);
                                err
                            })
                            .ok();

                        if let Some((items, new_urls)) = res {
                            for item in items {
                                let _ = items_tx.send(item).await;
                            }
                            urls = new_urls;
                        }

                        let _ = new_urls.send((queued_url, urls)).await;
                        sleep(delay).await;
                        active_spiders.fetch_sub(1, Ordering::SeqCst);
                    }
                })
                .await;

            drop(items_tx);
            barrier.wait().await;
        });
    }
```


**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build a crawler in Rust to scrape vulnerabilities and gather data about our targets.**

