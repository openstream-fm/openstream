<script lang="ts">
  import { browser } from "$app/environment";
  import { navigating } from "$app/stores";

  let frame: number;
  let el: HTMLElement;  
  let max = 0.9;
  let min = 0.1;
  let last = !!$navigating;
  
  $: browser && onChange(!!$navigating);

  const onChange = (v: boolean) => {
    if(el == null) return;
    if(!el.animate) return;
    if(last === v) return;
    last = v;
    if(v) {
      // el.animate({
      //   width: [`${min}%`, `${max}%`],
      //   opacity: [0, 1],
      // }, {
      //   easing: "ease",
      //   duration: 400,
      // })
      let w = min;
      el.style.opacity = "1";
      el.style.width = `${w * 100}%`;
      frame = requestAnimationFrame(function f(){
        if(!el) return;
        const nw = Math.min(w + 0.01, max);
        if(nw === w) return;
        w = nw;
        el.style.width = `${w * 100}%`;
        frame = requestAnimationFrame(f);
      })
    } else{
      // el.animate({
      //   width: "100%",
      //   opacity: 0,
      // }, {
      //   easing: "ease",
      //   duration: 200,
      // });
      if(frame) cancelAnimationFrame(frame);
      el.style.width = "100%"
      el.style.opacity = "0"
    }
  }
</script>

<style>
  .loading {
    background-color: var(--loading-color);
    position: fixed;
    z-index: 10000000;
    top: 0;
    left: 0;
    opacity: 0;
    height: 2px;
    pointer-events: none;
    transition: opacity 300ms ease 200ms;
  }
</style>

<div class="loading" bind:this={el} />