use clap::Parser;
use serde::Deserialize;
use chrono::{Local, Timelike};
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    latitude: f32,

    #[arg(long)]
    longitude: f32,

    #[arg(short, long)]
    celsius: bool,
}


#[derive(Deserialize, Debug)]
pub struct Data {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub hourly: HourlyData,
    pub hourly_units: HourlyUnits,
}
#[derive(Deserialize, Debug)]
pub struct HourlyData {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub precipitation_probability: Vec<Option<u8>>, // 0–100, nullable
}
#[derive(Deserialize, Debug)]
pub struct HourlyUnits {
    pub time: String,
    pub temperature_2m: String,
    pub precipitation_probability: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    colored::control::set_override(true);
    let args = Args::parse();
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,precipitation_probability&timezone=auto&forecast_days=1&temperature_unit={}", args.latitude, args.longitude, if args.celsius {"celsius"} else {"fahrenheit"});

    let data = reqwest::get(url).await?.json::<Data>().await?;

    let now = Local::now();

    let mut output = String::new();
    for time in 0..24{
        if time < now.hour(){
            continue;
        }
        if time % 4 == 0 || time == now.hour() || time == now.hour() + 1{
            output.push_str(&format!("{}:00 - temp: {}, rain: {}%   ", 
            time, 
            data.hourly.temperature_2m[time as usize].to_string().green(), 
            data.hourly.precipitation_probability[time as usize].unwrap().to_string().cyan()));
        }
    }

    println!("{}", output);

    Ok(())
}