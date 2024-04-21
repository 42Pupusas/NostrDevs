use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct EventTemplateProps {
    pub event_name: String,
    pub date: String,
    pub meeting_link: String,
    pub announcements: Vec<String>,
    pub topics: Vec<(String, String)>,
}

#[function_component(EventTemplate)]
pub fn event_template(props: &EventTemplateProps) -> Html {
    html! {
        <div class="text-purple-300 align-bottom">
            <h3 class="text-xl text-white my-2">{props.event_name.clone()}</h3>
            <div class="flex flex-row gap-4">
                <p class="align-bottom">{format!("Fecha: {}", props.date)}</p>
                <a href={props.meeting_link.clone()} target="_blank">
                    <p class="text-purple-300 hover:text-purple-600 hover:cursor-pointer underline">{"Link de Evento"}</p>
                </a>
            </div>
            <h3 class="text-xl text-white my-2 mt-6">{"Anuncios"}</h3>
            <ul class="ml-8 list-disc">
                { for props.announcements.iter().map(|announcement| html! { <li>{announcement}</li> }) }
            </ul>
            <h3 class="text-xl text-white my-2 mt-6">{"Temas A Discutir"}</h3>
            <ul class="ml-8 list-disc">
            { for props.topics.iter().map(|topic| 
                html! { 
                    <li>
                        <a
                            target="_blank"
                            href={topic.1.clone()}
                            class="text-purple-300 hover:text-purple-600 hover:cursor-pointer underline">{topic.0.clone()}
                        </a>
                    </li> 
                }) 
            }
            </ul>
        </div>
    }
}
