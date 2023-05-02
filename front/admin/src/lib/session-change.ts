import { browser } from "$app/environment";
import { default_logger } from "$server/logger";

type Listener = () => void;
const KEY = "openstream.sessionChange";
const fns = new Map<number, Listener>();
let i = 0;

const logger = default_logger.scoped("intertab");

export const on_tab_session_change = (fn: Listener) => {
  if(browser) {
    const idx = i++;  
    fns.set(idx, fn);
    return () => {
      fns.delete(idx);
    }
  } else {
    logger.warn("on_tab_session_change called in server side");
    // eslint-disable-next-line
    return () => {};
  }
}

export const dispatch_tab_session_change = () => {
  if(browser) {
    const id = Number(localStorage.getItem(KEY)) || 0;
    localStorage.setItem(KEY, `${id + 1}`);
  } else {
    logger.warn("dispatch_tab_session_change called in server side");
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