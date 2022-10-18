import { run_all } from "svelte/internal";

export const add = <E extends Event = Event>(target: EventTarget, event: string, fn: (event: E) => void, options: AddEventListenerOptions = {}) => {
  // @ts-ignore
  target.addEventListener(event, fn, options);
  return () => {
    // @ts-ignore
    target.removeEventListener(event, fn, options);
  }
}

export const intersect = (node: Element, options: IntersectionObserverInit = {}) => {
  if (typeof IntersectionObserver !== "undefined") {
    const observer = new IntersectionObserver(entries => {
      entries[0].isIntersecting ?
        node.dispatchEvent(new CustomEvent("enter-screen")) :
        node.dispatchEvent(new CustomEvent("leave-screen"))
    }, options)

    observer.observe(node);

    return { destroy: () => observer.disconnect() }

  } else {

    let prev: boolean;

    const fn = () => {
      const bcr = node.getBoundingClientRect();
      const is = (
        bcr.bottom > 0 &&
        bcr.right > 0 &&
        bcr.top < window.innerHeight &&
        bcr.left < window.innerWidth
      );

      if (prev !== is) {
        prev = is;
        is ?
          node.dispatchEvent(new CustomEvent("enter-screen")) :
          node.dispatchEvent(new CustomEvent("leave-screen"))
      }
    }

    fn();

    const remove = [
      add(window, "scroll", fn, { capture: true }),
      add(window, "resize", fn)
    ]

    return { 
      destroy() {
        run_all(remove);
      }
    }
  }
}

import { tick } from "svelte";

export const tooltip = (node: HTMLElement, _params: null | string | {tip: string}) => {

  const params: {tip: string | null} = ((typeof _params === "string" || _params == null) ? {tip: _params} : _params) as {tip: string | null};

  const el = document.createElement("div");
  el.classList.add("tooltip");
  el.textContent = params.tip;
  let timer: NodeJS.Timer;

  node.appendChild(el);

  let removeScroll: (() => void) | null = null;

  const removeEnter = add(node, "pointerenter", async () => {
    if(params.tip == null) return;
    clearTimeout(timer);
    el.classList.remove("visible");
    const target = node.getBoundingClientRect();
    document.body.appendChild(el);
    await tick();
    el.style.left = Math.max(5, Math.min(window.innerWidth - 5, target.left + (target.width / 2) - (el.clientWidth / 2))) + "px";
    el.style.top = Math.max(5, Math.min(window.innerHeight - 5, target.top - el.clientHeight - 7)) + "px";
    el.classList.add("visible");
    removeScroll = add(window, "scroll", () => removeTooltip(), {once: true, capture: true})
  })

  const removeTooltip = () => {
    el.classList.remove("visible");
    if(removeScroll) removeScroll();
    timer = setTimeout(() => {
      el.parentElement && el.parentElement.removeChild(el);
    }, 200)
  }

  const removeLeave = add(node, "pointerleave", removeTooltip);
  
  return {
    update(opts: string | null | {tip: string | null}) {
      if (typeof opts === "string" || opts == null) {
        opts = { tip: opts } as { tip: string | null };
      }

      params.tip = opts.tip;
      el.textContent = opts.tip;
      if(!params.tip) {
        el.parentElement && el.parentElement.removeChild(el);
      }
    },

    destroy() {
      removeEnter();
      removeLeave();
      // removeClick();
      if(removeScroll) removeScroll();
      if (el.parentElement) el.parentElement.removeChild(el);
    }
  }
}


export const clickOut = (node: Node) => {
  return { 
    destroy: add(node.ownerDocument || document, "click", (event) => {
      let target: Element | null = (event.target as Element);
      while(target != null) {
        if(target === node) return;
        target = target.parentElement;
      }

      const e = new CustomEvent("click-out", { detail: event });
      node.dispatchEvent(e)

    }, {capture: true})
  }
}

export const portal = (node: HTMLElement) => {
  document.body.appendChild(node);
  return {
    destroy() {
      node.parentElement?.removeChild(node);
    }
  }
}

export { ripple } from "./ripple";

export const intelliHide = (node: HTMLElement) => {

  let lastScroll: number | null = null;
  let state = "fix";

  const fix = (y: number) => {
      state = "fix"
      node.style.transition = "none";
      node.style.transform = `translateY(-${y}px)`
  }

  const show = () => {
      if(state === "show") return;
      state = "show";
      node.style.transition = "transform 400ms ease";
      node.style.transform = "translateY(0)";
      node.dispatchEvent(new CustomEvent("intellihide-show"));
  }

  const hide = () => {
      if(state === "hide") return;
      state = "hide";
      node.style.transition = "transform 400ms ease";
      node.style.transform = "translateY(-100%)";
      node.dispatchEvent(new CustomEvent("intellihide-hide"));
  }


  const onScroll = () => {
      const y = document.scrollingElement?.scrollTop ?? 0;
      if(lastScroll != null) {
        if(y === 0) {
            fix(y)
        } else if(y > lastScroll) {
            if(node.clientHeight > y) {
                fix(y);
            } else {
                hide();
            }
        } else if(y < lastScroll) {
            show();
        }
      }

      lastScroll = y;
  }

  onScroll();
  
  return {
    destroy: add(window, "scroll", onScroll)
  }
}