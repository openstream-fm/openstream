<svelte:options accessors={true} />

<script lang="ts" context="module">
  import { get, type Readable } from "svelte/store";
  export type Text = string | Readable<string>;
</script>

<script lang="ts">
  import { flip } from "svelte/animate";
  import { ripple } from "$share/ripple";

  import {
    mdiCheckCircleOutline as successIcon,
    mdiCloseCircleOutline as errorIcon,
    mdiInformationOutline as infoIcon,
    mdiAlertOutline as warningIcon,
  } from "@mdi/js";
  import Icon from "$share/Icon.svelte";
  import CircularProgress from "$share/CircularProgress.svelte";
  import { logical_fly } from "$share/transition";

  const icons = {
    success: successIcon,
    error: errorIcon,
    info: infoIcon,
    warning: warningIcon,
  };

  type Variant = keyof typeof icons | "normal" | "progress";

  const text = (message: Message) => {
    if(!message.text) return "";
    if(typeof message.text === "string") return message.text;
    else return get(message.text);
  }
  

  type Message = {
    id: number,
    actions?: {
      text: string;
      fn: (event: Event) => void;
    }[];
    icon?: string;
    persist?: boolean;
    html?: string;
    text?: Text;
    variant: Variant;
    duration: number;
    _unsub?: () => void,
  };

  export let messages: Message[] = [];
  export let maxStack: number = 3;
  export let duration: number = 4_000;
  export let duration_error: number = 6_000;
  //export let variant = "normal";

  // messages[]
  // action?: {text: string, fn: (event) => void}
  // persist?: false
  // html?: html //or
  // text?: string
  // variant: "success" | "error" | "info" | "warning" | "normal"

  export const add = (src: Partial<Message> & { duration?: number }) => {
    let message: Message = {
      id: Math.random(),
      variant: src.variant || "normal",
      text: src.text,
      html: src.html,
      // @ts-ignore
      icon: src.icon || (src.variant && src.variant !== "normal" ? icons[src.variant] : null),
      persist: !!src.persist,
      duration: src.duration || duration,
      actions: src.actions || [],
    };

    if(!message.html && message.text && typeof message.text !== "string") {
      message._unsub = message.text.subscribe(() => {
        messages = messages;
      })
      
    }

    messages = [...messages, message];

    if (!message.persist) {
      setTimeout(() => remove(message), message.duration);
    }

    if (messages.length > maxStack) {
      remove(messages[0]);
    }

    return message;
  };

  export const remove = (message: Message) => {
    message._unsub?.();
    messages = messages.filter(item =>  item.id !== message.id);
  };

  export const clear = () => {
    for(const message of messages) {
      message._unsub?.();
    }
    messages = [];
  };

  export const message = (text: Text, message: Partial<Message> = {}) =>
    add({ variant: "normal", text, ...message });
  export const progress = (text: Text, message: Partial<Message> = {}) =>
    add({ variant: "progress", text, persist: true, ...message });
  export const success = (text: Text, message: Partial<Message> = {}) =>
    add({ variant: "success", text, ...message });
  export const info = (text: Text, message: Partial<Message> = {}) =>
    add({ variant: "info", text, ...message });
  export const warn = (text: Text, message: Partial<Message> = {}) =>
    add({ variant: "warning", text, duration: duration_error, ...message });
  export const error = (text: Text, message: Partial<Message> = {}) =>
    add({ variant: "error", text, duration: duration_error,  ...message });
</script>

<style>
  .messenger {
    position: fixed;
    z-index: 99999999999;
    inset-block-end: 0.5em;
    inset-inline-start: 0.5em;
    display: flex;
    flex-direction: column-reverse;
    align-items: flex-start;
    max-width: 80%;
  }

  .message {
    --icon-size: 1.25em;
    /*min-width: 250px;*/
    max-width: 400px;
    display: flex;
    flex-direction: column;
    padding: 0.5em 0.5em 0.5em 1em;
    color: #fff;
    background-color: rgb(50, 50, 50);
    box-shadow: 0px 3px 5px -1px rgba(0, 0, 0, 0.2),
      0px 6px 10px 0px rgba(0, 0, 0, 0.14), 0px 1px 18px 0px rgba(0, 0, 0, 0.12);
    margin: 0.5em;
    border-radius: 0.25em;
  }

  .icon {
    flex: 1;
    align-items: center;
    justify-content: center;
    display: flex;
    font-size: 1.5rem;
  }

  .message-top {
    display: flex;
    flex-direction: row;
  }

  .success {
    background-color: #43a047;
  }

  .error {
    background-color: #d32f2f;
  }

  .info {
    background-color: #1976d2;
  }

  .warning {
    background-color: #ffa000;
  }

  .message-icon {
    display: flex;
    flex: none;
    margin: auto 0;
    font-size: 1.5em;
    align-items: center;
    justify-content: center;
    height: 22px;
    width: 22px;
  }

  .message-content {
    padding: 0.25em 3em 0.25em 1em;
    line-height: 1.5em;
    margin: auto 0;
  }

  .message-content.text {
    white-space: pre-wrap;
  }

  .message-actions {
    margin: 0.75rem 1rem 1rem auto;
    flex: none;
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .action-btn {
    color: #fff;
    font-weight: 500;
    background-color: rgba(255,255,255,0.05) ;
    padding: 0.5rem 0.75rem;
  }
</style>

<div class="messenger">
  {#each messages as message (message.id)}

    <div
      transition:logical_fly={{ x: -200, duration: 250 }}
      animate:flip={{ duration: 200 }}
      class="message {message.variant} {message.variant === "progress" ? "normal" : ""}" 
    >
      <div class="message-top">  
        {#if message.icon}
          <div class="icon">
            <Icon d={message.icon} />
          </div>
        {:else if message.variant === "progress"}
          <div class="icon">
            <CircularProgress />
          </div>
        {/if}
        <div class="message-content" class:html={message.html !== null} class:text={message.html == null}>
          {#if message.html != null}
            {@html message.html}
          {:else}
            {text(message)}
          {/if}
        </div>
      </div>
      {#if message.actions?.length}
        <div class="message-actions">
          {#each message.actions as action}
            <button class="btn-light action-btn ripple-container" use:ripple on:click={(event) => action?.fn(event)}>
              {action.text}
            </button>
          {/each}
        </div>
      {/if}
    </div>
  {/each}
</div>