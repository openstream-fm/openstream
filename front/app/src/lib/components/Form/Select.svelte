<script lang="ts">
	export let value: string;
  export let autocomplete: string | undefined = void 0;
  export let disabled: boolean = false;
  export let readonly: boolean = false;

  export let options: Array<{ label: string, value: string }>;
  export let on_change: ((v: string) => void) | null = null; 

  import css from "./forms.module.css"; 
</script>

<select class={css["forms-input"]} class:thin-scroll={true} {autocomplete} disabled={disabled || readonly}  bind:value on:change={event => on_change?.(event.currentTarget.value)}>
  {#each options as item}
    <option value={item.value} selected={value === item.value}>{item.label}</option>
  {/each}
</select>
