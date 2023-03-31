import type { Handle, HandleFetch } from "@sveltejs/kit";
import { env } from "./env.server";
import { FORWARD_IP_HEADER, PROTOCOL_HEADER, X_REAL_IP } from "$server/constants";
import { server_logger } from "$lib/logger.server";

export const handle: Handle = async ({ event, resolve }) => {
  const start = Date.now();

  const ip = event.request.headers.get(X_REAL_IP);
  if(ip == null) server_logger.warn(`handle: received request without ${X_REAL_IP} header: ${event.request.url}`);
  event.locals.ip = ip || "0.0.0.0";

  const proto = event.request.headers.get(PROTOCOL_HEADER);
  if(proto == null) server_logger.warn(`handle: received request without ${PROTOCOL_HEADER} header: ${event.request.url}`);
  if(proto !== "http" && proto !== "https") {
    server_logger.warn(`handle: received request with unknown ${PROTOCOL_HEADER} header (${proto}): ${event.request.url}`)
    event.locals.protocol = "http";
  } else {
    event.locals.protocol = "https";
  }

  const res = await resolve(event);

  const ms = Date.now() - start;

  server_logger[res.ok ? "debug" : "warn"](`handle: ${event.request.url} => ${res.status} ${res.statusText} - ${ms}ms`)

  return res;
}

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  const url = new URL(request.url);
  url.host = "127.0.0.1";
  url.port = String(env.APP_API_PORT);
  url.protocol = `http:`;
  
  server_logger.debug(`handle-fetch: ${event.request.url} => ${request.url} ip=${event.locals.ip} proto=${event.locals.protocol}`)

  request.headers.set(FORWARD_IP_HEADER, event.locals.ip);
  request.headers.set(PROTOCOL_HEADER, event.locals.protocol);
  const cookie = event.request.headers.get("cookie");
  if(cookie) request.headers.set("cookie", cookie);
  
  try {
    return await fetch(url, request);
  } catch(e: any) {
    server_logger.error(`handle-fetch error for ${event.request.url} => ${url}`)
    server_logger.error(e);
    throw e;
  }
}