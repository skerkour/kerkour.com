pub fn format_body(body: &str) -> String {
    format!(
        r#"
<body>
    {}
</body>
"#,
        body
    )
}
