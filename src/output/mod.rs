use super::geolocation::Location;
use super::weather::CurrentWeather;
use crate::config::args::{ClockDisplay, TemperatureUnit, WindspeedUnit};
use chrono::{DateTime, Local, Utc};
use colored::Colorize;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct WeatherOutput {}

impl WeatherOutput {
    /// Prints the current weather report to the console.
    ///
    /// # Arguments
    ///
    /// * `weather`: A reference to the `CurrentWeather` struct containing weather information.
    /// * `geo_info`: A reference to the `Location` struct containing geolocation information.
    /// * `temperature_unit`: The unit for temperature display (e.g. Celsius or Fahrenheit).
    /// * `windspeed_unit`: The unit for windspeed display (e.g. m/s or mph).
    /// * `clock_display`: The clock display format (12-hour or 24-hour).
    pub fn print_output(
        weather: &CurrentWeather,
        geo_info: &Location,
        temperature_unit: &TemperatureUnit,
        windspeed_unit: &WindspeedUnit,
        clock_display: &ClockDisplay,
    ) {
        let header = format!(
            "{} for {}, {}",
            "  Current Weather Report",
            geo_info.name.trim_matches('"'),
            geo_info.country_code.trim_matches('"')
        );

        let decoration_line = "─";
        let decoration = decoration_line.repeat(header.len());

        let temperature_formatted = format!(
            "{} {}",
            weather.temperature,
            match temperature_unit {
                TemperatureUnit::Fahrenheit => "°F",
                TemperatureUnit::Celsius => "°C",
            }
        );

        let windspeed_formatted = format!(
            "{} {}",
            weather.windspeed,
            match windspeed_unit {
                WindspeedUnit::Ms => "m/s",
                WindspeedUnit::Mph => "Mph",
                WindspeedUnit::Kn => "Knots",
                WindspeedUnit::Kmh => "Km/h",
            }
        );

        let day_night_icon = if weather.is_day == "1" {
            " "
        } else {
            " "
        };

        let formatted_date_local = format_date(weather.timestamp, clock_display).local;

        println!("┌{}┐", decoration);
        println!("  {}", header.cyan().bold(),);
        println!("  󱣖  Temperature: {}", temperature_formatted.bright_blue());
        println!("    Wind Speed:  {}", windspeed_formatted.bright_blue());
        println!(
            "  󰅆  City:        {}",
            geo_info.name.trim_matches('"').bright_blue()
        );

        let area = format!("{}, {}", geo_info.region, geo_info.country).bright_blue();
        println!("    Area:        {}", area);

        let coordinates = format!(
            "{}, {}",
            geo_info.coordinates.latitude, geo_info.coordinates.longitude
        )
        .bright_blue();
        println!("    Coordinates: {}", coordinates);

        println!(
            "    Time:        {} {} {}",
            formatted_date_local.bright_blue(),
            "|".bright_blue(),
            day_night_icon.bright_blue()
        );
        println!(
            "    Timezone:    {}",
            geo_info
                .timezone
                .trim_matches('"')
                .replace('_', " ")
                .bright_blue()
        );
        println!("└{}┘", decoration);
    }
}

struct FormattedDates {
    #[allow(dead_code)]
    utc: String,
    local: String,
}

fn format_date(timestamp: u64, clock_display: &ClockDisplay) -> FormattedDates {
    let converted_date: SystemTime = UNIX_EPOCH + std::time::Duration::from_secs(timestamp);

    let format = match clock_display {
        ClockDisplay::_12h => "%b %-e, %Y %I:%M %p",
        ClockDisplay::_24h => "%b %-e, %Y %H:%M",
    };

    let formatted_date_utc = format!("{}", DateTime::<Utc>::from(converted_date).format(format));
    let formatted_date_local =
        format!("{}", DateTime::<Local>::from(converted_date).format(format));

    FormattedDates {
        utc: formatted_date_utc,
        local: formatted_date_local,
    }
}

#[cfg(test)]
mod tests {
    use super::format_date;
    use crate::config::args::ClockDisplay;
    use chrono::{Local, TimeZone, Utc};

    #[test]
    fn format_date_returns_correctly_formatted_dates() {
        let timestamp = 1672531200;
        assert_eq!(
            "Jan 1, 2023 12:00 AM",
            format_date(timestamp, &ClockDisplay::_12h).utc
        );
        assert_eq!(
            "Jan 1, 2023 00:00",
            format_date(timestamp, &ClockDisplay::_24h).utc
        );

        // Convert UTC to local
        let local_time_12h_expected = Utc
            .timestamp_opt(timestamp as i64, 0)
            .earliest()
            .unwrap()
            .with_timezone(&Local)
            .format("%b %-e, %Y %I:%M %p")
            .to_string();
        assert_eq!(
            local_time_12h_expected,
            format_date(timestamp, &ClockDisplay::_12h).local
        );

        // Convert UTC to local
        let local_time_24h_expected = Utc
            .timestamp_opt(timestamp as i64, 0)
            .earliest()
            .unwrap()
            .with_timezone(&Local)
            .format("%b %-e, %Y %H:%M")
            .to_string();
        assert_eq!(
            local_time_24h_expected,
            format_date(timestamp, &ClockDisplay::_24h).local
        );
    }
}
