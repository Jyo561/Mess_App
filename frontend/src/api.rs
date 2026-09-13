use common::LoginResponse;
use gloo_net::http::Request;
use crate::storage::{clear_session, get_session};

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

    let req_with_body = if let Some(b) = body {
        req.json(b).map_err(|e| format!("Serialization error: {}", e))?
    } else {
        req
    };

    let response = req_with_body
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
