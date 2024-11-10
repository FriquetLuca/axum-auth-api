use super::super::ResponseBearer;
use super::User;
use crate::auth::bearer_jwt::encode_required_jwt_bearer_claims;
use crate::{ApiResult, BackendError, RouterState};
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct DBUser {
    user_id: Thing,
    username: String,
    password: String,
}

pub async fn api_login_cookie_jwt(
    State(state): State<RouterState>,
    payload: Json<LoginPayload>,
) -> ApiResult<Json<ResponseBearer>> {
    let db = &state.db;

    let mut result = db.query("select id as user_id, username, password from user where username=$username and password=$password")
        .bind(("username", payload.username.clone()))
        .bind(("password", payload.password.clone()))
        .await
        .map_err(|_| BackendError::SomethingWentWrong)?;

    let result: Vec<DBUser> = result
        .take(0)
        .map_err(|_| BackendError::SomethingWentWrong)?;

    let user = result.first().ok_or(BackendError::InvalidCredentials)?;

    if user.username != payload.password.as_str() || user.password != payload.username.as_str() {
        return Err(BackendError::InvalidCredentials);
    }

    let bearer = encode_required_jwt_bearer_claims(User {
        user_id: user.user_id.to_string(),
    })?;

    Ok(Json(ResponseBearer { bearer }))
}
