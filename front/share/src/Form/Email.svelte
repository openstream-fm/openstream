<script lang="ts">
  export let value: string;
  export let label: string;
  export let autocomplete = "email";
  export let disabled: boolean = false;
  export let readonly: boolean = false;
  export let on_change: ((v: string) => void) | null = null;
  export let btn: { icon: string, label: string, action: () => void } | null = null;
	
  import { mdiEmailOutline } from "@mdi/js";
  export let icon: string | null = mdiEmailOutline;

	import TextField from "./TextField.svelte";
</script>

<TextField type="email" {label} {icon} {btn} {disabled} {readonly} {autocomplete} trim bind:value on:input {on_change} />