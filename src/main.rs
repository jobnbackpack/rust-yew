use yew::prelude::*;
use yew_router::prelude::*;

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
    html! {
        <div>
            <h1>{ "Test" }</h1>
        </div>
    }
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
    yew::Renderer::<App>::new().render();
}
