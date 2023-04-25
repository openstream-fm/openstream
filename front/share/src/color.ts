import { browser } from "$app/environment";
import color from "kleur";

if(browser) {
  // firefox doesn't support colors in the console
  if(typeof globalThis !== "undefined" && "mozInnerScreenX" in globalThis) {
    color.enabled = false;
  }
}

export { color };

