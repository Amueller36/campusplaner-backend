use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum MensaPlanError {
    RequestError(reqwest::Error),
    ParseError(serde_xml_rs::Error),
    ConversionError(Box<dyn Error>),
    InvalidLocation(String),
}

impl fmt::Display for MensaPlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MensaPlanError::RequestError(ref err) => write!(f, "Request error: {}", err),
            MensaPlanError::ParseError(ref err) => write!(f, "Parse error: {}", err),
            MensaPlanError::ConversionError(ref err) => write!(f, "Conversion error: {}", err),
            MensaPlanError::InvalidLocation(ref loc) => write!(f, "Invalid location: {}", loc),
        }
    }
}

impl From<reqwest::Error> for MensaPlanError {
    fn from(err: reqwest::Error) -> MensaPlanError {
        MensaPlanError::RequestError(err)
    }
}

impl From<serde_xml_rs::Error> for MensaPlanError {
    fn from(err: serde_xml_rs::Error) -> MensaPlanError {
        MensaPlanError::ParseError(err)
    }
}

impl From<Box<dyn Error>> for MensaPlanError {
    fn from(err: Box<dyn Error>) -> MensaPlanError {
        MensaPlanError::ConversionError(err)
    }
}