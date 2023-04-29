<script lang="ts" context="module">
  const options = [
    { label: "", value: "" },
    ...Object.entries(country_names).map(([ value, label ]) => {
      return { label, value }
    })
  ];
</script>

<script lang="ts">
  export let value: CountryCode | "";
  export let icon: string | null = mdiMapMarkerOutline;
  export let label: string;
  export let autocomplete: string | undefined = void 0;
  export let disabled: boolean = false;
  export let on_change: ((v: string) => void) | null =  null; 

  let empty = value === "";

  import { country_names } from "$share/geo";
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