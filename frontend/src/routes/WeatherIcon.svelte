<script lang="ts">
    import Mist from "$lib/icons/mist.svelte";
    import PartlyCloudyDay from "$lib/icons/partly_cloudy_day.svelte";
    import QuestionMark from "$lib/icons/question_mark.svelte";
    import Rainy from "$lib/icons/rainy.svelte";
    import RainyHeavy from "$lib/icons/rainy_heavy.svelte";
    import RainyLight from "$lib/icons/rainy_light.svelte";
    import RainySnow from "$lib/icons/rainy_snow.svelte";
    import Snowing from "$lib/icons/snowing.svelte";
    import SnowingHeavy from "$lib/icons/snowing_heavy.svelte";
    import Sunny from "$lib/icons/sunny.svelte";
    import Thunderstorm from "$lib/icons/thunderstorm.svelte";
    
    let {
        weatherCode,
        size,
        fill,
        class: className
    }: {
        weatherCode: number, 
        size?: string| undefined, 
        fill?: string | undefined,
        class?: string
    } = $props();
    const getWeatherType = (code: number) => {
        if (code === 0) {
            return "clear-sky";
        } else if (code < 10) {
            return "partly-cloudy";
        } else if (code < 50) {
            return "fog";
        } else if (code < 56) {
            return "drizzle";
        } else if (code < 60) {
            return "freezing-drizzle";
        } else if (code < 66) {
            return "rain";
        } else if (code < 70) {
            return "freezing-rain";
        } else if (code < 80) {
            return "snow";
        } else if (code < 85) {
            return "rain-showers";
        } else if (code < 90) {
            return "snow-showers";
        } else if (code < 100) {
            return "thunderstorm";
        }
        return "unknown";
    }
    let currentWeatherType = getWeatherType(weatherCode);
</script>
<svg class="h-fit {className}" xmlns="http://www.w3.org/2000/svg" height={size ?? "24px"} viewBox="0 -960 960 960" width={size ?? "24px"} fill={fill ?? "currentColor"}>
    {#if currentWeatherType === "clear-sky"}
    <Sunny/>
    {:else if currentWeatherType === "partly-cloudy"}
    <PartlyCloudyDay/>
    {:else if currentWeatherType === "fog"}
    <Mist/>
    {:else if currentWeatherType === "drizzle"}
    <RainyLight/>
    {:else if currentWeatherType === "freezing-drizzle"}
    <RainyLight/>
    {:else if currentWeatherType === "rain"}
    <Rainy/>
    {:else if currentWeatherType === "freezing-rain"}
    <RainySnow/>
    {:else if currentWeatherType === "snow"}
    <Snowing/>
    {:else if currentWeatherType === "rain-showers"}
    <RainyHeavy/>
    {:else if currentWeatherType === "snow-showers"}
    <SnowingHeavy/>
    {:else if currentWeatherType === "thunderstorm"}
    <Thunderstorm/>
    {:else}
    <QuestionMark/>
    {/if}
</svg>