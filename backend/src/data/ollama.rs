use crate::model::WeatherItem;
use awc::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

const OLLAMA_URL: &str = "http://localhost:11434/api/generate";
const MODEL_NAME: &str = "llama3.2";

#[derive(Serialize, Debug)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

fn create_prompt(current_weather: &WeatherItem) -> String {
    format!(
        "Today is {}. Temperature is {} Celcius. Write cinquain about current weather",
        current_weather.time, current_weather.temperature
    )
}

pub async fn get_ollama_poem(current_weather: &WeatherItem) -> Result<String, Box<dyn Error>> {
    let client = Client::default();
    let request = OllamaRequest {
        model: MODEL_NAME.into(),
        prompt: create_prompt(current_weather),
        stream: false,
    };
    Ok(client
        .post(OLLAMA_URL)
        .send_json(&request)
        .await?
        .json::<OllamaResponse>()
        .await?
        .response)
}
