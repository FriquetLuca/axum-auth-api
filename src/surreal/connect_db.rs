use super::errors::DBErrors;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn connect_db(
    db_host: impl Into<String>,
    db_user: impl Into<String>,
    db_pswd: impl Into<String>,
    db_namespace: impl Into<String>,
    db_database: impl Into<String>,
) -> Result<Surreal<Client>, DBErrors> {
    let db_host = db_host.into();
    let db_user = db_user.into();
    let db_pswd = db_pswd.into();
    let db_namespace = db_namespace.into();
    let db_database = db_database.into();

    let db = Surreal::new::<Ws>(db_host.as_str())
        .await
        .map_err(|e| DBErrors::DbHost(e.to_string()))?;

    db.signin(Root {
        username: db_user.as_str(),
        password: db_pswd.as_str(),
    })
    .await
    .map_err(|e| DBErrors::SignIn(e.to_string()))?;
    db.use_ns(db_namespace)
        .use_db(db_database)
        .await
        .map_err(|e| DBErrors::DbConnection(e.to_string()))?;
    Ok(db)
}
