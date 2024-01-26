<script lang="ts">
  export let value: GooglePlayLang | "";
  export let icon: string | null = mdiTranslate;
  export let label: string;
  export let autocomplete: string | undefined = void 0;
  export let disabled: boolean = false;
  export let on_change: ((v: string) => void) | null =  null; 
  export let lang_names: Record<GooglePlayLang, string> = google_play_langs;  

  $: lang_options = Object.entries(lang_names).map(([ value, label ]) => {
    return { value, label: lang_names[value as GooglePlayLang] || label }
  }).sort((a, b) => a.label.localeCompare(b.label));

  $: empty = value == null || value === "";
  $: options = empty ? [ {label: "", value: ""}, ...lang_options ] : lang_options;

  import { mdiTranslate } from "@mdi/js";
	import FieldContainer from "$share/Form/FieldContainer.svelte";
	import Label from "$share/Form/Label.svelte";
	import Select from "$share/Form/Select.svelte";
	import { google_play_langs, type GooglePlayLang } from "./google-play-lang";
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