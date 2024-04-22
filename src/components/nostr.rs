use crate::components::keys::{NostrCheckIn, NostrCheckInProps};
use crate::components::layout::Layout;
use crate::components::toast::{ToastNotifications, ToastNotificationsProps};
use crate::pages::event_1::Event1;
use crate::pages::home::Home;
use crate::router::MainPanelRoute;

use std::{collections::HashMap, sync::Arc};

use async_channel::{unbounded, Sender};
use nostro2::{
    notes::SignedNote,
    relays::{NostrRelay, RelayEvents},
};
use serde_json::Value;

use yew::platform::spawn_local;
use yew::{prelude::*, props};
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RelayNotice {
    NoteSent(String),
    NoteNotSent(String),
}

pub enum Msg {
    RelayMessage(RelayEvents),
    NewNote(SignedNote),
    NewFilter(Value),
    Unsubscribe(String),
    ClearMessages,
}

#[derive(Properties, Clone, PartialEq)]
pub struct NostrProps {
    pub relay_events: HashMap<String, RelayEvents>,
    pub relay_notice: Option<RelayNotice>,
    pub send_note: Callback<SignedNote>,
    pub subscriber: Callback<Value>,
    pub unsubscriber: Callback<String>,
    pub clear_messages: Callback<()>,
}

pub struct NostrComponent {
    relay_events: HashMap<String, RelayEvents>,
    relay_notice: Option<RelayNotice>,
    sender_channel: Sender<SignedNote>,
    filter_channel: Sender<Value>,
    subscription_id_channel: Sender<String>,
    send_note_callback: Callback<SignedNote>,
    filter_callback: Callback<Value>,
    subscription_id_callback: Callback<String>,
    clear_messages: Callback<()>,
}

impl Component for NostrComponent {
    type Message = Msg;
    type Properties = ();

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let event_1_props = props!(NostrCheckInProps {
            send_note: self.send_note_callback.clone(),
            subscriber: self.filter_callback.clone(),
            clear_messages: self.clear_messages.clone(),
            relay_events: self.relay_events.clone(),
            event_id: "Evento1Test".to_string(),
        });

        let toast_props = props!(ToastNotificationsProps {
            relay_notices: self.relay_notice.clone(),
        });

        html! {
            <BrowserRouter>
                <Layout>
                    <Switch<MainPanelRoute> render = { move |switch: MainPanelRoute|{
                        match switch {
                            MainPanelRoute::Home => html!{
                                <Home />
                            },
                            MainPanelRoute::Event1 => html!{
                                <div class="w-full h-full flex flex-col gap-8 sm:flex-row sm:justify-between">
                                    
                                <Event1 />
                                <NostrCheckIn  ..event_1_props.clone() />
                                </div>
                            },
                        }

                    }} /> // <- must be child of <BrowserRouter>
                </Layout>
                <ToastNotifications ..toast_props/>
            </BrowserRouter>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        let message_cb = ctx.link().callback(Msg::RelayMessage);
        let (sender_channel, filter_channel, subscription_id_channel) =
            Self::read_relay(message_cb);
        let send_note_callback = ctx.link().callback(Msg::NewNote);
        let filter_callback = ctx.link().callback(Msg::NewFilter);
        let subscription_id_callback = ctx.link().callback(Msg::Unsubscribe);
        Self {
            relay_events: HashMap::new(),
            relay_notice: None,
            sender_channel,
            filter_channel,
            subscription_id_channel,
            send_note_callback,
            filter_callback,
            subscription_id_callback,
            clear_messages: ctx.link().callback(|_| Msg::ClearMessages),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RelayMessage(msg) => match &msg {
                RelayEvents::EVENT(_, _, signed_note) => {
                    self.relay_events
                        .insert(signed_note.get_id().to_string(), msg);
                }
                RelayEvents::EOSE(_, id) => {
                    self.relay_events.insert(id.to_string(), msg);
                }
                RelayEvents::OK(_, note_id, did_send, _) => {
                    if *did_send {
                        self.relay_notice = Some(RelayNotice::NoteSent(note_id.to_string()));
                    } else {
                        self.relay_notice = Some(RelayNotice::NoteNotSent(note_id.to_string()));
                    }
                }
                _ => {}
            },
            Msg::ClearMessages => {
                self.relay_events.clear();
            }
            Msg::NewNote(note) => {
                self.send_nostr_note(note);
            }
            Msg::NewFilter(filter) => {
                self.subscribe_to_filter(filter);
            }
            Msg::Unsubscribe(id) => {
                self.unsubscribe_to_filter(id);
            }
        }
        true
    }
}

impl NostrComponent {
    fn read_relay(
        note_cb: Callback<RelayEvents>,
    ) -> (Sender<SignedNote>, Sender<Value>, Sender<String>) {
        let (note_tx, note_rx) = unbounded::<SignedNote>();
        let (filter_tx, filter_rx) = unbounded::<Value>();
        let (subscription_id_tx, subscription_id_rx) = unbounded::<String>();

        spawn_local(async move {
            let relay = NostrRelay::new("wss://relay.arrakis.lat").await.unwrap();
            let relay_arc = Arc::new(relay);

            let sender_relay = relay_arc.clone();
            spawn_local(async move {
                while let Ok(note) = note_rx.recv().await {
                    sender_relay.send_note(note).await.unwrap();
                }
            });

            let reader_relay = relay_arc.clone();
            spawn_local(async move {
                let subscriber = reader_relay.clone();
                spawn_local(async move {
                    while let Ok(filter) = filter_rx.recv().await {
                        subscriber.subscribe(filter).await.unwrap();
                    }
                });

                let unsub_relay = reader_relay.clone();
                spawn_local(async move {
                    while let Ok(sub_id) = subscription_id_rx.recv().await {
                        unsub_relay.unsubscribe(sub_id).await.unwrap();
                    }
                });

                while let Ok(event) = reader_relay.read_relay_events().await {
                    note_cb.emit(event);
                }
            });
        });
        (note_tx, filter_tx, subscription_id_tx)
    }

    pub fn _build_props(&self) -> NostrProps {
        props!(NostrProps {
            relay_events: self.relay_events.clone(),
            relay_notice: self.relay_notice.clone(),
            send_note: self.send_note_callback.clone(),
            subscriber: self.filter_callback.clone(),
            unsubscriber: self.subscription_id_callback.clone(),
            clear_messages: self.clear_messages.clone(),
        })
    }

    fn send_nostr_note(&self, signed_note: SignedNote) {
        self.sender_channel.try_send(signed_note).unwrap();
    }

    fn subscribe_to_filter(&self, filter: Value) {
        self.filter_channel.try_send(filter).unwrap();
    }

    fn unsubscribe_to_filter(&self, id: String) {
        self.subscription_id_channel.try_send(id).unwrap();
    }
}
