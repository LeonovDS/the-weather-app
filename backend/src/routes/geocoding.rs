use crate::data::geocoding::get_location_from_geocoding_api;
use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize)]
#[allow(dead_code)]
struct LocationRequest {
    q: String,
    lang: Option<String>,
}

#[derive(Serialize)]
struct LocationResponseItem {
    location: String,
    latitude: f32,
    longitude: f32,
    country_code: String,
}

#[get("/api/locations")]
pub async fn get_locations(
    params: actix_web::web::Query<LocationRequest>,
) -> Result<impl Responder, Box<dyn Error>> {
    let request: LocationRequest = params.into_inner();
    let data = get_location_from_geocoding_api(&request.q).await?;
    let response_items: Vec<LocationResponseItem> = data
        .iter()
        .map(|it| LocationResponseItem {
            location: it.name.clone(),
            latitude: it.latitude,
            longitude: it.longitude,
            country_code: it.country_code.clone(),
        })
        .collect();
    let response = HttpResponse::Ok().json(response_items);
    Ok(response)
}
