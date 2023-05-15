<script lang="ts">
	export let value: number | null;
  export let label: string;
  export let autocomplete: string | undefined = void 0;
  export let disabled: boolean = false;
  export let readonly: boolean = false;
  export let max: number | undefined = void 0;
  export let min: number | undefined = void 0;
  export let step: number | undefined = void 0;
  export let icon: string | null = null;
  export let on_change: ((v: number | null) => void) | null =  null; 
  export let btn: { icon: string, action: () => void } | null = null;

  let empty = Number.isNaN(value);
  
  let prev_value = value;
  let text_value = String(value ?? ""); 
  let prev_text_value = text_value

  $: on_value(value);
  const on_value = (...args: any[]) => {
    if(prev_value === value) return;
    prev_value = value;
    set_text_value(String(value ?? ""));
  }

  const set_value = (v: number | null) => {
    prev_value = v;
    value = v;
  }

  const set_text_value = (v: string) => {
    prev_text_value = v;
    text_value = v;
  }

  $: on_text_value(text_value);
  const on_text_value = (...args: any) => {
    if(prev_text_value === text_value) return;
    prev_text_value = text_value;
    const v = Number(text_value);
    set_value(Number.isNaN(v) ? null : v);
  }

  const internal_on_change = (v: string) => {
    text_value = v;
    on_change?.(value);
  }

  import FieldContainer from "../FieldContainer.svelte";
	import Input from "../Input.svelte";
  import Label from "../Label.svelte";
</script>

<FieldContainer {disabled} {readonly} {icon} {btn}>
  <Input
    type="number"
    {autocomplete}
    {disabled}
    {min}
    {max}
    {step}
    {readonly}
    bind:value={text_value}
    bind:empty
    on:input
    on_change={internal_on_change}
  />
  <Label {label} full={!empty} />
</FieldContainer>