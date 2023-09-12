use clap::Parser;

#[derive(Parser)]
#[clap(
    author = "T. Nickel",
    version = "1.0.0",
    about = "A CLI tool to fetch weather from an API and display a beautified output. Supports arguments for city, temperature- and windspeed unit."
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
    pub temperature_unit: String,

    /// The unit used for displaying the windspeed.
    /// | Possible values: "kmh", "ms", "mph", "kn".
    /// | Example: --windspeed-unit mph.
    #[arg(short, long, default_value = "kmh")]
    pub windspeed_unit: String,
}
