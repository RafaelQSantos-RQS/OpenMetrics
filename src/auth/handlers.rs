use axum::response::Html;
use askama::Template;
use crate::templates::*;

pub async fn login_page() -> Html<String> {
    let template = LoginTemplate;
    Html(template.render().unwrap_or_else(|e| format!("Template error: {}", e)))
}

pub async fn login_submit() -> Html<String> {
    // TODO: Implement credential validation
    Html("<p>Login not yet implemented</p>".to_string())
}

pub async fn logout() -> Html<String> {
    // TODO: Implement session clearing
    Html("<p>Logout not yet implemented</p>".to_string())
}
