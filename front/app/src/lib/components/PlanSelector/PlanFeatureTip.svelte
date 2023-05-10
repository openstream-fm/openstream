<script lang="ts">
  export let text: string;

  import Icon from "$share/Icon.svelte";
	import { tooltip } from "$share/tooltip";
	import { mdiInformation } from "@mdi/js";
</script>

<style>
  .feature-i {
    display: flex;
  }

  .icon {
    display: flex;
  }
</style>

<div class="feature-i" aria-label={text} use:tooltip={text}>
  <div class="icon">
    <Icon d={mdiInformation} />
  </div>
</div>