const NODE_ENV = import.meta.env.WORKBOX_DEV === "1" ? "development" : "production";
// Workbox will test this property, if not set it will throw an error
// @ts-expect-error
self.process = { browser: false, env: { NODE_ENV } };