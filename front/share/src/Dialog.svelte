<script lang="ts">
  import { onMount } from "svelte";
  import { add } from "$share/util";

  export let title: string | undefined = void 0;
  export let width: string = "800px";
  export let padding: string = "1.5rem";
  export let on_close: () => void = () => {};

  onMount(() => {
    const uid = Array(5).fill(0).map(() => Math.floor(Math.random() * 36).toString(36)).join("");
    const hash = `#dialog-${uid}`;    
    location.hash += hash;
    const off = [
      add(window, "keydown", event => {
        if((event as KeyboardEvent).key === "Escape") on_close();
      }, { capture: true }),
      
      add(window, "hashchange", () => {
        if(!location.hash.includes(hash)) on_close();
      }),

      () => {
        if(location.hash.includes(hash)) {
          history.back();
        }
      }
    ];
  
    return () => {
      for(const fn of off) fn();
    }
  })

  import { fade } from "svelte/transition";

  const custom = (_node: Element, _options = {}) => {
    return {
      duration: 150,
      css: (t: number, u: number) => `transform: translateY(${100 * u}px) scale(${0.5 + 0.5 * t}); opacity: ${t}` 
    }
  }
</script>

<style>
  .overlay {
    padding: 5rem 1rem;
    cursor: pointer;
  }

  .dialog {
    cursor: default;
    width: var(--width);
    max-width: 90%;
    margin: auto;
    background: #fff;
    margin: auto;
    box-sizing: border-box;
    /*overflow: hidden;*/
    border-radius: 0 0 0.5rem 0.5rem;
    border-top: 2px var(--red) solid;
  }

  .title {
    padding: 1rem;
    font-size: 1.1rem;
    font-weight: 500;
    /*
    background: rgba(0,0,0,0.05);
    border-bottom: rgba(0,0,0,0.18) 1px solid;
    */
  }

  .content {
    padding: var(--padding);
  }
</style>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
  class="overlay"
  on:click={on_close}
  transition:fade={{ duration: 200 }}
>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div
    class="dialog elev3"
    style="--width: {width}; --padding: {padding}"
    on:click|stopPropagation={() => {}}
    transition:custom
  >
    {#if title}
      <div class="title">{title}</div>
    {/if}
    <div class="content">
      <slot />
    </div>
  </div>
</div>
