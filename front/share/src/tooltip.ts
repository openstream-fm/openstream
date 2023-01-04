import { add, tick } from "./util";

export const tooltip = (node: HTMLElement, _params: null | string | {tip: string}) => {

  const params: {tip: string | null} = ((typeof _params === "string" || _params == null) ? {tip: _params} : _params) as {tip: string | null};

  let el = document.createElement("div");
  el.classList.add("tooltip");
  el.textContent = params.tip;
  let timer: any;
  let on = false;

  node.appendChild(el);

  let removeScroll: (() => void) | null = null;

  const removeEnter = add(node, "mouseenter", async () => {
    if(params.tip == null) return;
    on = true;
    clearTimeout(timer);
    el.classList.remove("visible");
    const target = node.getBoundingClientRect();
    document.body.appendChild(el);
    await tick();
    el.style.left = Math.max(5, Math.min(window.innerWidth - 5, target.left + (target.width / 2) - (el.clientWidth / 2))) + "px";
    el.style.top = Math.max(5, Math.min(window.innerHeight - 5, target.top - el.clientHeight - 7)) + "px";
    el.classList.add("visible");
    removeScroll = add(window, "scroll", () => removeTooltip(), {once: true, capture: true, passive: true})
  })

  const removeTooltip = () => {
    on = false;
    el.classList.remove("visible");
    if(removeScroll) removeScroll();
    timer = setTimeout(() => {
      el.parentElement && el.parentElement.removeChild(el);
    }, 200)
  }

  const removeLeave = add(node, "mouseleave", removeTooltip);

  return {
    update(opts: string | null | {tip: string | null}) {
      if (typeof opts === "string" || opts == null) {
        opts = { tip: opts } as { tip: string | null };
      }

      params.tip = opts.tip;
      el.textContent = opts.tip;
      if(!params.tip) {
        on = false
        el.parentElement && el.parentElement.removeChild(el);
      }
    },

    destroy() {
      removeEnter();
      removeLeave();
      if(removeScroll) removeScroll();
      if (el.parentElement) el.parentElement.removeChild(el);
    }
  }
}