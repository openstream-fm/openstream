<script>
	import Player from "$lib/components/Player/Player.svelte";
</script>

<style>
  .player-layout {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
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