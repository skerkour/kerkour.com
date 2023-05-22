+++
date = 2021-09-14T13:00:00Z
title = "How to implement HTTP Long Polling in Rust"
type = "post"
tags = ["rust", "programming", "tutorial", "webdev"]
authors = ["Sylvain Kerkour"]
url = "/rust-http-long-polling"

[extra]
lang = "en"

comment ="""
Long polling is extremely efficient in Rust

Real time is bout reacting to change as fast as possible

So the challenge is to know when new data is available

there is basically two kind of workload:
"""
+++


<!-- Real-time communication is all about the propagation of new data as fast as possible. When talking about real-time data, there is 2 kinds of workloads:
- Low-latency, multidirectional streams: Websockets.
- Medium-latency, unidirectional streams: Short Polling, Server-Sent Events (SSE), and Long Polling.

Today, we are going to look at the latter as it's the workload you will encounter the most often when developing web applications.

## Short Polling

![Short polling](https://kerkour.com/2021/rust-http-long-polling/short_polling.png)

The first method for real-time communications is short polling.

In this scenario, the client sends a request to the server, and the server immediately replies. If there is no new data, the response is empty. And most of the time, it's the case. So most of the time, the server's responses are empty and could have been avoided.

Thus, short polling is wasteful both in terms of network transfers and CPU, as requests need to be parsed and encoded each time.

The only pro is that it's impossible to do simpler.


## Server-Sent Events (SSE)

![SSE](https://kerkour.com/2021/rust-http-long-polling/sse.png)


Contrary to WebSockets, SSE streams are unidirectional: only the server can send data back to the client. Also, the mechanism for auto-reconnection is (normally) built-in into clients.

The downside is that it's not easy to implement server-side.


## Long Polling

![Long polling](https://kerkour.com/2021/rust-http-long-polling/long_polling.png)

Finally, there is long polling: the client emits a request, with an indication of the last piece of data it has, and the server sends the response back only when new data is available or when a certain amount of time is reached.

It has the advantage of being extremely simple to implement, as it's not a stream but a simple request-response scheme, and thus is extremely robust, does not require a complex auto-reconnection algorithm, and can handle network errors gracefully. Also, contrary to short polling, long polling is less wasteful regarding resources usage.

The only downside, is that long polling's latency is not as good as with WebSockets, but it does not matter most of the time.

Long polling is extremely efficient in Rust: thanks to `async`, very few resources (a simple `async` Task) are used per open connection, while a lot of languages use a whole OS thread.

Finally, as long polling is plain and simple HTTP requests, it's the technique that has more chances of not being blocked by some kind of aggressive firewall or network equipment.


## Long Polling in Rust -->

We will use the new web framework developed by [tokio's team](https://github.com/tokio-rs): [axum](https://github.com/tokio-rs/axum). Its performance and simplicity are unparalleled in the Rust world. Also, please note that porting this code to another web framework is easy.

We will implement a simple chat server, as chat is the textbook application that benefits the most from long polling.

**There are 3 tricks to make this implementation efficient, so stay attentive ;)**


### The Chat Service

The Chat Service is an object that encapsulates all our business logic. To keep the example simple, we will only make database calls.

**Here is our first trick**: In order to enable message ordering, we don't use a `UUIDv4`. Instead, **we use a [ULID](https://github.com/ulid/spec)** that we convert to a `UUID` so there is no problem to serialize / deserialize it: `Uuid = Ulid::new().into()`

**chat.rs**
```rust
impl ChatService {
    pub fn new(db: DB) -> Self {
        ChatService { db }
    }

    pub async fn create_message(&self, body: String) -> Result<Message, Error> {
        if body.len() > 10_000 {
            return Err(Error::InvalidArgument("Message is too large".to_string()));
        }

        let created_at = chrono::Utc::now();
        let id: Uuid = Ulid::new().into();

        let query = "INSERT INTO messages
            (id, created_at, body)
            VALUES ($1, $2, $3)";

        sqlx::query(query)
            .bind(id)
            .bind(created_at)
            .bind(&body)
            .execute(&self.db)
            .await?;

        Ok(Message {
            id,
            created_at,
            body,
        })
    }
```


**Here is our second trick**: notice the `after.unwrap_or(Uuid::nil())` which return a "zero" UUID (`00000000-0000-0000-0000-000000000000`). With `WHERE id > $1` it allows us to return all the messages if `after` is `None`.

It's useful to rehydrate the whole state of a client, for example.

```rust
    pub async fn find_messages(&self, after: Option<Uuid>) -> Result<Vec<Message>, Error> {
        let query = "SELECT *
            FROM messages
            WHERE id > $1";

        let messages: Vec<Message> = sqlx::query_as::<_, Message>(query)
            .bind(after.unwrap_or(Uuid::nil()))
            .fetch_all(&self.db)
            .await?;

        Ok(messages)
    }
}
```


### The Web Server

Next, the boilerplate to run the web server.

Thanks to `.layer(AddExtensionLayer::new(ctx))`, `ServerContext` is injected into all the routes so we can call `ChatService`'s methods.

```rust
struct ServerContext {
    chat_service: chat::ChatService,
}

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
```


### Long Polling

Finally, our **third trick**: long polling is a simple loop with `tokio::time::sleep`.

By using `tokio::time::sleep`, an active connection will barely use any resources when waiting.

If new data is found, we immediately return with the new data. Else, we wait one more second.

After 10 seconds, we return empty data.

**main.rs**
```rust
async fn handler_find_messages(
    Extension(ctx): Extension<Arc<ServerContext>>,
    query_params: Query<FindMessagesQueryParameters>,
) -> Result<Json<Vec<Message>>, Error> {
    let sleep_for = Duration::from_secs(1);

    // long polling: 10 secs
    for _ in 0..10u64 {
        let messages = ctx.chat_service.find_messages(query_params.after).await?;
        if messages.len() != 0 {
            return Ok(messages.into());
        }

        tokio::time::sleep(sleep_for).await;
    }

    // return an empty response
    Ok(Vec::new().into())
}
```


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_long_polling) (please don't forget to star the repo 🙏).



**Want to learn how to craft more advanced web applications in Rust (such as a WebAssembly frontend and a JSON API backend)?** Get my book: **[Black Hat Rust](https://kerkour.com/black-hat-rust)**. All early-access supporters get a special discount and awesome bonuses: [https://kerkour.com/black-hat-rust](https://kerkour.com/black-hat-rust).

**Warning**: this offer is limited in time!
