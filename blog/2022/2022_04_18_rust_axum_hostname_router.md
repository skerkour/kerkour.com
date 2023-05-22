+++
date = 2022-04-18T02:00:00Z
title = "Hostname based router with axum in Rust"
type = "post"
tags = ["programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-axum-hostname-router"

[extra]
lang = "en"

comment ="""
"""
+++

[axum](https://docs.rs/axum)'s router does not natively support hostname-based routing, which may be useful if you need to serve multiple applications with a single server.

Fortunately, it's easy to compose multiple Routers to route your requests depending on the `Host` header.

Here is how.

## Host based routing

First, we define our two sub-routers:

```rust
use axum::{
    body::Body, extract::Host, http::Request, response::Html, routing::any, Extension, Router,
};
use std::sync::Arc;
use std::{
    net::SocketAddr,
    sync::atomic::{AtomicUsize, Ordering},
};
use tower::ServiceExt;

struct State {
    api_requests: AtomicUsize,
    website_requests: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let api_router = Router::new().route("/*path", any(api_handler));
    let website_router = Router::new().route("/*path", any(website_handler));

    // ...
}

async fn api_handler(Extension(state): Extension<Arc<State>>) -> Html<String> {
    state.api_requests.fetch_add(1, Ordering::SeqCst);
    Html(format!(
        "api: {}",
        state.api_requests.load(Ordering::SeqCst)
    ))
}

async fn website_handler(Extension(state): Extension<Arc<State>>) -> Html<String> {
    state.website_requests.fetch_add(1, Ordering::SeqCst);
    Html(format!(
        "website: {}",
        state.website_requests.load(Ordering::SeqCst)
    ))
}
```

Then we define the main router, which handles all the requests and dispatches them to the good sub-router depending on the hostname.

For that, we use the [Host extractor](https://docs.rs/axum/latest/axum/extract/struct.Host.html) and pattern matching:

```rust
#[tokio::main]
async fn main() {
    // ...
    let app = Router::new()
        .route(
            "/*path",
            any(|Host(hostname): Host, request: Request<Body>| async move {
                match hostname.as_str() {
                    "api.mydomain.com" => api_router.oneshot(request).await,
                    _ => website_router.oneshot(request).await,
                }
            }),
        )
        .layer(Extension(state));
    // ...
}
```

The great thing is that we can attach layers (middlewares and extensions) to the main router and they will be forwarded to the sub-routers.


Finally, we can run the server:
```rust
#[tokio::main]
async fn main() {

    // ...
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```



## The code is on GitHub


As usual, you can find the complete code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2022/rust_axum_hostname_router) (please don't forget to star the repo 🙏).

**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build multiple web applications in Rust to phish credentials and control a RAT (Remote Access Tool).**
