use crate::data::ollama::get_ollama_poem;
use crate::data::weather::get_weather_from_api;
use crate::model::WeatherItem;
use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize)]
struct WeatherRequest {
    latitude: f32,
    longitude: f32,
}

#[derive(Serialize)]
struct WeatherResponse {
    current_weather: WeatherItem,
    forecast: Vec<WeatherItem>,
    poem: String,
}

#[get("/api/weather")]
pub async fn get_weather(
    params: actix_web::web::Query<WeatherRequest>,
) -> Result<impl Responder, Box<dyn Error>> {
    let request: WeatherRequest = params.into_inner();
    let weather = get_weather_from_api(request.latitude, request.longitude).await?;

    let poem: String = get_ollama_poem(&weather.current_weather).await?;

    let response = HttpResponse::Ok().json(WeatherResponse {
        current_weather: weather.current_weather,
        forecast: weather.forecast,
        poem,
    });
    Ok(response)
}
