use hidapi::HidError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ScaleError {
    UsbError(HidError),
    ConnectError(HidError),
    ReadError,
    ParseError,
}

pub type ScaleResult<T> = Result<T, ScaleError>;

impl fmt::Display for ScaleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            ScaleError::UsbError(ctx) => format!("Could not open HID connection! {}", ctx),
            ScaleError::ConnectError(ctx) => format!("No scale connected! {}", ctx),
            ScaleError::ReadError => format!("Failed to read scale data!"),
            ScaleError::ParseError => format!("Error parsing data from scale!"),
        };

        write!(f, "{}", message)
    }
}

impl Error for ScaleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::UsbError(hid_error) => Some(hid_error),
            Self::ConnectError(hid_error) => Some(hid_error),
            Self::ReadError => None,
            Self::ParseError => None,
        }
    }
}
