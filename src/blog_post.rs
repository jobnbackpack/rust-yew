use pulldown_cmark::{html, Options, Parser};
use yew::prelude::*;

pub struct Post {
    pub meta: Meta,
    pub content: Html
}

impl Post {
    pub fn generate_from_name(name: String) -> Self {
    let test_markdown_input = "# Hello world
* first point
* second
* ~~strike~~ **bold**
";

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(test_markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let content = Html::from_html_unchecked(html_output.into());
        Self {
            meta: Meta::generate(name),
            content
        }
    }
}

pub struct Meta {
    pub title: String
}

impl Meta {
    pub fn generate(title: String) -> Self {
        Self {
            title
        }
    }
}
