<script lang="ts">
	export let value: string;
  export let label: string;
  export let type: string = "text";
  export let autocomplete: string | undefined = void 0;

  import FieldContainer from "./FieldContainer.svelte";
  import Label from "./Label.svelte";
  import css from "./forms.module.css"; 
</script>

<FieldContainer>
  <input type={type} value={value} {autocomplete} class={css["forms-input"]} on:input={event => value = event.currentTarget.value} />
  <Label {label} full={value !== ""} />
</FieldContainer>