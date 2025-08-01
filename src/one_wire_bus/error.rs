use core::fmt::Debug;
use std::error::Error;

pub type OneWireResult<T> = Result<T, OneWireError>;

#[derive(Debug)]
pub enum OneWireError {
    /// The Bus was expected to be pulled high by a ~5K ohm pull-up resistor, but it wasn't
    BusNotHigh,

    PinError(Box<dyn Error>),

    /// An unexpected response was received from a command. This generally happens when a new sensor is added
    /// or removed from the bus during a command, such as a device search.
    UnexpectedResponse,

    FamilyCodeMismatch,
    CrcMismatch,
    Timeout,
}