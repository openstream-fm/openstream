<script lang="ts">
  export let value: string | null;
  export let label: string;
  export let autocomplete = "email";
  export let disabled: boolean = false;
  export let readonly: boolean = false;
  export let on_change: ((v: string | null) => void) | null = null; 
  export let btn: { icon: string, action: () => void } | null = null;
	
  import { mdiEmailOutline } from "@mdi/js";
  export let icon: string | null = mdiEmailOutline;

	import NullTextField from "./NullTextField.svelte";
</script>

<NullTextField type="email" {label} {icon} {btn} {disabled} {readonly} {autocomplete} trim bind:value on:input {on_change} />