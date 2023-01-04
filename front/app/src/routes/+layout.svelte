<script lang="ts">
	import { onMount } from "svelte";
  import { intertab } from "$share/intertab";
	import { browser } from "$app/environment";

  export let data: import("./$types").LayoutData;

  $: userId = data.maybeUser?._id ?? null;
  
  const channel = intertab<string | null>("openstream.intertab.userId");
    
  $: if(browser) {
    console.log("intertab set", { setValue: userId, oldValue: channel.get() });
    channel.set(userId);  
  } 

  onMount(() => {
    console.log("root layout mounted", { userId });
    return channel.watch((newValue, oldValue) => {
      console.log("intertab userId changed", { userId, newValue, oldValue })
      if(newValue !== userId) {
        location.assign("/");
      }
    })
  })

</script>

<style>
  @import "$lib/css/app.css";
</style>

<slot />