use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GeocodingLocation {
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub country_code: String,
    pub timezone: Option<String>,
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherItem {
    pub time: String,
    pub temperature: f32,
    pub weather_code: i32,
    pub humidity: i32,
    pub pressure: f32,
}

#[derive(Serialize, Deserialize)]
pub struct WeaterData {
    pub current_weather: WeatherItem,
    pub forecast: Vec<WeatherItem>,
}
