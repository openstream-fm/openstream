<script lang="ts">
	export let value: string | null;

  let prev_value = value;
  let display_value = value || "";
  // svelte-ignore unused-export-let
  export let empty: boolean = display_value === "";

  export let type: string = "text";
  export let autocomplete: string | undefined = void 0;
  export let trim: boolean = false;

  export let disabled: boolean = false;

  export let multiline: boolean = false;
  export let minrows: number = 3;
  export let maxrows: number = 1000;

  $: empty = display_value === "";

  $: text_lines = (display_value || "").split("\n").length;
  $: rows = Math.max(minrows, Math.min(maxrows, text_lines));

  $: on_value(value);
  const on_value = (...args: any[]) => {
    if(prev_value !== value) {
      prev_value = value;
      display_value = value || "";
    }
  }

  const set_value = (current: string | null) => {
    prev_value = current;
    value = current;
  }

  const on_input = (current_display: string) => {
    display_value = current_display;
    let h = current_display;
    if(trim) h = h.trim();
    set_value(h || null);
  }

  import css from "../forms.module.css"; 
</script>

{#if multiline}
  <textarea class={css["forms-input"]} {rows} {autocomplete} {disabled} value={display_value} on:input={event => on_input(event.currentTarget.value)} on:input />
{:else}
  <input class={css["forms-input"]} {type} {autocomplete} {disabled} value={display_value} on:input={event => on_input(event.currentTarget.value)} on:input />
{/if}
  