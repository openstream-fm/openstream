import { fly as _fly } from "svelte/transition";

export const logical_fly: typeof _fly = (node, params = {}) => {
  let { x, ...rest } = params;
  if(typeof x === "number" && getComputedStyle(node).direction === "rtl") x = -x; 
  return _fly(node, {
    ...rest,
    x,
  })
}