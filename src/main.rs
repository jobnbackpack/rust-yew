use yew::prelude::*;
use yew_router::prelude::*;
use log::info;

mod components;
mod pages;
mod blog_post;
use components::navbar::Navbar;
use pages::blog_post_view::PostComponent;

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

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Test => html! {
            <PostComponent name="Hello" />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
