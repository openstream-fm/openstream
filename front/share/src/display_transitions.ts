import { tick } from "svelte";

export type FlyEnterParams = {
  show: boolean,
  start: boolean,
  duration?: number,
  delay?: number,
  easing?: string,
  x?: number,
  y?: number,
}

export const display_fly_enter = (node: HTMLElement, _params: FlyEnterParams) => {
  
  let prev_show = _params.start ? false : _params.show;

  const update = async (params: FlyEnterParams) => {
    if(prev_show === params.show) return;
    prev_show = params.show;
    
    if(!params.show) return;

    const {
      delay = 0,
      duration = 200,
      easing = "ease",
      x: _x = 0,
      y = 0.
    } = params;

    await tick();

    const css = getComputedStyle(node);
    const x = css.direction === "rtl" ? -_x : _x;
    const transform = `${css.transform === "none" ? "" : css.transform} translateX(${x}px) translateY(${y}px)`

    node.animate({
      easing,
      opacity: [0, Number(css.opacity)],
      transform: [transform, css.transform],
    }, {
      delay,
      duration,
    })
  }

  update(_params);

  return {
    update
  }
}