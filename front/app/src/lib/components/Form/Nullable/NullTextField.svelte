<script lang="ts">
	export let value: string | null;
  export let label: string;
  export let type: string = "text";
  export let autocomplete: string | undefined = void 0;
  export let trim: boolean = false;

  export let multiline: boolean | undefined = void 0;
  export let minrows: number | undefined = void 0;
  export let maxrows: number | undefined = void 0;

  let empty = !(trim ? value?.trim() : value);

  import FieldContainer from "../FieldContainer.svelte";
	import NullInput from "./NullInput.svelte";
  import Label from "../Label.svelte";
</script>

<FieldContainer>
  <NullInput
    type={type}
    {autocomplete}
    {trim}
    {multiline}
    {minrows}
    {maxrows}
    bind:value
    bind:empty
    on:input
  />
  <Label {label} full={!empty} />
</FieldContainer>