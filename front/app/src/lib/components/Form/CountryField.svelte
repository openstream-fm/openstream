<script lang="ts">
  export let value: CountryCode | "";
  export let icon: string | null = mdiMapMarkerOutline;
  export let label: string;
  export let autocomplete: string | undefined = void 0;
  export let disabled: boolean = false;
  export let on_change: ((v: string) => void) | null =  null; 
  export let country_names: Record<string, string | undefined>;  

  import { country_names as english_country_names } from "$share/geo";
  $: country_options = Object.entries(english_country_names).map(([ value, label ]) => {
    return { label: country_names[value] || label, value }
  }).sort((a, b) => a.label.localeCompare(b.label));

  $: empty = value === "";
  $: options = value === "" ?
    [ {label: "", value: ""}, ...country_options ] :
    country_options;

  import { mdiMapMarkerOutline } from "@mdi/js";
	import FieldContainer from "./FieldContainer.svelte";
	import type { CountryCode } from "$server/defs/CountryCode";
	import Label from "./Label.svelte";
	import Select from "./Select.svelte";
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