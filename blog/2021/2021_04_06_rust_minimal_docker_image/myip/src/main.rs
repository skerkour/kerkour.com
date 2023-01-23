use serde::Deserialize;
use std::error::Error;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Deserialize, Debug)]
struct ApiRes {
    ip: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let res = reqwest::blocking::get("https://api.myip.com")?.json::<ApiRes>()?;

    println!("{}", res.ip);

    Ok(())
}
