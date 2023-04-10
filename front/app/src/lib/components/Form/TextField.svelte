<script lang="ts">
	export let value: string;
  export let label: string;
  export let type: string = "text";
  export let autocomplete: string | undefined = void 0;
  export let trim: boolean = false;
  export let disabled: boolean = false;

  export let multiline: boolean | undefined = void 0;
  export let minrows: number | undefined = void 0;
  export let maxrows: number | undefined = void 0;

  let empty = (trim ? value.trim() : value) === "";

  import FieldContainer from "./FieldContainer.svelte";
	import Input from "./Input.svelte";
  import Label from "./Label.svelte";
</script>

<FieldContainer {disabled}>
  <Input
    type={type}
    {autocomplete}
    {trim}
    {disabled}
    bind:value
    bind:empty
    on:input
    {multiline}
    {minrows}
    {maxrows}
  />
  <Label {label} full={!empty} />
</FieldContainer>