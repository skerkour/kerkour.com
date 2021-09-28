use pulldown_cmark::{html, Options, Parser};

mod templates;

fn main() {
    let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    println!("{}", html_output);
}
