use yew::{function_component, html, Html};

use crate::components::event_template::EventTemplate;

#[function_component(Event1)]
pub fn event_1() -> Html {

    html! {
        <EventTemplate  
            event_name={"Evento Inaugural".to_string()}
            date={"2024-04-24".to_string()}
            meeting_link={"https://www.eventbrite.co/e/nostr-dev-reunion-el-salvador-aprender-y-conocer-tickets-884565548367?aff=oddtdtcreator".to_string()}
            announcements={vec![
                "Respeta la privacidad de los demas.".to_string(),
                "Las preguntas y aportes son bienvenidas.".to_string(),
            ]}
            topics={vec![
                ("Introduccion a Nostr".to_string(), "https://nips.nostr.com/".to_string()),
                ("Descentralizacion, Censura, Comodidad".to_string(), "https://twitter.com/jack/status/1666076985242836993".to_string()),
                ("Gossip Model vs Distribucion Masiva".to_string(), "https://mikedilger.com/gossip-model/".to_string()),
                ("Fondos Open-Source".to_string(), "https://opensats.org/funds/nostr".to_string()),
                ("ONOSENDAI/Protocolo CyberSpace".to_string(), "https://github.com/arkin0x/ONOSENDAI?tab=readme-ov-file".to_string()),
            ]}
            />
    }
}
