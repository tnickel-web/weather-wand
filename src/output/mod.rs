use super::geolocation::Location;
use super::weather::CurrentWeather;
use chrono::{DateTime, Utc};
use colored::Colorize;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct WeatherOutput {}

impl WeatherOutput {
    pub fn print_output(
        weather: CurrentWeather,
        geo_info: &Location,
        temperature_unit: &str,
        windspeed_unit: &str,
        clock_display: &str,
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
            if temperature_unit == "fahrenheit" {
                "°F"
            } else {
                "°C"
            }
        );

        let windspeed_formatted = format!(
            "{} {}",
            weather.windspeed,
            match windspeed_unit {
                "ms" => "m/s",
                "mph" => "Mph",
                "kn" => "Knots",
                _ => "Km/h",
            }
        );

        let day_night_icon = if weather.is_day == "1" {
            "   "
        } else {
            "   "
        };

        let day_night_status = if weather.is_day == "1" {
            " Day"
        } else {
            " Night"
        };

        let day_night_formatted = format!(
            "{} Day/Night:  {}",
            day_night_icon,
            day_night_status.bright_blue()
        );

        let formatted_time = format_time(weather.timestamp, clock_display);

        println!("┌{}┐", decoration);
        println!("  {}", header.cyan().bold(),);
        println!("  󱣖  Temperature: {}", temperature_formatted.bright_blue());
        println!("    Wind Speed:  {}", windspeed_formatted.bright_blue());
        println!(
            "    Location:    {}",
            geo_info.name.trim_matches('"').bright_blue()
        );
        println!(
            "    Coordinates: {}, {}",
            geo_info.coordinates.latitude.bright_blue(),
            geo_info.coordinates.longitude.bright_blue()
        );
        println!("{}", day_night_formatted);
        println!("    Time:        {}", formatted_time.bright_blue());
        println!(
            "    Timezone:    {}",
            weather
                .timezone
                .trim_matches('"')
                .replace('_', " ")
                .bright_blue()
        );
        println!("└{}┘", decoration);
    }
}

fn format_time(timestamp: u64, clock_display: &str) -> String {
    let converted_time: SystemTime = UNIX_EPOCH + std::time::Duration::from_secs(timestamp);

    let format = if clock_display == "12h" {
        "%Y-%m-%d %I:%M %p"
    } else {
        "%Y-%m-%d %H:%M"
    };

    let formatted_time = format!("{}", DateTime::<Utc>::from(converted_time).format(format));

    formatted_time
}
