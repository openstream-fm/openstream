<script lang="ts" context="module">
	import type { StationTypeOfContent } from "$server/defs/db/StationTypeOfContent";
  const default_type_names: Record<StationTypeOfContent, string> = {
    general: "General",
    talk: "Talk",
    news: "News",
    music: "Music",
    sports: "Sports",
    comedy: "Comedy",
    educational: "Educational",
    religious: "Religious",
  }
</script>

<script lang="ts">
  export let value: StationTypeOfContent | "";
  export let icon: string | null = mdiRadio;
  export let label: string;
  export let autocomplete: string | undefined = void 0;
  export let disabled: boolean = false;
  export let on_change: ((v: string) => void) | null =  null; 
  export let locale_names: Record<StationTypeOfContent, string>;  

  $: required_options = Object.entries(default_type_names).map(([value, name]) => {
    return {
      value,
      label: locale_names[value as StationTypeOfContent] || name,
    }
  })

  $: empty = value === "";
  $: options = value === "" ? [{label: "", value: ""}, ...required_options] : required_options;

  import { mdiRadio } from "@mdi/js";
	import FieldContainer from "./FieldContainer.svelte";
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