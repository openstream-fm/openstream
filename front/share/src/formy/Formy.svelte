<script lang="ts">
  export let action: () => void;

  import { setContext } from "svelte";
  import { FORMY_KEY } from "./formy";
  import type { FormyContext, ValidationItem } from "./formy"
  import { default_logger } from "$share/logger";

  const logger = default_logger.scoped("formy");

  let _token = 0;
  const map = new Map<number, ValidationItem>();

  const add = (item: ValidationItem) => {
    const token = _token++;
    map.set(token, item);
    return () => {
      map.delete(token);
    }
  }

  const context: FormyContext = { add };
  setContext(FORMY_KEY, context);

  const submit = (event: SubmitEvent | void) => {
    event?.preventDefault();
    let fails: { message: string, element: Element | null }[] = [];
    let element: Element | null = null;
    for(const item of map.values()) {
      const message = item.fn();
      if(message != null) {
        if(element == null) element = item.parent_element;
        fails.push({ message, element: item.parent_element });
      }
    }
    if(fails.length === 0) {
      action();
    } else {
      logger.warn("validation fails", fails)
      if(element != null) {
        element.scrollIntoView({ behavior: "smooth", block: "center", inline: "center" });
      }
    }
  }
</script>

<slot {submit}></slot>