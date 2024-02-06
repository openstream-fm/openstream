<script>
	import Player from "$lib/components/Player/Player.svelte";
  import { close } from "$lib/components/Player/player";
  import { onMount } from "svelte";
  onMount(() => {
    return () => close();
  })
</script>

<style>
  .player-layout {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .player-layout-dashboard {
    flex: 1;
    display: flex;
  }
</style>

<div class="player-layout">
  <div class="player-layout-dashboard">
    <slot />
  </div>
  <Player />
</div>