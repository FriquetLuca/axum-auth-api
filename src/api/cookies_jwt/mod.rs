use crate::auth::cookie_jwt::{cookie_jwt_bearer_auth, cookie_jwt_bearer_resolver};
use crate::RouterState;
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use tower_cookies::CookieManagerLayer;

mod login;
mod logout;
mod protected_content;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
}

pub fn create_cookie_jwt_router(state: RouterState) -> Router {
    Router::new()
        .route(
            "/cookie/page",
            get(protected_content::protected_cookie_content),
        )
        .layer(axum::middleware::from_fn(cookie_jwt_bearer_auth))
        .route("/cookie/logout", post(logout::logout_cookie))
        .route("/cookie/login", post(login::api_login_cookie_jwt))
        .layer(axum::middleware::from_fn(cookie_jwt_bearer_resolver))
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
