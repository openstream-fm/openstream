import { browser } from "$app/environment";
import { ApiError } from "$server/error";
import { error, redirect, type LoadEvent } from "@sveltejs/kit";
import StatusCode from "http-status-codes";

export type User = import("$server/defs/db/PublicUser").PublicUser & { media_key: string };

export const load_get_me = async (
  { fetch, url }: Pick<LoadEvent, "fetch" | "url">
): Promise<User | null> => {
  try {
    const { user, media_key }: { user: import("$server/defs/db/PublicUser").PublicUser, media_key: string } = await load_get("/api/users/me", { fetch, url }, { redirectToLoginOnAuthErrors: false });
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
  _target: string,
  { fetch, url }: Pick<LoadEvent, "fetch" | "url">,
  { redirectToLoginOnAuthErrors = true } = {}
): Promise<T> => {

  const target = browser ? _target : `${url.origin}${_target}`

  const res = await fetch(target).catch((_e) => {
    const e = new ApiError(StatusCode.BAD_GATEWAY, "FRONT_GATEWAY_FETCH", "Bad gateway (fetch)");
    throw error(e.status, e.toJSON().error);
  }) 
  
  const body: any = await res.json().catch((_e) => {
    const e = new ApiError(StatusCode.BAD_GATEWAY, "FRONT_GATEWAY_JSON", "Bad gateway (json)");
    throw error(e.status, e.toJSON().error);
  })

  if(redirectToLoginOnAuthErrors) {
    if(res.status === StatusCode.UNAUTHORIZED) {
      throw redirect(302, "/login");
    }
  }

  if(body.error) {
    const e = ApiError.from_error_payload(body.error);
    throw error(e.status, e.toJSON().error)
  }

  return body as T
}