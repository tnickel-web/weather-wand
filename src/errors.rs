use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CustomError {
    WeatherInfoNotFound(String),
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
