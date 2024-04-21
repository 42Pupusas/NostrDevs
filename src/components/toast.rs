use yew::prelude::*;
use gloo_timers::callback::Timeout;

use super::nostr::RelayNotice;

#[derive(Clone, Properties, PartialEq)]
pub struct ToastNotificationsProps {
    pub relay_notices: Option<RelayNotice>
}

#[function_component(ToastNotifications)]
pub fn toast_notifications(props: &ToastNotificationsProps) -> Html {
    let relay_notices = props.relay_notices.clone();
    let should_display = use_state(|| false);
    let success = use_state(|| false);

    let should_display_clone = should_display.clone();
    let success_clone = success.clone();
    use_effect_with(relay_notices, |relay_notices| {
        if let Some(notice) = relay_notices {
            match notice {
                RelayNotice::NoteSent(_id) => {
                    should_display_clone.set(true);
                    success_clone.set(true);
                    Timeout::new(2000, move || {
                        should_display_clone.set(false);
                        success_clone.set(false);
                    })
                    .forget();
                }
                RelayNotice::NoteNotSent(_id) => {
                    should_display_clone.set(true);
                    Timeout::new(2000, move || {
                        should_display_clone.set(false);
                    })
                    .forget();
                }
            }
        }
        || {}
    });

    html! {
        <>
            {if *should_display {
                if *success {
                    html! {<SuccessToast />}
                } else {
                    html! {<ErrorToast />}
                }
            } else {
                html! {}
            }}
        </>
    }
}

#[function_component(SuccessToast)]
pub fn success_toast() -> Html {
    html! {
        <div class="toastSuccess">
            <p>{"Success!"}</p>
        </div>
    }
}

#[function_component(ErrorToast)]
pub fn error_toast() -> Html {
    html! {
        <div class="toastDanger">
            <p>{"Failed!"}</p>
        </div>
    }
}

