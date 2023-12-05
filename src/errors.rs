use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CustomError {
    WeatherInfoNotFound,
    GeolocationNotFound,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::WeatherInfoNotFound => write!(f, "Weather information not found"),
            CustomError::GeolocationNotFound => write!(f, "Geolocation information not found"),
        }
    }
}

impl Error for CustomError {}
