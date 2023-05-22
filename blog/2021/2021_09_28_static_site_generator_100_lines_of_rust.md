+++
date = 2021-09-28T13:00:00Z
title = "Building a static site generator in 100 lines of Rust"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-static-site-generator"

[extra]
lang = "en"

comment ="""

"""
+++

And by that, I mean **exactly** 100 lines (excluding templates), with hot reload and an embedded web server 😃

Conceptually, a static site generator is straightforward.

It takes some files as input, often markdown, render them, merge them with pre-defined templates, and output everything as raw HTML files. Simple, basic.

In ours, we will embed a web server to preview the websites when files change.

![The architecture of a static site generator](https://kerkour.com/2021/rust-static-site-generator/static_site_generator.svg)


Here is the `Cargo.toml` file we are going to use:

**Cargo.toml**
```toml
[package]
name = "rust_static_site_generator"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
pulldown-cmark = "0.8.0"
hotwatch = "0.4"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
walkdir = "2"
axum = "0.2"
tower-http = { version = "0.1", features = ["fs"] }
```

## Templates

For our templates, we use plain Rust Strings, with the [`format!`](https://doc.rust-lang.org/std/macro.format.html) macro:

**templates.rs**
```rust
pub const HEADER: &str = r#"<!DOCTYPE html>
<html lang="en">

  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
  </head>

"#;

pub fn render_body(body: &str) -> String {
    format!(
        r#"  <body>
    <nav>
        <a href="/">Home</a>
    </nav>
    <br />
    {}
  </body>"#,
        body
    )
}

pub const FOOTER: &str = r#"

</html>
"#;
```

## Rebuilding the site on change

In order to detect files changes, we use [hotwatch](https://crates.io/crates/hotwatch), a simple wrapper over [notify](https://github.com/notify-rs/notify) that will allow us to save a few lines.

We fist build the website on startup, and then each time a change is detected in the `content` folder.

**main.rs**
```rust
use axum::{http::StatusCode, service, Router};
use std::{convert::Infallible, fs, net::SocketAddr, path::Path, thread, time::Duration};
use tower_http::services::ServeDir;

mod templates;
const CONTENT_DIR: &str = "content";
const PUBLIC_DIR: &str = "public";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    rebuild_site(CONTENT_DIR, PUBLIC_DIR).expect("Rebuilding site");

    tokio::task::spawn_blocking(move || {
        println!("listenning for changes: {}", CONTENT_DIR);
        let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");
        hotwatch
            .watch(CONTENT_DIR, |_| {
                println!("Rebuilding site");
                rebuild_site(CONTENT_DIR, PUBLIC_DIR).expect("Rebuilding site");
            })
            .expect("failed to watch content folder!");
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });

    // ...
}
```

We build the website the brutal way:
* We delete the entire `public` folder
* Iterate over all the `.md` files in the `content` folder
* And render them to HTML files in a new, empty, `public` folder
* for each file, we make sure that the parent folder exists (for example: in `content/blog/hello.md` -> `public/blog/hello.html` the `blog` subfolder is preserved)

We also keep a list of all the generated HTML files in order to add them to the index of our static site.

**main.rs**
```rust
fn rebuild_site(content_dir: &str, output_dir: &str) -> Result<(), anyhow::Error> {
    let _ = fs::remove_dir_all(output_dir);

    let markdown_files: Vec<String> = walkdir::WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| e.path().display().to_string())
        .collect();
    let mut html_files = Vec::with_capacity(markdown_files.len());

    for file in &markdown_files {
        let mut html = templates::HEADER.to_owned();
        let markdown = fs::read_to_string(&file)?;
        let parser = pulldown_cmark::Parser::new_ext(&markdown,  pulldown_cmark::Options::all());

        let mut body = String::new();
        pulldown_cmark::html::push_html(&mut body, parser);

        html.push_str(templates::render_body(&body).as_str());
        html.push_str(templates::FOOTER);

        let html_file = file
            .replace(content_dir, output_dir)
            .replace(".md", ".html");
        let folder = Path::new(&html_file).parent().unwrap();
        let _ = fs::create_dir_all(folder);
        fs::write(&html_file, html)?;

        html_files.push(html_file);
    }

    write_index(html_files, output_dir)?;
    Ok(())
}
```

## Generating the Index

After building all the pages of our site, we need to render the index to enable our visitor to naviguate to our pages. For that, we render the list of pages as HTML links:

**main.rs**
```rust
fn write_index(files: Vec<String>, output_dir: &str) -> Result<(), anyhow::Error> {
    let mut html = templates::HEADER.to_owned();
    let body = files
        .into_iter()
        .map(|file| {
            let file = file.trim_start_matches(output_dir);
            let title = file.trim_start_matches("/").trim_end_matches(".html");
            format!(r#"<a href="{}">{}</a>"#, file, title)
        })
        .collect::<Vec<String>>()
        .join("<br />\n");

    html.push_str(templates::render_body(&body).as_str());
    html.push_str(templates::FOOTER);

    let index_path = Path::new(&output_dir).join("index.html");
    fs::write(index_path, html)?;
    Ok(())
}
```

## The Web server

Finally, we need a web server to preview the pages when writing and editing the content. I have chosen the new [axum](https://github.com/tokio-rs/axum) framework by [tokio's team](https://github.com/tokio-rs/) because I find its API very good and intuitive, all while being built on top of [hyper](https://crates.io/crates/hyper) and thus being extremely reliable.

**main.rs**
```rust
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // ...

    let app = Router::new().nest(
        "/",
        service::get(ServeDir::new(PUBLIC_DIR)).handle_error(|error: std::io::Error| {
            Ok::<_, Infallible>((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            ))
        }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("serving site on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

And, the last missing piece: a Markdown page!


**content/hello-world.md**
```markdown
# Hellow world

Cool
```


You can serve your static site locally by running:

```shell
$ cargo run
```

Then visit [http://localhost:8080](http://localhost:8080)

![Index of the static site](https://kerkour.com/2021/rust-static-site-generator/static_site_index.png)


![Preview of the static site](https://kerkour.com/2021/rust-static-site-generator/static_site.png)

Cute, isn't it?


## Going further

Of course, our little baby is not perfect and more work is required to turn it into a reliable static site generator, but you know the refrain: It's left as an exercise for the reader 😉

* Adding CSS (such as [pico.css](https://picocss.com/) or [mvp.css](https://github.com/andybrewer/mvp/))
* Customizable templates
* Handle errors and edge cases
* Client-side hot reload
* Only rebuild the files that changed
* `sitemap.xml` (as easy as `index.html`)
* And many more things...


## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_static_site_generator) (please don't forget to star the repo 🙏).
