<script lang="ts">
  export let value: string | null | undefined;
  export let label: string;
  export let autocomplete = "email";
  export let disabled: boolean = false;
  export let readonly: boolean = false;
  export let on_change: ((v: string | null) => void) | null = null; 
  export let btn: { icon: string, label: string, action: () => void } | null = null;
	export let maxlength: number | undefined = void 0;

  import { mdiEmailOutline } from "@mdi/js";
  export let icon: string | null = mdiEmailOutline;

	import NullTextField from "./NullTextField.svelte";
</script>

<NullTextField type="email" {maxlength} {label} {icon} {btn} {disabled} {readonly} {autocomplete} trim bind:value on:input {on_change} />