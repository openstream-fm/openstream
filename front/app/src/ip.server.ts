import type { RequestEvent } from "@sveltejs/kit";
import { FORWARD_IP_HEADER, X_REAL_IP, LOCALHOST } from "$server/contants";

export const getEventIp = ({ request, getClientAddress }: Pick<RequestEvent, "getClientAddress" | "request">) => {
  
  let ip = getClientAddress();

  if(LOCALHOST.includes(ip)) {
    const v = request.headers.get(X_REAL_IP);
    if(v != null) ip = v;
  }

  if(LOCALHOST.includes(ip)) {
    const v = request.headers.get(FORWARD_IP_HEADER)
    if(v != null) ip = v;
  }

  return ip
}