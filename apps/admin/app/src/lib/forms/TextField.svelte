<script lang="ts">
  import { getContext, onMount } from "svelte";

  type Nullable<T> = T | undefined;

  export let type: "text" | "date" | "time" | "e-mail" | "tel" | "url" = "text";
  export let value: any = "";
  //export let placeholder = "";
  export let label: Nullable<string> = void 0;
  export let disabled: boolean = false;
  export let readonly: boolean = false;
  export let spellcheck: Nullable<boolean> = void 0; // string
  export let resize: Nullable<
    | "none"
    | "horizontal"
    | "vertical"
    | "both"
    | "unset"
    | "initial"
    | "inherit"
  > = void 0;
  export let id: Nullable<string> = void 0;
  export let name: Nullable<string> = void 0;
  export let step: Nullable<number> = void 0;
  export let max: Nullable<number> = void 0;
  export let min: Nullable<number> = void 0;
  export let nullable: boolean = false;
  export let trim: boolean = false;
  export let maxlength: number | undefined = void 0;

  export let multiline: boolean = false;
  export let minrows: number = 1;
  export let maxrows: number = Infinity;

  export let link: string | undefined = void 0;
  export let external: string | undefined = void 0;

  export let required: boolean = false;
  export let validate: boolean = false;

  export let validationError: string | null = null;

  export let focusable: HTMLElement | undefined = void 0;
  export let autocomplete: string | undefined = void 0;

  export let rounded = false;

  $: if(trim) value = value?.trim();
  $: if(!value && nullable) value = null;
  
  export const doValidation = (): boolean => {
    if(!validate) {
      validationError = null;
      return true
    };
   
    if(required && (value === "" || value == null)) {
      validationError = "¡Ups! Te faltó completar este campo";
      console.log("[Formy] validation fails [TextField] [Required]", label, value);
      return false;
    }

    if(type === "e-mail" && value !== "" && value != null && !isMail(value)) {
      validationError = "¡Ups! Al parecer este no es un email válido"
      console.log("[Formy] validation fails [TextField] [Email]", label, value);
      return false;
    }

    if(type === "url") {
      if(!(value == null || value === "" && !required)) {
        try {
          const url = new URL(value);
          if(!/^http/i.test(url.protocol)) {
            console.log("[Formy] validation fails [TextField] [URL] [protocol]", label, value);
            validationError = "¡Ups! La URL tiene que empezar por http:// o https://";
            return false;
          }
        } catch(e) {
          console.log("[Formy] validation fails [TextField] [URL]", label, value);
          validationError = "¡Ups! Esta URL es inválida";
          return false;
        }
      }
    }

    return true;
  }
  
  const formy = getContext("formy") as Context;
  onMount(() => formy && formy.register(doValidation));

  let rows = 1;
  if(multiline) {
    rows = Math.max(minrows, Math.min(maxrows, (value || "").split("\n").length));;
  }
  $: empty = value === "" || value == null;

  const handleChange = (event: any) => {
    
    validationError = null;

    const v: string = event.currentTarget.value;
    rows = Math.max(minrows, Math.min(maxrows, v.split("\n").length));
    
    let helper: string | null = v;
    if(trim) helper = helper.trim();
    if(!helper && nullable) helper = null;
    value = helper;
  };

  import { isMail } from "../util/validate";
  import type { Context } from "$share/components/Formy/Formy.svelte";
  import ValidationError from "$share/components/Formy/ValidationError.svelte";
  import { ripple } from "$share/actions";
</script>

<style>
  .text-field {
    position: relative;
    display: flex; 
  }

  .date.empty:not(:focus-within) {
    color: #fff;
  }

  input,
  textarea {
    flex: none;
    font: inherit;
    box-sizing: content-box;
    outline: none;
    flex: 1;
    margin: 0;
    border: 0;
    border-bottom: 1px rgba(0,0,0,0.23) solid;
    padding: 0.9em 0.75em;
    width: 100%;
    color: inherit;
    transition: border-color 150ms ease-in-out;
    background-color: #fff;
  }

  .rounded input, .rounded textarea {
    border: 1px rgba(0, 0, 0, 0.23) solid;
    border-radius: 3px;
  }

  textarea {
    resize: none;
    overflow: auto;
  }

  input:disabled,
  textarea:disabled {
    color: rgba(0, 0, 0, 0.5);
  }

  input:focus,
  textarea:focus {
    border-color: var(--blue);
  }

  .label {
    position: absolute;
    top: calc(2px + 0.9em);
    left: calc(1px + 0.75em);
    padding: 0 0.4em;
    margin: 0 -0.4em;
    pointer-events: none;
    background: #fff;
    color: rgba(0, 0, 0, 0.5);
    transition: color 150ms ease, transform 150ms ease-in-out;
    transform-origin: top left;
    max-width: calc(100% - 1em);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .text-field:not(.empty) > .label,
  input:focus + .label,
  textarea:focus + .label {
    transform: scale(0.7) translateY(-165%);
  }

  input:focus + .label,
  textarea:focus + .label {
    color: var(--blue);
  }

  .link {
    position: absolute;
    right: 0.5rem;
    top: 50%;
    transform: translateY(-50%);
    width: 2.25rem;
    height: 2.25rem;
    font-size: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
  }

  .with-link > input {
    padding-inline-end: 2.75rem;
  }
</style>

<div
  class="text-field"
  class:empty
  class:disabled
  class:readonly
  class:multiline
  class:rounded
  class:date={type === "date" || type === "time"}
  class:with-link={link || external}>
  {#if !multiline}
    <input
      bind:this={focusable}
      {id}
      {name}
      {type}
      {step}
      {max}
      {min}
      {value}
      {readonly}
      {disabled}
      {spellcheck}
      {autocomplete}
      {maxlength}
      on:input={handleChange}
      on:input
      on:focus={() => validationError = null}
      on:focus
      on:blur
      on:focusin
      on:focusout
      on:change
      on:keypress
      on:keydown
      on:keyup />
    {#if label != null}
      <span class="label">{label}</span>
    {/if}
  {:else}
    <textarea
      bind:this={focusable}
      {id}
      {name}
      {value}
      {readonly}
      {disabled}
      {rows}
      {spellcheck}
      {autocomplete}
      {maxlength}
      on:input={handleChange}
      on:input
      on:focus={() => validationError = null}
      on:focus
      on:blur
      on:change
      on:keypress
      on:keydown
      on:keyup
      style={resize ? `resize: " ${resize}` : ''} />
    {#if label != null}
      <span class="label">{label}</span>
    {/if}
  {/if}
  {#if validationError}
    <ValidationError message={validationError} on:click={() => focusable && focusable.focus()} />
  {/if}
</div>
