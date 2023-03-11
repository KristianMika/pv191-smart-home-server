use crate::{sensors::sampler::Sampler, server_repo::postgres_server_repo::PostgresServerRepo};
use std::sync::Arc;

pub struct ServerState {
    pub repo: Arc<PostgresServerRepo>,
    pub sampler: Arc<Sampler>,
}
