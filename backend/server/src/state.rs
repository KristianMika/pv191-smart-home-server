use common::server_repo::postgres_server_repo::PostgresServerRepo;
use std::sync::Arc;

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
