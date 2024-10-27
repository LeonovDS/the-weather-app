<script lang="ts">
    import CardContent from "$lib/components/ui/card/card-content.svelte";
    import CardDescription from "$lib/components/ui/card/card-description.svelte";
    import CardHeader from "$lib/components/ui/card/card-header.svelte";
    import CardTitle from "$lib/components/ui/card/card-title.svelte";
    import Card from "$lib/components/ui/card/card.svelte";
    import { locationStore } from "$lib/location.svelte";
	import WeatherItem from "./WeatherItem.svelte";
    
    let {
        class: className
    } : {
        class?: string | undefined
    } = $props();
    $effect(() => {
        locationStore.requestWeather();
    });
</script>

<Card class={className ?? ""}>
    <CardHeader>
        <CardTitle>Current weather</CardTitle>
        <CardDescription>{locationStore.weatherData?.current_weather.time}</CardDescription>
    </CardHeader>
    <CardContent class="flex space-x-4">
        <WeatherItem weatherData={locationStore.weatherData?.current_weather} class="w-full" imageSize="96"/>
    </CardContent>
</Card>