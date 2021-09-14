use axum::{handler::get, Router};

mod chat;
mod db;
mod error;

pub use db::DB;
pub use error::Error;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "rust_long_polling=info");
    env_logger::init();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    log::info!("Starting server on 0.0.0.0:8080");
    axum::Server::bind(
        &"0.0.0.0:8080"
            .parse()
            .expect("parsing server's bind address"),
    )
    .serve(app.into_make_service())
    .await
    .expect("running server");
}
