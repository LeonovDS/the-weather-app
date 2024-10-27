<script lang="ts">
    import CardContent from "$lib/components/ui/card/card-content.svelte";
    import CardFooter from "$lib/components/ui/card/card-footer.svelte";
    import CardHeader from "$lib/components/ui/card/card-header.svelte";
    import CardTitle from "$lib/components/ui/card/card-title.svelte";
    import Card from "$lib/components/ui/card/card.svelte";
	import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
    import { locationStore } from "$lib/location.svelte";
    import {Content_copy} from "svelte-google-materialdesign-icons"
    
    let {
        class: className
    } : {
        class?: string | undefined
    } = $props();
    
    let isToastShown = $state(false);
    const showToast = () => {
        if (isToastShown) {
            return;
        }
        isToastShown = true;
        setTimeout(() => {
            isToastShown = false;
        }, 1000);
    }   
    
    let poemSplitted = $derived(locationStore.weatherData?.poem.split('\n') ?? []);
</script>
<Card class={"flex flex-col " + (className ?? "")}>
    <CardHeader>
        <CardTitle class="flex justify-between items-center">
            Daily mood
            <div class="relative flex items-center">
                <Content_copy size="24" on:click={ (e) =>
                    navigator.clipboard.writeText(locationStore.weatherData?.poem ?? "").then(showToast)
                }/>
                {#if isToastShown}
                <span class="absolute left-full ml-2 text-sm">Скопировано!</span>
                {/if}                
            </div>
        </CardTitle>
    </CardHeader>
    <CardContent>
        <ScrollArea class="text-lg italic select-text">
            {#each poemSplitted as poemLine, i}
            <p class="text-lg italic select-text">{poemLine}</p>
            {/each}
        </ScrollArea>
    </CardContent>
    <div class="flex-1"></div>
    <CardFooter>
        <p class ="text-sm font-thin">Powered by ollama</p>
    </CardFooter>
</Card>