import { Router } from "express"
import { Readable } from "stream";
import { PROTOCOL_HEADER, REAL_IP_HEADER } from "./constants";
import { ip } from "./ip";

export const sveltekit_dev_proxy = (port: number) => {
  
  const proxy = Router();
  
  proxy.use(async (req, res, next) => {

    try {

      const reqHeaders = new Headers();
  
      for(const key of [
        "accept",
        "accept-language",
        "content-type",
        "content-length",
        "user-agent",
        "cookie",
        "if-none-match",
        "host",
        PROTOCOL_HEADER,
      ]) {
        const value = req.header(key);
        if(value != null) {
          reqHeaders.append(key, value);
        }
      }

      const proto = req.header(PROTOCOL_HEADER);
      if(proto) reqHeaders.set(REAL_IP_HEADER, ip(req));
      
      let back: Response;
      const url = `http://127.0.0.1:${port}${req.url}`;

      if(req.method === "GET" || req.method === "HEAD" || req.method === "DELETE") {
        back = await fetch(url, {
          method: req.method,
          headers: reqHeaders,
          redirect: "manual",
        })
      } else {
        back = await fetch(url, {
          method: req.method,
          headers: reqHeaders,
          redirect: "manual",
          // @ts-ignore
          body: Readable.toWeb(req)
        })
      }

      const resHeaders: Record<string, string | string[]> = {};
      for(const [key, value] of back.headers) {
        if(value.includes(",")) {
          resHeaders[key] = value.split(",");
        } else {
          resHeaders[key] = value;
        }
      }

      res.writeHead(back.status, resHeaders);
      if(back.body) {
        // @ts-ignore
        Readable.fromWeb(back.body).pipe(res);
      }
    } catch(e: any) {
      console.error(e);
      const cause = e?.cause;
      cause && console.error(cause)
      next(cause || e);
    }
  })

  return proxy;
}