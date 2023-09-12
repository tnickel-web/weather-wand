mod config;
mod geolocation;
mod output;
mod weather;

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

    let geo_info = geolocation::get_info_for(&args.city[0]).await?;

    let temperature_unit = &args.temperature_unit;
    let windspeed_unit = &args.windspeed_unit;

    let weather_info =
        weather::get_info_for(&geo_info.coordinates, temperature_unit, windspeed_unit).await?;

    // TODO: Use struct/enum for handling all units (temperature_unit & metric/imperial)
    WeatherOutput::print_output(weather_info, &geo_info, temperature_unit, windspeed_unit);

    Ok(())
}
