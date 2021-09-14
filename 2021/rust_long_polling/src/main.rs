use axum::{
    extract::{Extension, Query},
    handler::{get, Handler},
    response::IntoResponse,
    AddExtensionLayer, Json, Router,
};
use serde::Deserialize;
use std::{sync::Arc, time::Duration};

mod chat;
mod db;
mod error;

use chat::Message;
pub use db::DB;
pub use error::Error;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "rust_long_polling=info");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| Error::BadConfig("DATABASE_URL env var is missing".to_string()))?;

    let db = db::connect(&database_url).await?;
    db::migrate(&db).await?;

    let chat_service = chat::ChatService::new(db);
    let ctx = Arc::new(ServerContext::new(chat_service));

    // build our application with a single route
    let app = Router::new()
        .route(
            "/messages",
            get(handler_find_messages).post(handler_create_message),
        )
        .or(handler_404.into_service())
        .layer(AddExtensionLayer::new(ctx));

    log::info!("Starting server on 0.0.0.0:8080");
    axum::Server::bind(
        &"0.0.0.0:8080"
            .parse()
            .expect("parsing server's bind address"),
    )
    .serve(app.into_make_service())
    .await
    .expect("running server");

    Ok(())
}

struct ServerContext {
    chat_service: chat::ChatService,
}

impl ServerContext {
    pub fn new(chat_service: chat::ChatService) -> Self {
        ServerContext { chat_service }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct CreateMessage {
    body: String,
}

#[derive(Deserialize)]
struct FindMessagesQueryParameters {
    after: Option<Uuid>,
}

async fn handler_404() -> impl IntoResponse {
    Error::NotFound("Route not found".to_string())
}

async fn handler_create_message(
    Extension(ctx): Extension<Arc<ServerContext>>,
    Json(input): Json<CreateMessage>,
) -> Result<Json<Message>, Error> {
    let message = ctx.chat_service.create_message(input.body).await?;

    Ok(message.into())
}

async fn handler_find_messages(
    Extension(ctx): Extension<Arc<ServerContext>>,
    query_params: Query<FindMessagesQueryParameters>,
) -> Result<Json<Vec<Message>>, Error> {
    let sleep_for = Duration::from_secs(1);

    // long polling: 5 secs
    for _ in 0..5u64 {
        let messages = ctx.chat_service.find_messages(query_params.after).await?;
        if messages.len() != 0 {
            return Ok(messages.into());
        }

        tokio::time::sleep(sleep_for).await;
    }

    Ok(Vec::new().into())
}
