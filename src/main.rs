mod config;
mod errors;
mod geolocation;
mod output;
mod weather;

use clap::Parser;
use config::args::Args;
use output::WeatherOutput;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    let geo_info_result = geolocation::get_info_for(&args.city[0]).await;

    let geo_info = match geo_info_result {
        Ok(geo_info) => geo_info,
        Err(e) => {
            let error_display = format!("{}", e);
            // TODO: Match on error type instead of error message
            match error_display.as_str() {
                message if message.contains("Geolocation information not found") => {
                    eprintln!(
                        "Error: Required information not found in the geolocation data \n{}\nPlease provide a valid city name", message
                    );
                }
                _ => {
                    eprintln!("Unexpected error type: {}", e);
                }
            }
            return Ok(());
        }
    };

    let temperature_unit = &args.temperature_unit;
    let windspeed_unit = &args.windspeed_unit;
    let clock_display = &args.display;

    let weather_info_result =
        weather::get_info_for(&geo_info.coordinates, temperature_unit, windspeed_unit).await;

    let weather_info = match weather_info_result {
        Ok(weather_info) => weather_info,
        Err(e) => {
            let error_display = format!("{}", e);
            // TODO: Match on error type instead of error message
            match error_display.as_str() {
                message if message.contains("Weather information not found") => {
                    eprintln!(
                        "Error: Required information not found in the weather data \n{}\nPlease provide valid arguments",
                        message
                    );
                }
                _ => {
                    eprintln!("Unexpected error type: {}", e);
                }
            }
            return Ok(());
        }
    };

    WeatherOutput::print_output(
        &weather_info,
        &geo_info,
        temperature_unit,
        windspeed_unit,
        clock_display,
    );

    Ok(())
}
