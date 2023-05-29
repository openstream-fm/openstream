<script lang="ts">
  export let data: import("./$types").LayoutData;

  import { onMount } from "svelte";
  import { browser } from "$app/environment";
	import { default_logger } from "$share/logger";
	import { goto } from "$app/navigation";
  import Loading from "$share/Loading.svelte";
	import { user_id_channel } from "$share/intertab-user-id-channel"; 

  $: user_id = data.maybe_user?._id ?? null;
  
  const logger = default_logger.scoped("app")
  
  $: if(browser) {
    logger.info("intertab set", { set_value: user_id, old_value: user_id_channel.get() });
    user_id_channel.set(user_id);  
  }

  onMount(() => {
    logger.info("app layout mounted", { user_id });
    return user_id_channel.watch((new_value, old_value) => {
      logger.info("intertab user_id changed from another tab", { user_id, new_value, old_value })
      if(new_value !== user_id) {
        logger.info("intertab user_id changed, local doesn't match, navigating to root");
        goto("/", { invalidateAll: true })
      }
    })
  })

</script>

<Loading />

<slot />