mod api;
mod auth;
mod config;
mod error;
mod router;
mod state;
mod surreal;

pub use error::*;
use std::sync::OnceLock;

use axum::Router;
use config::{load_config, Config};
pub use state::RouterState;
use surreal::*;

pub(crate) fn env_config() -> &'static Config {
    static ENV_CONFIG: OnceLock<Config> = OnceLock::new();
    ENV_CONFIG.get_or_init(|| match load_config() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    })
}

fn main() {
    tracing_subscriber::fmt().init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let db = match connect_db(
                env_config().db_host.as_str(),
                env_config().db_user.as_str(),
                env_config().db_pswd.as_str(),
                env_config().db_namespace.as_str(),
                env_config().db_database.as_str(),
            )
            .await
            {
                Ok(db) => db,
                Err(err) => panic!("{:#?}", err),
            };
            tracing::info!("DB connexion created");

            if let Err(err) = migrate(&db, env_config().db_version).await {
                panic!("{}", err.as_str());
            }

            let state = RouterState { db };

            let app = Router::new().nest("/api", router::create_router(state));
            tracing::info!("API router created");

            let host = env_config()
                .host_port
                .map(|port| format!("{}:{port}", env_config().host_name))
                .unwrap_or(env_config().host_name.to_string());
            let listener = tokio::net::TcpListener::bind(host.as_str()).await.unwrap();
            tracing::info!("Start server");
            axum::serve(listener, app).await.unwrap();
        });
}
