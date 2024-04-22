use crate::components::toast::ErrorToast;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use gloo::console::{error, log};
use gloo_timers::callback::Timeout;
use nostro2::relays::RelayEvents;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, HtmlInputElement, SubmitEvent};
use yew::{
    function_component, html, platform::spawn_local, use_state, Callback, ContextProvider, Html,
    Reducible,
};
use yew::{use_context, use_effect_with, use_reducer, Properties, UseReducerHandle};

use nostro2::{
    notes::{Note, SignedNote},
    userkeys::UserKeys,
};


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckInNote {
    chek_in_note: Option<SignedNote>,
}

impl Reducible for CheckInNote {
    type Action = SignedNote;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Rc::new(Self {
            chek_in_note: Some(action),
        })
    }
}

pub type CheckInNoteContext = UseReducerHandle<CheckInNote>;

#[derive(Properties, Clone, PartialEq)]
pub struct NostrCheckInProps {
    pub send_note: Callback<SignedNote>,
    pub subscriber: Callback<Value>,
    pub clear_messages: Callback<()>,
    pub relay_events: HashMap<String, RelayEvents>,
    pub event_id: String,
}

#[function_component(NostrCheckIn)]
pub fn provider(props: &NostrCheckInProps) -> Html {
    let check_in_context = use_reducer(|| CheckInNote { chek_in_note: None });

    let check_in_clone = check_in_context.clone();
    let callback_clone = props.send_note.clone();
    use_effect_with(check_in_clone, move |check_in_clone| {
        if let Some(note) = &check_in_clone.chek_in_note {
            log!("Sending note:", note.to_string());
            callback_clone.emit(note.clone());
        }
        || {}
    });

    let subscriber = props.subscriber.clone();
    let clear_messages = props.clear_messages.clone();
    let event_id_clone = props.event_id.clone();
    use_effect_with((), move |()| {
        let filter = serde_json::json!({
            "kind": 42,
            "#d": [&event_id_clone],
        });
        subscriber.emit(filter);
        move || clear_messages.emit(())
    });

    let relay_events = props.relay_events.clone();
    let event_id_clone = props.event_id.clone();
    let unique_keys = use_state(|| 0);

    let unique_keys_clone = unique_keys.clone();
    use_effect_with(
        (relay_events.clone(), ),
        move |deps| {
            let relay_events = &deps.0;
            let mut unique_key_set = HashSet::new();

            for event in relay_events.values() {
                match event {
                    RelayEvents::EVENT(_, _, signed_note) => {
                        log!("Received note:", signed_note.to_string());
                        if signed_note.get_kind() != 42 {
                            continue;
                        }
                        if signed_note
                            .get_tags_by_id("d")
                            .unwrap()
                            .contains(&event_id_clone)
                        {
                            let public_key = signed_note.get_pubkey();
                            unique_key_set.insert(public_key);
                            unique_keys_clone.set(unique_key_set.len());
                        }
                    }
                    RelayEvents::EOSE(_, _id) => {}
                    _ => {}
                }
            }

            move || {}
        },
    );

    let event_id_clone = props.event_id.clone();
    html! {
        <>
            <ContextProvider<CheckInNoteContext> context={check_in_context}>
                <div class="flex flex-col gap-4 text-purple-400">
                <h1 class="text-xl text-white my-2 mt-6">{"Check In:"}</h1>
                <h2>{"Firma con tu llave privada:"}</h2>
                <KeyForm event_number={event_id_clone.clone()}/>
                <h2>{"O usa una extension NIP-07"}</h2>
                <NIP07Auth event_number={event_id_clone}/>
                <h2>{"Atendieron:"}</h2>
                <p>{&*unique_keys}</p>
                </div>
            </ContextProvider<CheckInNoteContext>>
        </>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct EventNumberProps {
    pub event_number: String,
}

#[function_component(KeyForm)]
fn form_component(props: &EventNumberProps) -> Html {
    let private_key = use_state(|| String::new());
    let check_in_context = use_context::<CheckInNoteContext>().unwrap();

    let errored = use_state(|| (false, String::new()));

    let private_key_onchange = {
        let private_key = private_key.clone();
        Callback::from(move |event: Event| {
            let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
            private_key.set(input.value());
        })
    };

    let errored_clone = errored.clone();
    let event_tag = props.event_number.clone();
    let onsubmit = {
        let private_key = private_key.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let user_keys = UserKeys::new(&*private_key);
            match user_keys {
                Ok(user_keys) => {
                    let mut check_in_note = Note::new(&user_keys.get_public_key(), 42, "Check In!");
                    check_in_note.add_tag("d", &event_tag);
                    let signed_note = user_keys.sign_nostr_event(check_in_note);
                    check_in_context.dispatch(signed_note);
                }
                Err(e) => {
                    errored_clone.set((true, e.to_string()));
                    return;
                }
            }
        })
    };

    let error_clone = errored.clone();
    let error_notice = match errored.0 {
        true => html! {<ErrorToast message={error_clone.1.clone()} />},
        false => html! {},
    };

    use_effect_with(errored, move |errored| {
        let errored = errored.clone();
        if errored.0 {
            Timeout::new(2000, move || {
                errored.set((false, String::new()));
            })
            .forget();
        }
        || {}
    });

    html! {
    <>
     <form onsubmit={onsubmit} class="flex flex-row gap-4 h-fit items-center">

         <input type="password" name="nostr_key" id="nostrKey" onchange={private_key_onchange}
            class="inline-block h-fit text-purple-950 placeholder:text-purple-950"
            placeholder="Llave Privada" required=true />

         <button id="loginButton" type="submit" class="">

             <svg class="h-12 w-12" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none"
                 viewBox="0 0 24 24">
                 <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                     d="m5 12 4.7 4.5 9.3-9" />
             </svg>

         </button>

     </form>
     {error_notice}
    </>
    }
}

#[wasm_bindgen(module = "/src/js/nip07.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn getPublicKey() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    async fn signEvent(note: String) -> Result<JsValue, JsValue>;
}

#[function_component(NIP07Auth)]
pub fn nip_component(props: &EventNumberProps) -> Html {
    let public_key = use_state(|| String::new());
    let unsigned_note = use_state(|| String::new());
    let check_in_note = use_context::<CheckInNoteContext>().unwrap();

    let errored = use_state(|| false);

    let event_tag = props.event_number.clone();
    let error_clone = errored.clone();
    let check_in_onclick = {
        Callback::from(move |_| {
            let public_key = public_key.clone();
            let first_note = unsigned_note.clone();
            let check_in_note = check_in_note.clone();
            let error_clone = error_clone.clone();
            let event_tag = event_tag.clone();
            log!("Getting public key");
            spawn_local(async move {
                let result = getPublicKey().await;
                let first_note = first_note.clone();
                match result {
                    Ok(value) => {
                        log!("Got public key: {:?}", value.clone());
                        public_key.set(value.as_string().unwrap());
                        let mut note = Note::new(&public_key, 42, "Check In!");
                        note.add_tag("d", &event_tag);
                        first_note.set(note.to_string());
                        let signed_note_result = signEvent(note.to_string()).await.unwrap();
                        let signed_note_struct = serde_json::from_str::<SignedNote>(
                            &signed_note_result.as_string().unwrap(),
                        )
                        .unwrap();
                        check_in_note.dispatch(signed_note_struct);
                    }

                    Err(e) => {
                        error!("Error getting public key: {:?}", e);
                        error_clone.set(true);
                    }
                }
            });
        })
    };

    let error_clone = errored.clone();
    let error_notice = match *error_clone {
        true => html! {<ErrorToast message="No Extension Found!" />},
        false => html! {},
    };

    let error_clone = errored.clone();
    use_effect_with(error_clone, move |error_clone| {
        let error_clone = error_clone.clone();
        if *error_clone {
            Timeout::new(2000, move || {
                error_clone.set(false);
            })
            .forget();
        }
        || {}
    });

    html! {
        <>
        <div>
            <button onclick={check_in_onclick} class="rounded-lg bg-white text-purple-950 px-2 py-4 m-4">
                {"NIP-07"}
            </button>
        </div>
        {error_notice}
        </>
    }
}
