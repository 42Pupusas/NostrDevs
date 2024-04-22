use yew::prelude::*;

mod components;
mod models;
mod pages;
mod router;

use components::nostr::NostrComponent;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <NostrComponent />
        </>
    }
}
