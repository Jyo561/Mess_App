use common::{AttendanceEntry, LoginRequest, LoginResponse, MealType, SignupRequest};
use gloo_net::http::Request;
use crate::storage::{clear_session, get_session};

const API_BASE: &str = "http://localhost:3000/api";

/// Generic unauthenticated request for endpoints like /auth/login and /auth/signup
async fn unauth_request<B: serde::Serialize, T: for<'de> serde::Deserialize<'de>>(
    url: &str,
    body: &B,
) -> Result<T, String> {
    let response = Request::post(url)
        .json(body)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network connection error: {}", e))?;

    if !response.ok() {
        let status = response.status();
        let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error".into());
        return Err(format!("Server returned {}: {}", status, error_msg));
    }

    response
        .json::<T>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

/// Generic authenticated request (Your helper function)
pub async fn auth_request<B: serde::Serialize, T: for<'de> serde::Deserialize<'de>>(
    method: &str,
    url: &str,
    body: Option<&B>,
) -> Result<T, String> {
    let session = get_session().ok_or_else(|| "No session found. Please sign in.".to_string())?;

    let mut req = match method {
        "GET" => Request::get(url),
        "POST" => Request::post(url),
        _ => return Err("Unsupported HTTP method".into()),
    };

    req = req.header("Authorization", &format!("Bearer {}", session.token));

    // Fix: call .build()? in the else branch so both return `Request`
    let built_request = if let Some(b) = body {
        req.json(b).map_err(|e| format!("Serialization error: {}", e))?
    } else {
        req.build().map_err(|e| format!("Request build error: {}", e))?
    };

    let response = built_request
        .send()
        .await
        .map_err(|e| format!("Network connection error: {}", e))?;

    // Handle session expiry
    if response.status() == 401 {
        clear_session();
        return Err("Session expired. Please log in again.".into());
    }

    if !response.ok() {
        let status = response.status();
        let error_msg = response.text().await.unwrap_or_else(|_| "Unknown error".into());
        return Err(format!("Server returned {}: {}", status, error_msg));
    }

    // Safely deserialize valid JSON
    response
        .json::<T>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

// -------------------------------------------------------------
// Public Endpoints (No Auth Header)
// -------------------------------------------------------------

pub async fn login(req: LoginRequest) -> Result<LoginResponse, String> {
    unauth_request(&format!("{API_BASE}/auth/login"), &req).await
}

pub async fn signup(req: SignupRequest) -> Result<LoginResponse, String> {
    unauth_request(&format!("{API_BASE}/auth/signup"), &req).await
}

// -------------------------------------------------------------
// Authenticated Endpoints (Leverage your `auth_request`)
// -------------------------------------------------------------

pub async fn get_available_meals() -> Result<Vec<MealType>, String> {
    auth_request::<(), Vec<MealType>>("GET", &format!("{API_BASE}/meals"), None).await
}

pub async fn submit_entry(entry: AttendanceEntry) -> Result<AttendanceEntry, String> {
    auth_request("POST", &format!("{API_BASE}/entry"), Some(&entry)).await
}

pub async fn get_history() -> Result<Vec<AttendanceEntry>, String> {
    auth_request::<(), Vec<AttendanceEntry>>("GET", &format!("{API_BASE}/history"), None).await
}
