import Notify from "./Notify.svelte";

let instance: Notify | null = null;

export const getNotify = (): Notify => {
  
  if(instance == null){
    // @ts-ignore
    instance = new Notify({target: document.body});
  }
  
  return instance;
}

export const _error = (message: string) => {
  getNotify().error(message)
}

export const _message = (message: string) => {
  getNotify().message(message);
}

export const _warn = (message: string) => {
  getNotify().warn(message);
}

export const _success = (message: string) => {
  getNotify().success(message);
}