<script lang="ts">
	export let value: Date | null;
  export let label: string;
  export let type: string = "datetime-local";
  export let autocomplete: string | undefined = void 0;
  export let trim: boolean = false;
  export let disabled: boolean = false;
  export let readonly: boolean = false;

  let value_str = value?.toISOString().replace(/Z$/, "") || "";

  export let icon: string | null | undefined = void 0;
  export let icon_viewbox: string | undefined = void 0;
  export let btn: { icon: string, label: string, action: () => void } | null = null;
	
  export let on_change: ((v: Date | null) => void) | null = null;

  const internal_on_change = (v: string) => {
    const d = new Date(v);
    if(Number.isNaN(+d)) {
      value = null;
    } else {
      value = d;
    }
    on_change?.(value);    
  }
  
  import FieldContainer from "./FieldContainer.svelte";
  import Input from "./Input.svelte";
  import Label from "./Label.svelte";
</script>

<FieldContainer {disabled} {readonly} {icon} {icon_viewbox} {btn}>
  <Input
    type={type}
    {autocomplete}
    {trim}
    {disabled}
    {readonly}
    value={value_str}
    on:input={event => {
      // @ts-ignore
      internal_on_change((event.currentTarget?.value || ""));
    }}
  />
  <Label {label} full={true} />
</FieldContainer>