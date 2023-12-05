mod config;
mod errors;
mod geolocation;
mod output;
mod weather;

use crate::errors::CustomError;
use clap::Parser;
use config::args::Args;
use output::WeatherOutput;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
     * TODO: Display x day forecast (use arguments)
     * TODO: Add request timout to avoid a blocked terminal when there is no connection
     * TODO: Add possible values to arguments
     */
    let args: Args = Args::parse();

    let geo_info_result = geolocation::get_info_for(&args.city[0]).await;
    let geo_info = match geo_info_result {
        Ok(geo_info) => geo_info,
        Err(e) => {
            match e.downcast_ref::<CustomError>() {
                Some(CustomError::GeolocationNotFound) => {
                    eprintln!("Error: Required information not found in the geolocation data. \
                    Please check the city name you provided. There may also be an issue with the external ressource.");
                }
                _ => {
                    eprintln!("Error retrieving geolocation information: {}", e);
                    eprintln!("Unexpected error type: {:?}", e);
                }
            }
            return Ok(());
        }
    };

    let temperature_unit = &args.temperature_unit;
    let windspeed_unit = &args.windspeed_unit;

    let weather_info_result =
        weather::get_info_for(&geo_info.coordinates, temperature_unit, windspeed_unit).await;
    let weather_info = match weather_info_result {
        Ok(weather_info) => weather_info,
        Err(e) => {
            match e.downcast_ref::<CustomError>() {
                Some(CustomError::WeatherInfoNotFound) => {
                    eprintln!("Error: Required information not found in the weather data. \
                    Please check the units you provided. There may also be an issue with the external ressource.");
                }
                _ => {
                    eprintln!("Error retrieving weather information: {}", e);
                }
            }
            return Ok(());
        }
    };

    // TODO: Use struct/enum for handling all units (temperature_unit & metric/imperial)
    WeatherOutput::print_output(weather_info, &geo_info, temperature_unit, windspeed_unit);

    Ok(())
}
