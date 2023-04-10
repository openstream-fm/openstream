<script lang="ts">
  export let value: string;
  export let label: string;
  export let autocomplete = "email";
  export let disabled: boolean = false;

	import TextField from "./TextField.svelte";
</script>

<TextField type="email" {label} {disabled} {autocomplete} trim bind:value on:input />