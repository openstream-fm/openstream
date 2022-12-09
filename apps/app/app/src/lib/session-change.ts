import { browser } from "$app/environment";

type Listener = () => void;
const KEY = "openstream.sessionChange";
const fns = new Map<number, Listener>();
let i = 0;

export const onTabSessionChange = (fn: Listener) => {
  if(browser) {
    const idx = i++;  
    fns.set(idx, fn);
    return () => {
      fns.delete(idx);
    }
  } else {
    console.warn("onTabSessionChange called in server side");
    // eslint-disable-next-line
    return () => {};
  }
}

export const dispatchTabSessionChange = () => {
  if(browser) {
    const id = Number(localStorage.getItem(KEY)) || 0;
    localStorage.setItem(KEY, `${id + 1}`);
  } else {
    console.warn("dispatchTabSessionChange called in server side");
  }
}

if(browser) {
  window.addEventListener("storage", event => {
    if(event.key === KEY) {
      for(const fn of fns.values()) {
        fn();
      }
    }
  })
}