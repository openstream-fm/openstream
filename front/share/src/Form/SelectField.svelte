<script lang="ts">
	export let value: string;
  export let label: string;
  export let autocomplete: string | undefined = void 0;
  export let disabled: boolean = false;
  export let readonly: boolean = false;
  export let on_change: ((v: string) => void) | null =  null; 
  export let icon: string | null = null;
  export let options: Array<{ label: string, value: string }>;

  $: empty = value === "";

  import FieldContainer from "./FieldContainer.svelte";
	import Label from "./Label.svelte";
	import Select from "./Select.svelte";
</script>

<FieldContainer {disabled} {readonly} {icon}>
  <Select
    bind:value
    on:input
    {disabled}
    {readonly}
    {options}
    {autocomplete}
    {on_change}
  />
  <Label {label} full={!empty} />
</FieldContainer>