use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ControllerError;

impl Error for ControllerError {}
impl fmt::Display for ControllerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Controller error ocurred")
    }
}
