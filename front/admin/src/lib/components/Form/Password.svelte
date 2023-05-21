<script lang="ts">
	export let value: string;
  export let label: string;
  export let visible: boolean = false;
  export let autocomplete = "password";
  export let disabled: boolean = false;
  export let readonly: boolean = false;
  export let on_change: ((v: string) => void) | null = null;
  export let icon: string | null = mdiLockOutline; 

  import { mdiEye, mdiEyeOff, mdiLockOutline } from "@mdi/js";
	import TextField from "./TextField.svelte";
	import { locale } from "$lib/locale";

  const action = () => visible = !visible;
  $: btn = {
    action,
    icon: visible ? mdiEyeOff : mdiEye,
    tabindex: -1,
    label: visible ? $locale.hide_password : $locale.show_password,
  }

  $: type = visible ? "text" : "password";
</script>

<TextField {type} {label} {icon} {btn} {disabled} {readonly} {autocomplete} bind:value {on_change} on:input/>