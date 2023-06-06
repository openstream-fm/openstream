<script lang="ts">
	import { onMount } from "svelte";
  import { intertab } from "$share/intertab";
	import { browser } from "$app/environment";
	import { default_logger } from "$share/logger";
	import { goto } from "$app/navigation";
  import Loading from "$share/Loading.svelte";
	import { invalidate_siblings } from "$lib/invalidate";

  export let data: import("./$types").LayoutData;

  $: admin_id = data.maybe_admin?._id ?? null;
  
  const logger = default_logger.scoped("app")

  const channel = intertab<string | null>("openstream.intertab.admin_id");
    
  $: if(browser) {
    logger.info("intertab set", { set_value: admin_id, old_value: channel.get() });
    channel.set(admin_id);  
  } 

  onMount(() => {
    logger.info("app layout mounted", { admin_id });
    return channel.watch((new_value, old_value) => {
      logger.info("intertab admin_id changed", { admin_id, new_value, old_value })
      if(new_value !== admin_id) {
        goto("/", { invalidateAll: true })
        invalidate_siblings();
      }
    })
  })

</script>

<Loading />

<slot />