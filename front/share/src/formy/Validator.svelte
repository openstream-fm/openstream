<script lang="ts">
  type Value = $$Generic;
  export let value: Value;
  export let fn: (v: Value) => MaybePromise<string | null>; 

  import { sleep } from "$share/util";
  import type { MaybePromise } from "./util";
  import { slide } from "svelte/transition";
  import { FORMY_KEY } from "./formy";
  import type { FormyContext } from "./formy";
  import { getContext } from "svelte";

  let current_message: string | null;

  $: on_value(value);
  let _token = 0;
  let on_value_executing = false;
  const on_value = async (...args: any[]) => {
    if(current_message != null) {
      const token = ++_token;
      while(on_value_executing) {
        await sleep(100);
        if(token !== _token) return;
      }

      on_value_executing = true;
      
      try {
        const message = await fn(value);
        if(token === _token) {
          current_message = message;
        }
        on_value_executing = false;
      
      } catch(e) {
        on_value_executing = false;
      }
    }
  }

  const context = getContext<FormyContext | undefined>(FORMY_KEY);

  const anchor = (node: HTMLElement) => {
    if(context != null) {
      const parent_element = node.parentElement;
      if(parent_element != null) {
        const validate = async () => {
          let token = ++_token;
          let message = await fn(value);
          if(token === _token) {
            current_message = message;
          }
          return message;
        }
        return {
          destroy: context.add({ fn: validate, parent_element })
        }
      }
    }
  }
</script>

<style>
  .validator-anchor {
    display: none;
  }

  .message {
    font-size: var(--validator-message-font-size, 0.9rem);
    margin: var(--validator-message-margin, 0.5rem 0 0 0);
    color: var(--validator-message-color, var(--red));
  }
</style>

<div class="validator-anchor" use:anchor aria-hidden hidden />

{#if current_message != null}
  <div class="message" transition:slide={{ duration: 200 }} aria-errormessage={current_message}>
    {current_message}
  </div>
{/if}