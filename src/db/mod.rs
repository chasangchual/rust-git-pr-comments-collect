pub mod connection;
pub mod models;
pub mod schema;

use std::sync::Arc;
use connection::PgPooledConnection;

pub struct Context {
    pub conn: Arc<PgPooledConnection>
}

pub fn create_context(pg_pool: PgPooledConnection) -> Context {
    Context {
        conn: Arc::new(pg_pool),
    }
}