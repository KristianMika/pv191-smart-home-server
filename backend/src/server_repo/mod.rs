use crate::models::{MeasurementStore, NewMeasurementStore};
use std::{error::Error, fmt};

pub(crate) mod postgres_server_repo;

/// Repository trait for database backends
pub trait ServerRepo {
    /// Stores the latest measurement
    fn store_measurement(
        &self,
        measurement: NewMeasurementStore,
    ) -> error_stack::Result<(), DbError>;
    /// Fatches the latest measurement
    fn get_last_measurement(&self) -> error_stack::Result<Option<MeasurementStore>, DbError>;
}

#[derive(Debug)]
pub struct DbError;

impl Error for DbError {}
impl fmt::Display for DbError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("could not interract with the db")
    }
}
