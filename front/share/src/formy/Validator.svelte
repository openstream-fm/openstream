<script lang="ts">
  import { MaybePromise } from "./util";

  import { slide } from "svelte/transition";

  type Value = $$Generic;
  export let value: Value;
  export let fn: (v: Value) => MaybePromise<string | null>; 

  import { FORMY_KEY } from "./formy";
  import type { FormyContext } from "./formy";
  import { getContext } from "svelte";
  // import { add } from "$share/util";

  let current_message: string | null;

  $: on_value(value);
  let _token = 0;
  const on_value = async (...args: any[]) => {
    if(current_message != null) {
      const token = ++_token; 
      const message = await fn(value);
      if(token === _token) {
        current_message = message;
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

  // const mount = (node: Node) => {
  //   const parent = node.parentElement;
  //   if(parent) {
  //     return {
  //       destroy: add(parent, "focusin", event => {
  //         current_message = null;
  //       })
  //     }
  //   }
  // }
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
  <div class="message" transition:slide|local={{ duration: 200 }} aria-errormessage={current_message}>
    {current_message}
  </div>
{/if}