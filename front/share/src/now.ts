import { writable } from "svelte/store";

const _now = writable(new Date());
setInterval(() => _now.set(new Date()), 1_000)

export const now = { subscribe: _now.subscribe }