<script lang="ts">
	export let value: string | null;
  export let label: string;
  export let type: string = "text";
  export let autocomplete: string | undefined = void 0;
  export let trim: boolean = false;
  export let disabled: boolean = false;
  
  export let multiline: boolean | undefined = void 0;
  export let minrows: number | undefined = void 0;
  export let maxrows: number | undefined = void 0;

  export let icon: string | null | undefined = void 0;

  export let on_change: ((v: string | null) => void) | null = null;

  const internal_on_change = (v: string) => {
    value = v || null;
    on_change?.(value);    
  }
  
  import TextField from "../TextField.svelte";
</script>

<TextField
  {type}
  {label}
  {disabled}
  {autocomplete}
  {trim}
  {multiline}
  {minrows}
  {maxrows}
  {icon}
  value={value || ""}
  on:input
  on_change={internal_on_change}
/>