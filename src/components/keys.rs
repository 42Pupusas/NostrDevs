use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, SubmitEvent};
use yew::{
    function_component, html, use_context, use_reducer, use_state, Callback, ContextProvider, Html,
    Properties, Reducible, UseReducerHandle,
};

use nostro2::{userkeys::UserKeys, notes::{SignedNote, Note}};

use crate::models::auth::AuthHandler;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserNostrKeys {
    user_keys: Option<UserKeys>,
}

impl UserNostrKeys {
    pub fn get_pubkey(&self) -> String {
        self.user_keys.as_ref().unwrap().get_public_key()
    }
    pub fn sign_note(&self, note: Note) -> SignedNote{
        self.user_keys.as_ref().unwrap().sign_nostr_event(note)
    }
}

impl Reducible for UserNostrKeys {
    type Action = String;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let user_keys = UserKeys::new(&action).unwrap();
        let auth_handler = AuthHandler::new();
        if !auth_handler.is_authorized(&user_keys) {
            return self;
        };
        UserNostrKeys {
            user_keys: Some(user_keys),
        }
        .into()
    }
}

pub type UserNostrKeysContext = UseReducerHandle<UserNostrKeys>;

#[derive(Properties, Debug, PartialEq)]
pub struct NostrKeysProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn NostrKeysProvider(props: &NostrKeysProviderProps) -> Html {
    let user_keys = use_reducer(|| UserNostrKeys { user_keys: None });

    let app_body = match &user_keys.user_keys {
        Some(_) => {
            html! {
                {props.children.clone()}
            }
        }
        None => {
            html! {
                <>
                    <KeyForm />
                </>
            }
        }
    };

    html! {
        <>
            <ContextProvider<UserNostrKeysContext> context={user_keys}>
                {app_body}
            </ContextProvider<UserNostrKeysContext>>
        </>
    }
}

#[function_component]
fn KeyForm() -> Html {
    let user_keys = use_context::<UserNostrKeysContext>().unwrap();
    let private_key = use_state(|| String::new());

    let private_key_onchange = {
        let private_key = private_key.clone();
        Callback::from(move |event: Event| {
            let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
            private_key.set(input.value());
        })
    };

    let onsubmit = {
        let private_key = private_key.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            user_keys.dispatch((&*private_key).clone());
        })
    };

    html! {
    <div class="flex h-screen w-full bg-black text-white">

        <div class="flex h-full w-full flex-col items-center justify-center px-4 py-16 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-lg text-center">
                <h3 class="text-2xl font-bold sm:text-3xl">{"Soma Admin"}</h3>
                <span class="my-4 ">
                    {"Speak the word, friend, and enter."}
                </span>
            </div>

            <form onsubmit={onsubmit} class="mx-auto mb-0 mt-8 max-w-md flex flex-col gap-8 items-center">

                <input type="password" name="nostr_key" id="nostrKey" onchange={private_key_onchange}
                    class="placeholder:font-monse w-full rounded-lg border-white bg-black p-4 pe-12 text-sm text-white shadow-sm placeholder:text-xs placeholder:text-white sm:placeholder:text-lg"
                    placeholder="Llave Privada" />

                <button id="loginButton" type="submit" class="inline-block rounded-lg bg-white text-black">

                    <svg class="h-12 w-12" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none"
                        viewBox="0 0 24 24">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="m5 12 4.7 4.5 9.3-9" />
                    </svg>

                </button>



            </form>
        </div>

    </div>
    }
}
