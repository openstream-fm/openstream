<script lang="ts">
  export let value: LangCode | "";
  export let icon: string | null = mdiTranslate;
  export let label: string;
  export let autocomplete: string | undefined = void 0;
  export let disabled: boolean = false;
  export let on_change: ((v: string) => void) | null =  null; 
  export let lang_names: Record<LangCode, string>;  

  $: lang_options = Object.entries(lang_names).map(([ value, label ]) => {
    return { value, label: lang_names[value as LangCode] || label }
  }).sort((a, b) => a.label.localeCompare(b.label));

  $: empty = value === "";
  $: options = value === "" ?
    [ {label: "", value: ""}, ...lang_options ] :
    lang_options;

  import { mdiTranslate } from "@mdi/js";
	import FieldContainer from "./FieldContainer.svelte";
	import Label from "./Label.svelte";
	import Select from "./Select.svelte";
	import type { LangCode } from "$server/defs/LangCode";
</script>

<FieldContainer {icon}>
  <Select
    {autocomplete}
    {disabled}
    bind:value
    {options}
    {on_change}
  />
  <Label {label} full={!empty} />
</FieldContainer>