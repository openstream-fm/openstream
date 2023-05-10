<script lang="ts">
	export let value: string;
  export let label: string;
  export let visible: boolean = false;
  export let autocomplete = "password";
  export let disabled: boolean = false;
  export let readonly: boolean = false;
  export let on_change: ((v: string) => void) | null = null;

  import { mdiEye, mdiEyeOff, mdiLockOutline } from "@mdi/js";
	import TextField from "./TextField.svelte";
  export let icon: string | null = mdiLockOutline; 

  const action = () => visible = !visible;
  $: btn = {
    action,
    icon: visible ? mdiEyeOff : mdiEye,
  }

  $: type = visible ? "text" : "password";
</script>

<TextField {type} {label} {icon} {btn} {disabled} {readonly} {autocomplete} bind:value {on_change} on:input/>