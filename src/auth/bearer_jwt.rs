use crate::{env_config, ApiResult, BackendError};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An extractor for Bearer token.
pub struct BearerJWTClaims {
    pub data: String,
    pub exp: usize,
    pub iat: usize,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for BearerJWTClaims {
    type Rejection = BackendError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| BackendError::InvalidToken)?;
        let token = decode::<BearerJWTClaims>(
            bearer.token(),
            &&env_config().jwt_decode,
            &Validation::default(),
        )
        .map_err(|_| BackendError::InvalidToken)?;
        let now = Utc::now();
        if token.claims.exp < now.timestamp() as usize {
            Err(BackendError::InvalidToken)
        } else {
            Ok(token.claims)
        }
    }
}

/// Encode a required JWT Bearer token.
pub fn encode_required_jwt_bearer_claims<T: Serialize>(data: T) -> ApiResult<String> {
    let now = Utc::now();
    let expire = Duration::hours(24);
    let claim = BearerJWTClaims {
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
        data: serde_json::to_string(&data).map_err(|_| BackendError::SomethingWentWrong)?,
    };
    encode(&Header::default(), &claim, &env_config().jwt_encode)
        .map_err(|_| BackendError::JWTEncodingFailed)
}

/// Deserialize the content of the bearer's string value.
pub fn deserialize_bearer_claims<T: DeserializeOwned>(claims: BearerJWTClaims) -> ApiResult<T> {
    serde_json::from_str(&claims.data).map_err(|_| BackendError::SomethingWentWrong)
}
