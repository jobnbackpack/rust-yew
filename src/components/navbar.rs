use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route; 

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class="navbar">
            <h2>{ "Jobnbackpack" }</h2>
            <section class="links">
                <Link<Route> to={Route::Home}>
                    { "Home" }
                </Link<Route>>
                <Link<Route> to={Route::Test}>
                    { "Test" }
                </Link<Route>>
            </section>
        </div>
    }
}
