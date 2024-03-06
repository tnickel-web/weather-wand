use clap::Parser;
use std::fmt::Display;

#[derive(Parser)]
#[clap(
    version = "1.2.0",
    about = "A CLI tool to fetch weather from an API and display a beautified output. Supports arguments for city, temperature- and wind speed unit."
)]
pub struct Args {
    /// The city you want to see the current weather for.
    /// | Example: --city "New York"
    #[arg(short, long, required = true)]
    pub city: Vec<String>,

    /// The unit used for displaying the temperature.
    /// | Possible values: "celsius", "fahrenheit".
    /// | Example: --temperature-unit fahrenheit.
    #[arg(short, long, default_value = "celsius")]
    pub temperature_unit: TemperatureUnit,

    /// The unit used for displaying the windspeed.
    /// | Possible values: "kmh", "ms", "mph", "kn".
    /// | Example: --windspeed-unit mph.
    #[arg(short, long, default_value = "kmh")]
    pub windspeed_unit: WindspeedUnit,

    /// The clock's display mode.
    /// | Possible values: "12h", "24h"
    /// | Example: --display 12h
    #[arg(short, long, default_value = "24h")]
    pub display: ClockDisplay,
}

#[derive(Clone, clap::ValueEnum)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TemperatureUnit::Celsius => "celsius".to_string(),
            TemperatureUnit::Fahrenheit => "fahrenheit".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Clone, clap::ValueEnum)]
pub enum WindspeedUnit {
    Kmh,
    Ms,
    Mph,
    Kn,
}

impl Display for WindspeedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            WindspeedUnit::Kmh => "kmh".to_string(),
            WindspeedUnit::Ms => "ms".to_string(),
            WindspeedUnit::Mph => "mph".to_string(),
            WindspeedUnit::Kn => "kn".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Clone, clap::ValueEnum)]
pub enum ClockDisplay {
    _12h,
    _24h,
}

impl Display for ClockDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ClockDisplay::_12h => "12h".to_string(),
            ClockDisplay::_24h => "24h".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_unit_fmt_formats_correct_strings() {
        assert_eq!(format!("{}", TemperatureUnit::Celsius), "celsius");
        assert_eq!(format!("{}", TemperatureUnit::Fahrenheit), "fahrenheit");
    }

    #[test]
    fn test_windspeed_unit_fmt_formats_correct_strings() {
        assert_eq!(format!("{}", WindspeedUnit::Kmh), "kmh");
        assert_eq!(format!("{}", WindspeedUnit::Ms), "ms");
        assert_eq!(format!("{}", WindspeedUnit::Mph), "mph");
        assert_eq!(format!("{}", WindspeedUnit::Kn), "kn");
    }

    #[test]
    fn clock_display_fmt_formats_correct_strings() {
        assert_eq!(format!("{}", ClockDisplay::_12h), "12h");
        assert_eq!(format!("{}", ClockDisplay::_24h), "24h");
    }
}
