<script lang="ts">
  export let data: import("./$types").LayoutData;
	import Keyed from "$share/Keyed.svelte";
  import KeyedLayout from "./keyed-layout.svelte";
</script>

<Keyed key={data.station._id}>
  <KeyedLayout bind:data>
    <slot />
  </KeyedLayout> 
</Keyed>