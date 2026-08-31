pub mod handlers;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Clone)]
pub struct AuthState {
    pub username: String,
    pub password_hash: String,
    pub session_secret: String,
    pub session_expiry_hours: u64,
}

impl AuthState {
    pub fn from_config(config: &crate::config::AuthConfig) -> Self {
        AuthState {
            username: config.username.clone(),
            password_hash: config.password_hash.clone(),
            session_secret: config.session_secret.clone(),
            session_expiry_hours: config.session_expiry_hours,
        }
    }
}

pub struct Unauthorized;

impl IntoResponse for Unauthorized {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}
