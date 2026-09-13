use common::LoginResponse;
use web_sys::window;

const SESSION_KEY: &str = "enclave_user_session";

pub fn get_session() -> Option<LoginResponse> {
    let storage = window()?.local_storage().ok()??;
    let raw = storage.get_item(SESSION_KEY).ok()??;
    serde_json::from_str(&raw).ok()
}

pub fn save_session(session: &LoginResponse) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(serialized) = serde_json::to_string(session) {
            let _ = storage.set_item(SESSION_KEY, &serialized);
        }
    }
}

pub fn clear_session() {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.remove_item(SESSION_KEY);
    }
}
