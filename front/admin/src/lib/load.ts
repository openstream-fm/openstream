import { browser } from "$app/environment";
import { ApiError } from "$server/error";
import { error, redirect, type LoadEvent } from "@sveltejs/kit";
import StatusCode from "http-status-codes";

export const load_get_me = async (
  { fetch, url }: Pick<LoadEvent, "fetch" | "url">
): Promise<(import("$server/defs/PublicAdmin").PublicAdmin & { media_key: string }) | null> => {
  try {
    const { admin, media_key }: { admin: import("$server/defs/PublicAdmin").PublicAdmin, media_key: string } = await load_get("/api/admins/me", { fetch, url }, { redirectToLoginOnAuthErrors: false });
    return { ...admin, media_key }
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
      const to = `${url.pathname}${url.search}`;
      const login_url = to === "/" ? "/login" : `/login#${encodeURIComponent(target)}`
      throw redirect(302, login_url);
    }
  }

  if(body.error) {
    const e = ApiError.from_error_payload(body.error);
    throw error(e.status, e.toJSON().error)
  }

  return body as T
}

export const not_found_load = () => {
  throw error(404, { status: 404, message: "Page not found", code: "CLIENT_PAGE_NOT_FOUND" });
}