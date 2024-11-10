use include_dir::{include_dir, Dir};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use surrealdb_migrator::Migrations;

const MIGRATIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub async fn migrate(db: &Surreal<Client>, version: Option<usize>) -> Result<(), String> {
    let migrations = Migrations::from_directory(&MIGRATIONS_DIR)
        .map_err(|e| format!("Error while building from directory: {}", e.to_string()))?;
    match version {
        Some(version) => migrations
            .to_version(&db, version)
            .await
            .map_err(|e| format!("Failed to apply migration(s):{}", e)),
        None => migrations
            .to_latest(&db)
            .await
            .map_err(|e| format!("Failed to apply migration(s):{}", e)),
    }
}
