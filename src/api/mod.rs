mod bearer_jwt;
mod cookies_jwt;

use crate::RouterState;
use axum::Router;
use bearer_jwt::create_bearer_jwt_router;
use cookies_jwt::create_cookie_jwt_router;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBearer {
    pub bearer: String,
}

pub fn create_test_router(state: RouterState) -> Router {
    Router::new()
        .merge(create_cookie_jwt_router(state.clone()))
        .merge(create_bearer_jwt_router(state.clone()))
}

#[cfg(test)]
mod tests {
    use crate::api::ResponseBearer;
    use axum::http::StatusCode;
    use serde_json::json;

    #[tokio::test]
    async fn login_with_jwt_cookie() -> anyhow::Result<()> {
        let hc = httpc_test::new_client("http://localhost:3000/api")?;

        let me_get_unauthorized = hc.do_get("/cookie/page").await?;
        assert_eq!(
            me_get_unauthorized.status(),
            StatusCode::UNAUTHORIZED,
            "Shouldn't be authorized to access the resource"
        );

        let login_post = hc
            .do_post(
                "/cookie/login",
                json!({
                    "username": "root",
                    "password": "root"
                }),
            )
            .await?;
        assert_eq!(login_post.status(), StatusCode::OK, "Should be logged in");

        let cookie = login_post.client_cookie("session");
        assert!(cookie.is_some(), "Should have a cookie");

        let login_lost_value = login_post.json_body_as::<ResponseBearer>();
        assert!(login_lost_value.is_ok(), "Should contain a response body");

        if let Some(cookie) = cookie {
            if let Ok(bearer) = login_lost_value {
                assert_eq!(bearer.bearer, cookie.value, "Should contain a response body with the same value as the one present in the cookie");
            }
        }

        let me_get_unauthorized = hc.do_get("/cookie/page").await?;
        assert_eq!(
            me_get_unauthorized.status(),
            StatusCode::OK,
            "Should be authorized to access the resource"
        );

        let logout_post = hc.do_post("/cookie/logout", json!({})).await?;
        assert_eq!(
            logout_post.status(),
            StatusCode::OK,
            "Should be a positive response status code"
        );

        let me_get_unauthorized = hc.do_get("/cookie/page").await?;
        assert_eq!(
            me_get_unauthorized.status(),
            StatusCode::UNAUTHORIZED,
            "Shouldn't be authorized to access the resource"
        );

        Ok(())
    }

    #[tokio::test]
    async fn failed_login_with_jwt_cookie() -> anyhow::Result<()> {
        let hc = httpc_test::new_client("http://localhost:3000/api")?;

        let me_get_unauthorized = hc.do_get("/cookie/page").await?;
        assert_eq!(
            me_get_unauthorized.status(),
            StatusCode::UNAUTHORIZED,
            "Shouldn't be authorized to access the resource"
        );

        let login_post = hc
            .do_post(
                "/cookie/login",
                json!({
                    "username": "reut",
                    "password": "rotten"
                }),
            )
            .await?;
        assert_eq!(
            login_post.status(),
            StatusCode::UNAUTHORIZED,
            "Shouldn' be logged in"
        );

        let me_get_unauthorized = hc.do_get("/cookie/page").await?;
        assert_eq!(
            me_get_unauthorized.status(),
            StatusCode::UNAUTHORIZED,
            "Shouldn't be authorized to access the resource"
        );

        Ok(())
    }

    #[tokio::test]
    async fn login_with_jwt() -> anyhow::Result<()> {
        let hc = httpc_test::new_client("http://localhost:3000/api")?;
        let unauthorized_page = hc.do_get("/bearer/page").await?;
        assert_eq!(
            unauthorized_page.status(),
            StatusCode::UNAUTHORIZED,
            "Shouldn't be authorized to access the resource"
        );

        let bearer = hc
            .do_post(
                "/bearer/login",
                json!({
                    "username": "root",
                    "password": "root"
                }),
            )
            .await?;
        let client = hc.reqwest_client();
        let bearer = bearer.json_body_as::<ResponseBearer>();
        match bearer {
            Ok(bearer) => {
                let result = client
                    .request(
                        reqwest::Method::GET,
                        "http://localhost:3000/api/bearer/page",
                    )
                    .bearer_auth(bearer.bearer)
                    .send()
                    .await?;
                assert_eq!(result.status(), StatusCode::OK, "The status should be OK");
            }
            Err(_) => assert!(false, "Bearer token should exist in the response"),
        };

        Ok(())
    }

    #[tokio::test]
    async fn failed_login_with_jwt() -> anyhow::Result<()> {
        let hc = httpc_test::new_client("http://localhost:3000/api")?;
        let unauthorized_page = hc.do_get("/bearer/page").await?;
        assert_eq!(
            unauthorized_page.status(),
            StatusCode::UNAUTHORIZED,
            "Shouldn't be authorized to access the resource"
        );

        let bearer = hc
            .do_post(
                "/bearer/login",
                json!({
                    "username": "reuut",
                    "password": "rawt"
                }),
            )
            .await?;
        assert_eq!(
            bearer.status(),
            StatusCode::UNAUTHORIZED,
            "Shouldn' be logged in"
        );

        Ok(())
    }
}
