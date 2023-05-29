<script lang="ts">
  import { browser } from "$app/environment";
	import { lang, dir } from "$lib/locale";

  $: browser && (document.documentElement.lang = $lang);
  $: browser && (document.documentElement.dir = $dir);
</script>

<slot />