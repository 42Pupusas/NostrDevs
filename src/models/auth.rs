use nostro2::{notes::Note, userkeys::UserKeys};
use serde_json::{Value, json};

const WHITELIST: [&str; 2] = [
    "3db609423efd172da2d4261c59a93210b58bce1fc9975ce858e7122452f3e85a", 
    "e33924a9953fd68dcd5fc8813c401f7b933d4f89dec7011f4349b35cab5a3de6",
];

pub struct AuthHandler {
    whitelist: Vec<&'static str>,
}

impl AuthHandler {

    pub fn new() -> AuthHandler {
        AuthHandler {
            whitelist: WHITELIST.to_vec(),
        }
    }

    pub fn is_authorized(&self, user_keys: &UserKeys) -> bool {
        let new_note = Note::new(&user_keys.get_public_key(), 4, "auth");
        let auth_note = user_keys.sign_nostr_event(new_note);

        let created_at = auth_note.get_created_at();
        let now = nostro2::utils::get_unix_timestamp();
        let is_recent = now - created_at < 60;
        
        let is_real_note = auth_note.verify();
        
        let note_id = auth_note.get_pubkey();
        let is_whitelisted = self.whitelist.contains(&note_id);
        is_real_note && is_whitelisted && is_recent
    }

    pub fn resource_filter(&self) -> Value {
        json!({
            "kinds": [9600],
            "authors": WHITELIST,
        })
    }

    pub fn inbox_filter(&self) -> Value {
        json!({
            "kinds": [4242],
            "authors": WHITELIST,
        })
    }
}
