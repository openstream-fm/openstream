export const run_all = (fns: (() => void)[]) => {
  for(const fn of fns) fn();
}


export const add = <E extends Event = Event>(target: EventTarget, event: string, fn: (event: E) => void, options: AddEventListenerOptions = {}) => {
  target.addEventListener(event, fn as any, options);
  return () => {
    target.removeEventListener(event, fn as any, options);
  }
}

export const intersect = (
  node: Element, {
    enter,
    leave,
    options = {}
  }: {
    enter: () => void,
    leave: () => void,
    options?: IntersectionObserverInit,
  }) => {
  if (typeof IntersectionObserver !== "undefined") {
    const observer = new IntersectionObserver(entries => {
      if(entries[0]?.isIntersecting) {
        enter();
      } else {
        leave();
      }  
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
        if(is) {
          enter();
        } else {
          leave();
        }
      }
    }

    fn();

    const remove = [
      add(window, "scroll", fn, { passive: true, capture: true }),
      add(window, "resize", fn)
    ]

    return { 
      destroy() {
        run_all(remove);
      }
    }
  }
}

export const click_out = (node: Node, handler: (event: MouseEvent) => void) => {
  return { 
    destroy: add(node.ownerDocument || document, "click", (event: MouseEvent) => {
      let target: Element | null = (event.target as Element);
      while(target != null) {
        if(target === node) return;
        target = target.parentElement;
      }
     

      handler(event);
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