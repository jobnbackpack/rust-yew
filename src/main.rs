use pulldown_cmark::{Options, Parser, html};
use yew::prelude::*;
use yew_router::prelude::*;
use log::info;

mod components;
mod pages;
use components::navbar::Navbar;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/test")]
    Test,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn App() -> Html {
    html!{
        <BrowserRouter>
            <Navbar />
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

#[function_component(Test)]
fn test() -> Html {
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

    Html::from_html_unchecked(html_output.into()) 
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Test => html! {
            <Test />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
