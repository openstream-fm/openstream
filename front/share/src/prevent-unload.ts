import { beforeNavigate } from "$app/navigation";

/**
 * return string from fn indicates prevent with message, return null means don't prevent unload
 *  */  
export const prevent_unload = (fn: () => string | null) => {
  beforeNavigate(({ willUnload, from, to, cancel }) => {
    if(from && to && from.route === to.route) return;
    let message = fn();
    if(message == null) return;
    if(willUnload) {
      cancel();
    } else {
      const ok = confirm(message);
      if(!ok) cancel();
    }
  })
}