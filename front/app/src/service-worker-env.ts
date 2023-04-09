// Workbox will test this property, if not set it will throw an error
const NODE_ENV = import.meta.env.WORKBOX_DEV === "1" ? "development" : "production";
// @ts-ignore
self.process = { browser: false, env: { NODE_ENV } };