use common::server_repo::postgres_server_repo::PostgresServerRepo;
use std::{error::Error, fmt, sync::Arc};

/// Holds the whole server state
pub struct ServerState {
    /// database repository for storing and fetching measurements
    repo: Arc<PostgresServerRepo>,
}

impl ServerState {
    pub fn new(repo: Arc<PostgresServerRepo>) -> Self {
        Self { repo }
    }

    pub fn get_repo(&self) -> &Arc<PostgresServerRepo> {
        &self.repo
    }
}

#[derive(Debug)]
pub struct ServerError;

impl Error for ServerError {}
impl fmt::Display for ServerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Server error ocurred")
    }
}
