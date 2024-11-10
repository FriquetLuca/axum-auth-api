use super::User;
use crate::auth::cookie_jwt::{deserialize_cookie_claims, CookieJWTClaims};
use crate::ApiResult;
use axum::Json;
use serde_json::{json, Value};

pub async fn protected_cookie_content(session: CookieJWTClaims) -> ApiResult<Json<Value>> {
    let claims: User = deserialize_cookie_claims(session)?;
    Ok(Json(json!({
        "value": format!("nice secret page here! Oh btw your user id is: `{}`", claims.user_id),
    })))
}
