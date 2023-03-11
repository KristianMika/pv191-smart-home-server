use error_stack::ResultExt;

use crate::{
    sensors::sampler::Sampler,
    server_repo::{postgres_server_repo::PostgresServerRepo, ServerRepo},
};
use std::{error::Error, fmt, sync::Arc};

pub struct ServerState {
    pub repo: Arc<PostgresServerRepo>,
    pub sampler: Arc<Sampler>,
}

impl ServerState {
    pub fn sample_sensors(&self) -> error_stack::Result<(), ServerError> {
        let sample = self
            .sampler
            .perfom_measurement()
            .change_context(ServerError)
            .attach_printable("Couldn't perform measurement")?;
        self.repo
            .store_measurement(sample)
            .change_context(ServerError)
            .attach_printable("Couldn't store measurement")
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
