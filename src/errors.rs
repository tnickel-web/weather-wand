use std::error::Error;
use std::fmt;

/// Represents custom errors that may occur during weather and geolocation operations.
#[derive(Debug, Clone)]
pub enum CustomError {
    /// Indicates that the weather information could not be parsed correctly.
    WeatherInfoNotFound(String),

    /// Indicates that the geolocation information could not be parsed correctly.
    GeolocationNotFound(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::WeatherInfoNotFound(info) => {
                write!(f, "Weather information not found - {}", info)
            }
            CustomError::GeolocationNotFound(info) => {
                write!(f, "Geolocation information not found - {}", info)
            }
        }
    }
}

impl Error for CustomError {}
