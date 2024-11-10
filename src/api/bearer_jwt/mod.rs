use crate::RouterState;
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};

mod login;
mod protected_content;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
}

pub fn create_bearer_jwt_router(state: RouterState) -> Router {
    Router::new()
        .route(
            "/bearer/page",
            get(protected_content::protected_bearer_content),
        )
        .route("/bearer/login", post(login::api_login_cookie_jwt))
        .with_state(state)
}
