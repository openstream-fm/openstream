<script lang="ts">
  export let value: string;
  export let label: string;
  export let autocomplete = "email";

	import TextField from "./TextField.svelte";
</script>

<TextField type="email" {label} {autocomplete} trim bind:value on:input />