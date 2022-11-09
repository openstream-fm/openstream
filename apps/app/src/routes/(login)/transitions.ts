import { cubicIn } from "svelte/easing";

export const form = (node: HTMLElement, _params = {}) => {
  return {
    ease: cubicIn,
    duration: 150,
    css: (t: number, u: number) => {
      return `opacity: ${t  * 1}; transform: scale(${0.75 + t * 0.25}) translateY(${u * -100}px)`;
    }
  } 
}