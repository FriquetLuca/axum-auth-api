#[derive(Clone, Debug, thiserror::Error)]
pub enum DBErrors {
    #[error("Failed to find the database: {0}")]
    DbHost(String),
    #[error("Failed to signin to the database: {0}")]
    SignIn(String),
    #[error("Failed to connect to the database: {0}")]
    DbConnection(String),
}
