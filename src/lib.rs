use yew::prelude::*;

mod components;
mod models;
mod pages;
mod router;

use components::{keys::NostrKeysProvider, nostr::NostrComponent};

#[function_component(App)]
pub fn app() -> Html {
    html! {
            <NostrComponent />
    }
}
