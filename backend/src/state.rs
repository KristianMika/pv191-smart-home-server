use error_stack::ResultExt;
use tokio::sync::Mutex;

use crate::{
    display_printer::DisplayPrinter,
    measurement,
    sensors::sampler::Sampler,
    server_repo::{postgres_server_repo::PostgresServerRepo, ServerRepo},
};
use std::{error::Error, fmt, sync::Arc};

/// Holds the whole server state
pub struct ServerState {
    /// database repository for storing and fetching measurements
    pub repo: Arc<PostgresServerRepo>,
    /// struct for interacting with all the sensors
    pub sampler: Arc<Sampler>,
    pub display_printer: Arc<Mutex<DisplayPrinter>>,
}

impl ServerState {
    /// Samples all sensors and stores the values into the db
    pub async fn sample_sensors(&self) -> error_stack::Result<(), ServerError> {
        let sample = self
            .sampler
            .perfom_measurement()
            .change_context(ServerError)
            .attach_printable("Couldn't perform measurement")?;
        self.repo
            .store_measurement(sample.clone())
            .change_context(ServerError)
            .attach_printable("Couldn't store measurement")?;
        self.display_printer
            .lock()
            .await
            .print_measurement(sample.into())
            .change_context(ServerError)
            .attach_printable("Coudln't print measurement")?;
        Ok(())
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
