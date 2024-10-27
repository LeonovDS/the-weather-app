<script lang="ts">
    import AvatarFallback from "$lib/components/ui/avatar/avatar-fallback.svelte";
    import AvatarImage from "$lib/components/ui/avatar/avatar-image.svelte";
    import Avatar from "$lib/components/ui/avatar/avatar.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import CardContent from "$lib/components/ui/card/card-content.svelte";
    import CardDescription from "$lib/components/ui/card/card-description.svelte";
    import CardFooter from "$lib/components/ui/card/card-footer.svelte";
    import CardHeader from "$lib/components/ui/card/card-header.svelte";
    import CardTitle from "$lib/components/ui/card/card-title.svelte";
    import Card from "$lib/components/ui/card/card.svelte";
    import Input from "$lib/components/ui/input/input.svelte";
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
	import Skeleton from "$lib/components/ui/skeleton/skeleton.svelte";
    import { locationStore } from "$lib/location.svelte";
    
    let {
        class: className,
    }: {
        class?: string | undefined
    } = $props();
    
    const coordToString = (coord: number) =>{
        const absCoord = Math.abs(coord);
        const deg = Math.floor(absCoord);
        const minutes = (absCoord - deg) * 60;
        return `${absCoord.toFixed(0)}° ${minutes.toFixed(0)}'`
    }
</script>

<Card class={className + " flex flex-col"}>
    <CardHeader>
        <CardTitle>Location</CardTitle>
        <CardDescription>Find your city</CardDescription>
    </CardHeader>    
    <CardContent>
        <form class="flex space-x-1 mb-4">
            <Input bind:value={locationStore.searchInput} /> 
            <Button type="submit" onclick={locationStore.search}> Search </Button>
        </form>
        <ScrollArea class="h-[20rem]">
            {#each locationStore.cities as item, i (item.latitude + " " + item.longitude)}
            <label class="flex space-x-2 p-2 hover:bg-slate-900" class:bg-slate-800={item===locationStore.selectedCity}>
                <input type="radio" class="hidden" bind:group={locationStore.selectedCity} value={item}>
                <Avatar class="my-auto">
                    <!-- <AvatarImage src={`https://flagsapi.com/${item.country_code}/flat/64.png`}/> -->
                    <AvatarFallback>{item.country_code}</AvatarFallback>
                </Avatar>
                <div class="space-y-1">
                    <p>{item.location}</p>
                    <p class="text-sm">{coordToString(item.latitude)} {item.latitude >= 0 ? "N" : "S"}, {coordToString(item.longitude)} {item.longitude >= 0 ? "E" : "W"}</p>
                </div>
            </label>
            {:else}
            <div class="">
                Place not found
            </div>
            {/each}
        </ScrollArea>
    </CardContent>
    <CardFooter class="mt-auto">
        <p class="text-sm font-thin">Provided by open-meteo.com</p>
    </CardFooter>
</Card>