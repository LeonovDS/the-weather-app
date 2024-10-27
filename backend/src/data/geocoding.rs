use crate::model::GeocodingLocation;
use awc::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

const GEOCODING_API_URL: &str = "http://geocoding-api.open-meteo.com/v1/search";

#[derive(Serialize)]
struct GeocodingApiRequest {
    name: String,
    count: Option<i32>,
    format: Option<String>,
    language: Option<String>,
}

#[derive(Deserialize)]
struct GeocodingApiResponse {
    results: Vec<GeocodingApiResultItem>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct GeocodingApiResultItem {
    id: i32,
    name: String,
    latitude: f32,
    longitude: f32,
    elevation: Option<f32>,
    feature_code: Option<String>,
    country_code: String,
    admin1_id: Option<i32>,
    admin2_id: Option<i32>,
    admin3_id: Option<i32>,
    admin4_id: Option<i32>,
    timezone: Option<String>,
    population: Option<i32>,
    postcodes: Option<Vec<String>>,
    country_id: Option<i32>,
    country: Option<String>,
    admin1: Option<String>,
    admin2: Option<String>,
    admin3: Option<String>,
    admin4: Option<String>,
}

impl From<GeocodingApiResultItem> for GeocodingLocation {
    fn from(val: GeocodingApiResultItem) -> Self {
        GeocodingLocation {
            name: val.name,
            latitude: val.latitude,
            longitude: val.longitude,
            country_code: val.country_code,
            timezone: val.timezone,
            country: val.country,
        }
    }
}

const DEFAULT_LOCATION_COUNT: i32 = 10;
const DEFAULT_FORMAT: &str = "json";

pub async fn get_location_from_geocoding_api(
    query: &str,
) -> Result<Vec<GeocodingLocation>, Box<dyn Error>> {
    let client = Client::default();
    let request_params = GeocodingApiRequest {
        name: query.to_owned(),
        count: Some(DEFAULT_LOCATION_COUNT),
        format: Some(DEFAULT_FORMAT.to_owned()),
        language: Some("en".to_owned()),
    };

    let api_response = client
        .get(GEOCODING_API_URL)
        .query(&request_params)?
        .send()
        .await?
        .json::<GeocodingApiResponse>()
        .await?;

    Ok(api_response
        .results
        .into_iter()
        .map(|it| it.into())
        .collect())
}
