import type { Handle, HandleFetch } from "@sveltejs/kit";
import { env } from "./env.server";
import { FORWARD_IP_HEADER, PROTOCOL_HEADER, X_REAL_IP } from "$server/constants";
import { server_logger } from "$lib/logger.server";

export const handle: Handle = async ({ event, resolve }) => {
  
  if(event.url.pathname === "/internal-error.html") {
    throw new Error("Internal error");
  }

  const start = Date.now();

  const ip = event.request.headers.get(X_REAL_IP);
  if(ip == null) server_logger.warn(`handle: received request without ${X_REAL_IP} header: ${event.request.url}`);
  event.locals.ip = ip || "0.0.0.0";

  let proto_header = event.request.headers.get(PROTOCOL_HEADER);
  let proto: "http" | "https";
  if(proto_header == null) {
    server_logger.warn(`handle: received request without ${PROTOCOL_HEADER} header: ${event.request.url}`);
    proto = "http";
  } else if(proto_header !== "http" && proto_header !== "https") {
    server_logger.warn(`handle: received request with unknown ${PROTOCOL_HEADER} header (${proto_header}): ${event.request.url}`)
    proto = "http";
  } else {
    proto = proto_header;
  }

  event.locals.protocol = proto;

  const res = await resolve(event);

  // for(const cookie of event.locals.set_cookie) {
  //   res.headers.append("set-cookie", cookie);
  // }

  const ms = Date.now() - start;

  const ok = res.status >= 200 && res.status <= 399;
  const message = `handle: ${event.request.url} => ${res.status} ${res.statusText} - ${ms}ms`;

  if(ok) server_logger.debug(message);
  else server_logger.warn(message);

  return res;
}

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  const url = new URL(request.url);
  url.host = "127.0.0.1";
  url.port = String(env.ADMIN_API_PORT);
  url.protocol = `http:`;
  
  server_logger.debug(`handle-fetch: ${event.request.url} => ${request.url} ip=${event.locals.ip} proto=${event.locals.protocol}`)

  const target = new Request(request)

  for(const key of [
    "x-forwarded-proto",
    "x-forwarded-for",
    "accept-language",
    "user-agent",
    "host",
    "cookie"
  ]) {
    const v = event.request.headers.get(key);
    if(v) target.headers.set(key, v);
  }
  
  target.headers.set(FORWARD_IP_HEADER, event.locals.ip);
  target.headers.set(PROTOCOL_HEADER, event.locals.protocol);
  target.headers.set("x-kit-url", event.request.url);

  // const src_cookies = (event.request.headers.get("cookie")?.split(";") || []).map(s => s.trim());
  // const cookie = [...new Set([...src_cookies, ...event.locals.cookie])].join("; ").trim(); 
  // if(cookie) target.headers.set("cookie", cookie);
    
  try {
    const res = await fetch(url, {
      method: target.method,
      headers: target.headers,
      body: target.body,
      // mode: "same-origin"
    });

    // const set_cookie = res.headers.get("set-cookie");
    // if(set_cookie) {
    //   event.locals.set_cookie.add(set_cookie);
    //   const cookie = set_cookie.split(";")[0];
    //   if(cookie) event.locals.cookie.add(cookie);
    // }

    return res;

  } catch(e: any) {
    server_logger.error(`handle-fetch error for ${event.request.url} => ${url}`)
    server_logger.error(e?.cause ? e.cause : e);
    throw e;
  }
}