<script lang="ts" context="module">
  import { crossfade, fade } from "svelte/transition";
  const [ send, receive ] = crossfade({ duration: 300, fallback: fade as any })

  const is_current = (href: string, url: URL): boolean => {
    const path = href.split("?")[0];
    if(/^\/accounts\/([a-z0-9]+)\/?$/i.test(path)) {
      return url.pathname === path
    } else {
      return url.pathname === path || url.pathname.startsWith(path + "/");
    }
  }
</script>

<script lang="ts">
  export let href: string;
  export let label: string;
  export let icon: string | null = null;
  export let iconStroke: boolean = false;
  export let iconStrokeWidth: number | "" = "";
  export let on_click: () => void = () => {};
  $: current = is_current(href, $page.url);

  import { page } from "$app/stores";
	import { ripple } from "$share/ripple";
	import Icon from "$share/Icon.svelte";
</script>

<style>
  .drawer-item {
    position: relative;
  }

  a {
    user-select: none;
    transition: background-color 200ms ease;
    display: flex;
    flex-direction: row;
    align-items: center;
    position: relative;
    overflow: hidden;
    --h: 3.5rem;
    height: var(--h);
    /*border-radius: 0 5rem 5rem 0;*/
  }

  a:hover {
    background: rgba(0,0,0,0.025);
  }

  .current {
    background: rgba(0,0,0,0.05);
  }

  .current-line {
    position: absolute;
    inset-inline-start: 0;
    inset-block-start: 0;
    inset-block-end: 0;
    inline-size: 3px;
    background: var(--red);
  }

  .icon {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.2rem;
    margin-inline: 1.5rem 1.25rem;
  }
</style>

<div class="na drawer-item">
  <a {href} class="na ripple-container" class:current use:ripple on:click={() => { on_click() }}>

    <div class="icon">
      {#if icon}
        <Icon d={icon} stroke={iconStroke} strokeWidth={iconStrokeWidth} />
      {/if}
    </div>
  
    <div class="content">
      {label}
    </div>
  </a>

  {#if current}
    <div class="current-line" out:send|local={{ key: null }} in:receive|local={{ key: null }} />
  {/if}
</div>
