use crate::RouterState;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get_service;
use axum::{routing::get, Json, Router};
use serde::Deserialize;
use tower_http::services::ServeDir;

pub fn create_router(state: RouterState) -> Router {
    let base_get_routes = Router::new()
        .route("/hi", get(index))
        .route("/route/:name", get(hello_html2));

    let session_login_api = Router::new()
        .route("/hello", get(hello_html))
        .merge(crate::api::create_test_router(state.clone()));

    Router::new()
        .merge(base_get_routes)
        .merge(session_login_api)
        .fallback_service(route_static())
}

fn route_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./assets")))
}

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

pub async fn hello_html(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("Doe");
    Html(format!("<p>Hello {name}!</p>"))
}

pub async fn hello_html2(Path(name): Path<String>) -> impl IntoResponse {
    Json(format!("Hello {name}!"))
}

pub async fn index() -> &'static str {
    "Hello, World!"
}
