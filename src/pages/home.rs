use yew::{function_component, html, Html, Properties};

const INTRO_TEXT: &str = r#"
    Somos una comunidad que busca crear espacios públicos para discutir y aprender sobre la red Nostr, 
    un protocolo de comunicación descentralizado, seguro y abierto.
"#;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="flex flex-col gap-8">
            <p class="text-purple-300">{INTRO_TEXT}</p>

            <div class="flex flex-col gap-4">
                <h2 class="text-lg font-bold">{"Eventos Recientes y Proximos"}</h2>
                <ul class="flex flex-col gap-2">
                    <li>
                        <EventEntry date="2024-04-24" title="Meetup #1" link="eventN1"/>
                    </li>
                </ul>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct EventEntryProps {
    pub date: String,
    pub title: String,
    pub link: String,
}

#[function_component(EventEntry)]
pub fn event_entry(props: &EventEntryProps) -> Html {
    html! {
        <div class="flex flex-row gap-2 items-center">
            <p class="font-bold">{props.date.clone()}</p>
            <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m9 5 7 7-7 7"/>
            </svg>
            <a href={props.link.clone()} class="text-purple-300 hover:text-purple-600">
                <p>{props.title.clone()}</p>
            </a>
        </div>
    }
}
