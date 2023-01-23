import { FORWARD_IP_HEADER } from "$server/contants";
import { ApiError } from "$server/error";
import { error, redirect } from "@sveltejs/kit";
import { env } from "../env.server";
import { getEventIp } from "../ip.server";
import StatusCode from "http-status-codes";
import type { RequestEvent } from "@sveltejs/kit"

export type User = import("$server/defs/db/PublicUser").PublicUser & { media_key: string };

export const load_get_me = async (
  { request, getClientAddress }: Pick<RequestEvent, "getClientAddress" | "request">
): Promise<User | null> => {
  try {
    const { user, media_key }: { user: import("$server/defs/db/PublicUser").PublicUser, media_key: string } = await load_get("/api/users/me", { request, getClientAddress }, { redirectToLoginOnAuthErrors: false });
    return { ...user, media_key }
  } catch (e: any) {
    if(e?.status === StatusCode.UNAUTHORIZED) {
      return null;
    } else {
      throw e;
    }
  }
}

export const load_get = async <T>(
  url: string,
  { request, getClientAddress }: Pick<RequestEvent, "getClientAddress" | "request">,
  { redirectToLoginOnAuthErrors = true } = {}
): Promise<T> => {
  const headers = new Headers();
  for(const key of ["host", "cookie", "user-agent", "accept-language"]) {
    const value = request.headers.get(key);
    if(value != null) headers.set(key, value);
  }
  
  headers.set(FORWARD_IP_HEADER, getEventIp({ getClientAddress, request }));

  const res = await fetch(`http://127.0.0.1:${env.APP_API_PORT}${url}`, { headers }).catch((_e) => {
    const e = new ApiError(StatusCode.BAD_GATEWAY, "FRONT_GATEWAY_FETCH", "Bad gateway (fetch)");
    throw error(e.status, e.toJSON().error);
  }) 
  
  const body: any = await res.json().catch((_e) => {
    const e = new ApiError(StatusCode.BAD_GATEWAY, "FRONT_GATEWAY_JSON", "Bad gateway (json)");
    throw error(e.status, e.toJSON().error);
  })

  if(res.status === StatusCode.UNAUTHORIZED) {
    if(redirectToLoginOnAuthErrors) {
      throw redirect(302, "/login");
    }
  }

  if(body.error) {
    const e = ApiError.from_error_payload(body.error);
    throw error(e.status, e.toJSON().error)
  }

  return body as T
}