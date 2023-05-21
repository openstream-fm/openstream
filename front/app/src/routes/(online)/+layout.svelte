<script lang="ts">
	import { onMount } from "svelte";
  import { intertab } from "$share/intertab";
	import { browser } from "$app/environment";
	import { default_logger } from "$share/logger";
	import { goto } from "$app/navigation";
  import Loading from "$share/Loading.svelte";
	import { invalidateSiblings } from "$lib/invalidate";

  export let data: import("./$types").LayoutData;

  $: user_id = data.maybe_user?._id ?? null;
  
  const logger = default_logger.scoped("app")

  const channel = intertab<string | null>("openstream.intertab.user_id");
    
  $: if(browser) {
    logger.info("intertab set", { set_value: user_id, old_value: channel.get() });
    channel.set(user_id);  
  } 

  onMount(() => {
    logger.info("app layout mounted", { user_id });
    return channel.watch((new_value, old_value) => {
      logger.info("intertab user_id changed", { user_id, new_value, old_value })
      if(new_value !== user_id) {
        goto("/", { invalidateAll: true })
        invalidateSiblings();
      }
    })
  })

</script>

<Loading />

<slot />