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
