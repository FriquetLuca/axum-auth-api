use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Clone, Debug)]
pub struct RouterState {
    pub(crate) db: Surreal<Client>,
}
