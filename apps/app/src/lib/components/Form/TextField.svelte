<script lang="ts">
	export let value: string;
  export let label: string;
  export let type: string = "text";

  import FieldContainer from "./FieldContainer.svelte";
  import Label from "./Label.svelte";
</script>

<style>
  input {
    display: block;
    flex: 1;
    border: 0;
    outline: 0;
    padding: var(--spacing);
    min-width: 0;
    background: transparent;
  }
</style>

<FieldContainer>
  <input type={type} value={value} on:input={event => value = event.currentTarget.value} />
  <Label {label} full={value !== ""} />
</FieldContainer>