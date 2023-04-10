<script lang="ts">
  export let value: string | null;
  export let label: string;
  export let autocomplete = "email";
  export let disabled: boolean = false;
  
	import NullTextField from "./NullTextField.svelte";
</script>

<NullTextField type="email" {label} {disabled} {autocomplete} trim bind:value on:input />