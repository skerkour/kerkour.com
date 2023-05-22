+++
date = 2022-02-22T08:00:00Z
title = "Which Rust web framework to choose in 2022 (with code examples)"
type = "post"
tags = ["rust", "programming", "tutorial", "webdev"]
authors = ["Sylvain Kerkour"]
url = "/rust-web-framework-2022"

[extra]
lang = "en"

comment ="""

"""
+++

## Table of contents

* [The frameworks](#the-frameworks)
* [Performance](#performance)
* [Ecosystem and Community](#ecosystem-and-community)
* [JSON deserialization](#json-deserialization)
* [Routing](#routing)
* [Middleware](#middlewares)
* [State](#state)
* [Conclusion](#conclusion)

<!-- * [File upload](#file-upload) -->

> Want to learn Rust, Cybersecurity and Cryptography? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust)!

<!-- > Code examples can't be compiled as is, I've included them so you can make your own opinion about the API of each framework -->

## The frameworks


*as of February 2022*
| **crate**  | **Version**  | **Total Downloads** | **Description** |
| --- |  --- | --- | --- |
| [actix-web](https://crates.io/crates/actix-web) | 4.0.0-rc.3 | 5,134,720 | Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust  |
| [warp](https://crates.io/crates/warp) | 0.3.2 | 4,114,095 | serve the web at warp speeds  |
| [axum](https://crates.io/crates/axum) | 0.4.5 | 235,150 |  Web framework that focuses on ergonomics and modularity  (By the tokio team)  |


There are many other frameworks such as:
* [poem](https://crates.io/crates/poem)
* [hyper](https://crates.io/crates/hyper)
* [rocket](https://crates.io/crates/rocket)
* [tide](https://crates.io/crates/tide)
* [rouille](https://crates.io/crates/rouille)
* [iron](https://crates.io/crates/iron)
* ...

But they were not included because they are too young, too low-level, lack `async-await` support, don't support [tokio](https://crates.io/crates/tokio) or are no really longer maintained.

You can find the most up-to-date list: [https://www.arewewebyet.org/topics/frameworks](https://www.arewewebyet.org/topics/frameworks/).


## Performance

All these frameworks have good enough™ performance for most use cases. Thus we won't spend time on overoptimized micro-benchmarks that won't teach us anything about the real world.

If you have specific requirements such as millions of requests per second on a single machine, then I let you do your own benchmark with your specific setup.

## Ecosystem and Community

A good web framework needs both a good community to help you, and third-party packages to either save you time by using them directly or finding inspiration.


*as of February 2022*
| **crate**  | **GitHub stars**  | **3rd party crates** | **number of official examples** | **Open issues** | **Closed issues** |
| --- |  --- | --- | --- | --- | --- |
| [actix-web](https://crates.io/crates/actix-web) | ~13.3k | ~500 | [57](https://github.com/actix/examples) | 95 | 1234 |
| [warp](https://crates.io/crates/warp) | ~6k | ~184 | [24](https://github.com/seanmonstar/warp/tree/master/examples) | 134 | 421 |
| [axum](https://crates.io/crates/axum) | ~3.6k | ~50 | [36](https://github.com/tokio-rs/axum/tree/main/examples) | 6 | 192 |


**winner**: Being famous for its performance in the [TechEmpower Web Framework Benchmarks](https://www.techempower.com/benchmarks/#section=data-r20), `actix` is the framework with the biggest ecosystem and community.

That being said, `axum` is part of the [tokio](https://github.com/tokio-rs/) project and thus benefits from its huge ecosystem and community.


## JSON deserialization


### actix-web

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Hello {
    message: String,
}

async fn index(item: web::Json<Hello>) -> HttpResponse {
    HttpResponse::Ok().json(item.message) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### warp

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Hello {
    message: String,
}

async fn index(item: Hello) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&hello.message))
}

#[tokio::main]
async fn main() {
    let promote = warp::post()
        .and(warp::body::json())
        .map(index);

    warp::serve(promote).run(([127, 0, 0, 1], 8080)).await
}
```

### axum

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Hello {
    message: String,
}

async fn index(item: Json<Hello>) ->impl IntoResponse { {
    Json(item.message)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Winner

All frameworks use generics to provide easy JSON deserialization.

That being said, I find both `axum` and `actix-web` to be more straightforward to use with their helpers to automatically extract typed body payloads.


## Routing

### actix-web

```rust
fn main() {
    App::new()
        .service(web::resource("/").route(web::get().to(api::list)))
        .service(web::resource("/todo").route(web::post().to(api::create)))
        .service(web::resource("/todo/{id}")
          .route(web::post().to(api::update))
          .route(web::delete().to(api::delete)),
        );
}
```

### warp

```rust
pub fn todos() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    todos_list(db.clone())
        .or(todos_create(db.clone()))
        .or(todos_update(db.clone()))
        .or(todos_delete(db))
}

pub fn todos_list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and_then(handlers::list_todos)
}

pub fn todos_create() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::create_todo)
}

pub fn todos_update() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos" / u64)
        .and(warp::put())
        .and(json_body())
        .and_then(handlers::update_todo)
}

pub fn todos_delete() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos" / u64)
        .and(warp::delete())
        .and_then(handlers::delete_todo)
}

fn main() {
  let api = filters::todos(db);
  warp::serve(api).run(([127, 0, 0, 1], 8080)).await
}
```

### axum

```rust
    let app = Router::new()
        .route("/todos", get(todos_list).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete));
```

### Winner

`axum` is the clear winner, followed closely by `actix-web`.

Then comes `warp` which has a functional API favoring composition, far from what we usually expect for a web framework.


## Middlewares

### actix-web

```rust
pub struct SayHi;

impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("before");

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            println!("after");
            Ok(res)
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    App::new()
        .wrap(simple::SayHi)
        .service(
            web::resource("/").to(|| async {
                "Hello, middleware! Check the console where the server is run."
            }),
        )
}
```

### warp

```rust
pub fn json_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


fn main() {
    let api = api.and(warp::path("jobs"))
      .and(warp::path::end())
      .and(warp::post())
      .and(json_body())
      .and_then(create_job);
}
```

### axum

```rust
#[derive(Clone)]
struct MyMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for MyMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        println!("before");
        // best practice is to clone the inner service like this
        // see https://github.com/tower-rs/tower/issues/547 for details
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let res: Response = inner.call(req).await?;

            println!("after");

            Ok(res)
        })
    }
}

fn main() {
  let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .layer(layer_fn(|inner| MyMiddleware { inner }));
}
```

### Winner

`warp`, without a doubt...


## State

When building web services, you need to share some variables such as a Database connection pool or some clients for external services.

### actix-web


```rust
struct State {}

async fn index(
    state: Data<Arc<State>>,
    req: HttpRequest,
) -> HttpResponse {
  // ...
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let state = Arc::new(State {});

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### warp


```rust
struct State {}

pub fn with_state(
    state: Arc<State>,
) -> impl Filter<Extract = (Arc<State>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub async fn create_job(
    state: Arc<AppState>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // ...
}

fn main() {
    let state = Arc::new(State{});
    let api = api.and(warp::path("jobs"))
      .and(warp::path::end())
      .and(warp::post())
      .and(with_state(state))
      .and_then(create_job);
}
```

### axum

```rust
struct State {}

async fn handler(
    Extension(state): Extension<Arc<State>>,
) {
    // ...
}

fn main() {
    let shared_state = Arc::new(State {});

    let app = Router::new()
        .route("/", get(handler))
        .layer(AddExtensionLayer::new(shared_state));
}
```

### Winner

It's a draw. Ergonomics are very similar for all frameworks.


## Conclusion

My heart goes to `axum`. I find it has the cleanest API, it is built on top of [hyper](https://crates.io/crates/hyper), which is (certainly) the most tested and reliable HTTP stack in Rust, and because it is developed by tokio's team.

But its youthfulness might make some people uncomfortable.

For larger projects, I think that `actix-web` is the incontestable winner. That's why it's my choice for [Bloom](https://github.com/skerkour/bloom).

For smaller projects (up to ~30/40 routes), `warp` is very good, despite its original API, as it is also built on top of `hyper` and thus benefits from its reliability and performance.

Anyway, if you are using [a clean architecture](https://kerkour.com/rust-web-application-clean-architecture) switching from a framework to another should be easy, so [don't overthink it](https://kerkour.com/overthinking) and just start shipping :)


**Want to learn more? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build multiple web applications in Rust to phish credentials and control a RAT (Remote Access Tool).**
