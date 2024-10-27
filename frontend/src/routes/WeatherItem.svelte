<script lang="ts">
    import type { WeaterData } from "$lib/model";
    import WeatherIcon from "./WeatherIcon.svelte";
    
    let {
        class: className,
        weatherData,
        imageSize
    } : {
        class?: string|undefined,
        weatherData: WeaterData | undefined,
        imageSize?: string
    } = $props();

    const getTime = (dateTime: string) => dateTime.split("T")[1]
</script>
<div class="grid grid-cols-2 gap-2 {className}">
    {#if weatherData}
    <WeatherIcon class="m-auto" size={imageSize} weatherCode={weatherData.weather_code}/>
    <div class="m-auto flex flex-col gap-1">
        <p>{getTime(weatherData.time)}</p>  
        <p class="text-2xl">{weatherData.temperature}°</p>
        <p>{weatherData.humidity}%</p>
    </div>
    {/if}
</div>