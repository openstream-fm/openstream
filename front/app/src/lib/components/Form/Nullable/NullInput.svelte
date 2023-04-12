<script lang="ts">
	import Input from "../Input.svelte";

	export let value: string | null;
  export let type: string = "text";
  export let autocomplete: string | undefined = void 0;
  export let trim: boolean = false;
  export let disabled: boolean = false;
  export let empty = !value;

  export let multiline: boolean | undefined = void 0;
  export let minrows: number | undefined = void 0;
  export let maxrows: number | undefined = void 0;

  export let on_change: ((v: string | null) => void) | null = null;

  const internal_on_change = (v: string) => {
    let out_v = v || null;
    value = out_v;
    on_change?.(out_v);    
  }
</script>

<Input
  {type}
  {disabled}
  {autocomplete}
  {trim}
  {multiline}
  {minrows}
  {maxrows}
  value={value || ""}
  on:input
  bind:empty
  on_change={internal_on_change}
/>