use pulldown_cmark::{html, Options, Parser};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use yew::prelude::*;

pub struct Post {
    pub meta: Meta,
    pub content: Html
}

impl Post {
    pub fn generate_from_name(name: String) -> Self {
    // let markdown_content = fs::read_to_string("./posts/test.md").expect("Should have been able to read the file.");
    let markdown_content = read_file("./posts/test.md").expect("FAIL");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_content[..], options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let content = Html::from_html_unchecked(html_output.into());
        Self {
            meta: Meta::generate(name),
            content
        }
    }
}

#[wasm_bindgen(module = "/src/readFile.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn read_file(path: &str) -> Result<String, JsValue>;
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
