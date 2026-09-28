use common::LoginResponse;
use web_sys::window;

const SESSION_KEY: &str = "enclave_user_session";

pub fn save_session(session: &LoginResponse) {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(json) = serde_json::to_string(session) {
                let _ = storage.set_item(SESSION_KEY, &json);
            }
        }
    }
}

pub fn get_session() -> Option<LoginResponse> {
    let win = window()?;
    let storage = win.local_storage().ok()??;
    let raw = storage.get_item(SESSION_KEY).ok()??;
    serde_json::from_str(&raw).ok()
}

pub fn clear_session() {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            let _ = storage.remove_item(SESSION_KEY);
        }
    }
}
