import type { LocationData, WeatherResponse } from "./model";

class LocationStore {
    searchInput = $state("");
    cities = $state.raw<LocationData[]>([]);
    selectedCity = $state.raw<LocationData | undefined>(undefined);
    weatherData = $state.raw<WeatherResponse | undefined>(undefined);
    
    public requestWeather = async () => {
        if (this.selectedCity) {
            const response = await fetch(`http://localhost:8080/api/weather?latitude=${this.selectedCity.latitude}&longitude=${this.selectedCity.longitude}`);
            const data = await response.json();
            this.weatherData = data as WeatherResponse;
            console.log(data);
        }
    };
    
    public search = async () => {
        if (this.searchInput === undefined || this.searchInput.trim().length === 0) {
            return; 
        }
        const response = await fetch(`http://localhost:8080/api/locations?q=${this.searchInput}`);
        const data = await response.json();
        console.log(data);
        this.cities = data as LocationData[];
        if (this.cities.length > 0) {
            this.selectedCity = this.cities[0];
        } else {
            this.selectedCity = undefined;
        }
    };
}

export const locationStore = new LocationStore();
