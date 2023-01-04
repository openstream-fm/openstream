<script lang="ts">
	import { onMount } from "svelte";
  import { intertab } from "$share/intertab";
	import { browser } from "$app/environment";

  export let data: import("./$types").LayoutData;

  $: user_id = data.maybe_user?._id ?? null;
  
  const channel = intertab<string | null>("openstream.intertab.user_id");
    
  $: if(browser) {
    console.log("intertab set", { set_value: user_id, old_value: channel.get() });
    channel.set(user_id);  
  } 

  onMount(() => {
    console.log("root layout mounted", { user_id });
    return channel.watch((new_value, old_value) => {
      console.log("intertab user_id changed", { user_id, new_value, old_value })
      if(new_value !== user_id) {
        location.assign("/");
      }
    })
  })

</script>

<style>
  @import "$lib/css/app.css";
</style>

<slot />