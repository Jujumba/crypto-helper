use yew::{function_component, html, Html};
use yew_router::prelude::Link;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <Link<Route> to={Route::Home}>{"Crypto helper"}</Link<Route>>
            <Link<Route> to={Route::Jwt}>{"JWT/JWE"}</Link<Route>>
            <Link<Route> to={Route::About}>{"About"}</Link<Route>>
        </header>
    }
}
