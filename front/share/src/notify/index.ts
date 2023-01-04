import  Notify from "./Notify.svelte";
import type { Text } from "./Notify.svelte";

let instance: Notify | null = null;

export const getNotify = (): Notify => {
  
  if(instance == null){
    // @ts-ignore
    instance = new Notify({target: document.body});
  }
  
  return instance;
}

export const _error = (text: Text) => {
  getNotify().error(text)
}

export const _message = (text: Text) => {
  getNotify().message(text);
}

export const _warn = (text: Text) => {
  getNotify().warn(text);
}

export const _success = (text: Text) => {
  getNotify().success(text);
}

export const _progress = (text: Text) => {
  const msg = getNotify().progress(text);
  
  const remove = () => getNotify().remove(msg);
  
  const resolve = (text: Text) => {
    remove();
    _message(text);
  }

  const reject = (text: Text) => {
    remove();
    _error(text);
  }

  return { remove, resolve, reject }
} 