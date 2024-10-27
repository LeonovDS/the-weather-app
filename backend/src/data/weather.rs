use awc::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::model::{WeaterData, WeatherItem};

const WEATHER_API_URL: &str = "http://api.open-meteo.com/v1/forecast";

#[derive(Serialize)]
struct WeatherApiParameters {
    latitude: f32,
    longitude: f32,
    current: String,
    hourly: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct WeatherApiResponse {
    latitude: f32,
    longitude: f32,
    elevation: f32,
    generationtime_ms: f32,
    utc_offset_seconds: i32,
    timezone: String,
    timezone_abbreviation: String,
    hourly: WeatherApiHourlyData,
    current: WeatherApiCurrentData,
}

#[derive(Deserialize)]
struct WeatherApiCurrentData {
    time: String,
    temperature_2m: f32,
    relative_humidity_2m: i32,
    weather_code: i32,
    surface_pressure: f32,
}

impl From<WeatherApiCurrentData> for WeatherItem {
    fn from(value: WeatherApiCurrentData) -> Self {
        Self {
            time: value.time,
            temperature: value.temperature_2m,
            weather_code: value.weather_code,
            humidity: value.relative_humidity_2m,
            pressure: value.surface_pressure,
        }
    }
}

#[derive(Deserialize)]
struct WeatherApiHourlyData {
    time: Vec<String>,
    temperature_2m: Vec<f32>,
    relative_humidity_2m: Vec<i32>,
    weather_code: Vec<i32>,
    surface_pressure: Vec<f32>,
}

pub async fn get_weather_from_api(
    latitude: f32,
    longitude: f32,
) -> Result<WeaterData, Box<dyn Error>> {
    let client = Client::default();
    let query_params = WeatherApiParameters {
        latitude,
        longitude,
        current: "temperature_2m,relative_humidity_2m,weather_code,surface_pressure".to_owned(),
        hourly: "temperature_2m,relative_humidity_2m,weather_code,surface_pressure".to_owned(),
    };

    let weather_api_response = client
        .get(WEATHER_API_URL)
        .query(&query_params)?
        .send()
        .await?
        .json::<WeatherApiResponse>()
        .await?;

    Ok(WeaterData {
        current_weather: weather_api_response.current.into(),
        forecast: (1..weather_api_response.hourly.time.len())
            .map(|i| WeatherItem {
                time: weather_api_response.hourly.time[i].clone(),
                temperature: weather_api_response.hourly.temperature_2m[i],
                humidity: weather_api_response.hourly.relative_humidity_2m[i],
                pressure: weather_api_response.hourly.surface_pressure[i],
                weather_code: weather_api_response.hourly.weather_code[i],
            })
            .collect(),
    })
}
