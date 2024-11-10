use crate::auth::cookie_jwt::remove_cookie_jwt_bearer_claims;
use crate::ApiResult;
use axum::Json;
use serde_json::{json, Value};
use tower_cookies::Cookies;

pub async fn logout_cookie(cookies: Cookies, _: Json<Value>) -> ApiResult<Json<Value>> {
    remove_cookie_jwt_bearer_claims(cookies);
    Ok(Json(json!({
        "value": "You're now disconnected",
    })))
}
