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

    let state = Arc::new(State {
        website_requests: AtomicUsize::new(0),
        api_requests: AtomicUsize::new(0),
    });

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

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
