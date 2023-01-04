export const add = (target: EventTarget, event: string, fn: EventListener, options: AddEventListenerOptions = {}) => {
  target.addEventListener(event, fn, options);
  return () => {
    target.removeEventListener(event, fn, options);
  }
}

export const tick = () => Promise.resolve();

export const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));