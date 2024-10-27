interface LocationData {
    location: string,
    country_code: string,
    latitude: number,
    longitude: number,
};

interface WeatherResponse {
    current_weather: WeaterData,
    forecast: WeaterData[],
    poem: string,
};

interface WeaterData {
    time: string,
    temperature: number,
    weather_code: number,
    humidity: number,
    pressure: number,
};

export type {
    LocationData,
    WeatherResponse,
    WeaterData
};