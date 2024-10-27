<script lang="ts">
    import PoemCard from "./PoemCard.svelte";
    import ForecastCard from "./ForecastCard.svelte";
	import SearchCard from "./SearchCard.svelte";
	import CurrentCard from "./CurrentCard.svelte";
</script>
<div class="w-screen h-screen flex select-none p-8">
    <div class="m-auto grid grid-cols-1 lg:grid-cols-2 lg:w-[1000px] gap-2">
        <div class="lg:col-span-2 flex flex-col items-center mb-8">
            <h3 class="text-3xl">The Weather App</h3>
            <h2 class="text-muted-foreground text-lg" >Check weather around the world!</h2>
        </div>
        <SearchCard class="row-span-2"/>  

        <CurrentCard class="h-fit"/> 
        <PoemCard class="h-fit"/>    
        <ForecastCard class="lg:col-span-2"/> 
    </div>
</div>
