<script lang="ts">
    import CardContent from "$lib/components/ui/card/card-content.svelte";
    import CardHeader from "$lib/components/ui/card/card-header.svelte";
    import CardTitle from "$lib/components/ui/card/card-title.svelte";
    import Card from "$lib/components/ui/card/card.svelte";
    import CarouselContent from "$lib/components/ui/carousel/carousel-content.svelte";
    import CarouselItem from "$lib/components/ui/carousel/carousel-item.svelte";
    import Carousel from "$lib/components/ui/carousel/carousel.svelte";
    import { locationStore } from "$lib/location.svelte";
    import {Wb_sunny} from "svelte-google-materialdesign-icons"
    import WeatherIcon from "./WeatherIcon.svelte";
    import WeatherItem from "./WeatherItem.svelte";
    import CarouselNext from "$lib/components/ui/carousel/carousel-next.svelte";
    import CarouselPrevious from "$lib/components/ui/carousel/carousel-previous.svelte";
    
    let {
        class: className
    } : {
        class?: string | undefined
    } = $props();
    
    
    
</script>
<!-- <Card class={className??""}>
<CardHeader>
<CardTitle>Forecast</CardTitle>
</CardHeader>
<CardContent>
<Carousel class="w-full">
<CarouselContent>
{#if locationStore.weatherData}
{#each locationStore.weatherData.forecast as item}
<CarouselItem class="basis-1/4 flex flex-col items-center space-y-1">
<p>{item.time}</p>
<WeatherIcon weatherCode={item.weather_code} size="48"></WeatherIcon>
<p>{item.temperature}°</p>
<WeatherItem weatherData={item} imageSize="48"/>
</CarouselItem>
{/each}
{/if}
</CarouselContent>
<CarouselPrevious/>
<CarouselNext/>
</Carousel>
</CardContent>
</Card> -->

<div class="w-full {className} overflow-hidden">
    <Carousel class="w-full">
        <CarouselContent class="h-fit -ml-2">
            {#if locationStore.weatherData}
            {#each locationStore.weatherData.forecast as item}
            <CarouselItem class="basis-1/2 lg:basis-1/4 pl-2">
                <Card class="">
                    <CardHeader>
                        <CardTitle>
                            Forecast
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <WeatherItem weatherData={item} imageSize="48"/>
                    </CardContent>
                </Card>
            </CarouselItem>
            {/each}
            {/if}
        </CarouselContent>
        <CarouselPrevious/>
        <CarouselNext/>
    </Carousel>
</div>
