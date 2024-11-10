use crate::{env_config, ApiResult, BackendError};
use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, Request};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use tower_cookies::cookie::time::OffsetDateTime;
use tower_cookies::{cookie, Cookie, Cookies};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Contain the data for a user cookie claims.
pub struct CookieJWTClaims {
    pub data: String,
    pub exp: usize,
    pub iat: usize,
}

/// Deserialize the content of the cookie's string value.
pub fn deserialize_cookie_claims<T: DeserializeOwned>(claims: CookieJWTClaims) -> ApiResult<T> {
    serde_json::from_str(&claims.data).map_err(|_| BackendError::SomethingWentWrong)
}

/// Apply the use of a `CookieJWTClaims` to a layer.
pub async fn cookie_jwt_bearer_auth(
    ctx: ApiResult<CookieJWTClaims>,
    req: Request<Body>,
    next: Next,
) -> ApiResult<Response> {
    ctx?;
    Ok(next.run(req).await)
}

/// Resolve the jwt layer to validate the jwt cookie token.
pub async fn cookie_jwt_bearer_resolver(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> ApiResult<Response> {
    let auth_token = cookies
        .get(&env_config().jwt_cookie_name)
        .map(|c| c.value().to_string());
    let compute_auth: ApiResult<CookieJWTClaims> = if let Some(token) = auth_token {
        let token = decode::<CookieJWTClaims>(
            token.as_str(),
            &env_config().jwt_decode,
            &Validation::default(),
        )
        .map_err(|_| BackendError::InvalidToken)?;
        let now = Utc::now();
        if token.claims.exp < now.timestamp() as usize {
            Err(BackendError::InvalidToken)
        } else {
            Ok(token.claims)
        }
    } else {
        Err(BackendError::NoCookieFound)
    };
    req.extensions_mut().insert(compute_auth);
    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for CookieJWTClaims {
    type Rejection = BackendError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> ApiResult<Self> {
        parts
            .extensions
            .get::<ApiResult<CookieJWTClaims>>()
            .ok_or(BackendError::NoCookieFound)?
            .clone()
    }
}

/// Encode a required data in the cookie jwt claims then returns the content of the jwt bearer.
pub fn encode_cookie_jwt_bearer_claims<T: Serialize>(
    cookies: Cookies,
    data: T,
) -> ApiResult<String> {
    let now = Utc::now();
    let now_cookie = OffsetDateTime::now_utc();
    let expire = Duration::hours(24);
    let claim = CookieJWTClaims {
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
        data: serde_json::to_string(&data).map_err(|_| BackendError::SomethingWentWrong)?,
    };
    let encoded_jwt = encode(&Header::default(), &claim, &env_config().jwt_encode)
        .map_err(|_| BackendError::JWTEncodingFailed)?;

    let mut cookie = Cookie::new(&env_config().jwt_cookie_name, encoded_jwt.clone());
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.set_expires(now_cookie.add(cookie::time::Duration::hours(24)));
    cookies.add(cookie);

    Ok(encoded_jwt)
}

/// Remove a cookie from a jwt bearer
pub fn remove_cookie_jwt_bearer_claims(cookies: Cookies) {
    let now_cookie = OffsetDateTime::now_utc();
    let mut cookie = Cookie::new(&env_config().jwt_cookie_name, "");
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.set_expires(now_cookie);
    cookies.add(cookie);
}
