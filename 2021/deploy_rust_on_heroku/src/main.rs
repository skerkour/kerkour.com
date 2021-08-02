use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))?;

    // GET /hello
    let hello = warp::path("hello").and(warp::get()).map(|| "World");

    // GET /world
    let world = warp::path("world").and(warp::get()).map(|| "Hello");

    // fallback
    let fallback = warp::any().map(|| "Not found");

    let routes = hello.or(world).or(fallback);

    let (_addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], port), async {
            tokio::signal::ctrl_c()
                .await
                .expect("http_server: Failed to listen for CRTL+c");
            println!("Shutting down HTTP server");
        });

    server.await;

    Ok(())
}
