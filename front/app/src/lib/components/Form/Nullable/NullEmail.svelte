<script lang="ts">
  export let value: string | null;
  export let label: string;
  export let autocomplete = "email";

	import NullTextField from "./NullTextField.svelte";
</script>

<NullTextField type="email" {label} {autocomplete} trim bind:value on:input />