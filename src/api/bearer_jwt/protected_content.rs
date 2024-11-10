use super::User;
use crate::auth::bearer_jwt::{deserialize_bearer_claims, BearerJWTClaims};
use crate::ApiResult;
use axum::Json;
use serde_json::{json, Value};

pub async fn protected_bearer_content(bearer: BearerJWTClaims) -> ApiResult<Json<Value>> {
    let bearer: User = deserialize_bearer_claims(bearer)?;
    Ok(Json(json!({
        "value": format!("nice secret page here! Oh btw your user id is: `{}`", bearer.user_id),
    })))
}
