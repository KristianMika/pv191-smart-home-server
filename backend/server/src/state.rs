use common::server_repo::postgres_server_repo::PostgresServerRepo;
use std::{error::Error, fmt, sync::Arc};

/// Holds the whole server state
pub struct ServerState {
    /// database repository for storing and fetching measurements
    pub repo: Arc<PostgresServerRepo>,
}

#[derive(Debug)]
pub struct ServerError;

impl Error for ServerError {}
impl fmt::Display for ServerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Server error ocurred")
    }
}
