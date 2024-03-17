import { browser } from "$app/environment";
import { ApiError } from "$server/error";
import { error, redirect, type LoadEvent, type NumericRange } from "@sveltejs/kit";
import StatusCode from "http-status-codes";

import type { paths } from  "../../../../openapi";
import createClient from  "openapi-fetch";
export const client = createClient<paths>({
  baseUrl: browser ? "/api/" : "https://internal.test/api/",
});

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
    error(e.status as NumericRange<400, 599>, e.toJSON().error);
  }) 
  
  const body: any = await res.json().catch((_e) => {
    const e = new ApiError(StatusCode.BAD_GATEWAY, "FRONT_GATEWAY_JSON", "Bad gateway (json)");
    error(e.status as NumericRange<400, 599>, e.toJSON().error);
  })

  if(redirectToLoginOnAuthErrors) {
    if(res.status === StatusCode.UNAUTHORIZED) {
      const to = `${url.pathname}${url.search}`;
      const login_url = to === "/" ? "/login" : `/login#${target}`
      redirect(302, login_url);
    }
  }

  if(body.error) {
    const e = ApiError.from_error_payload(body.error);
    error(e.status as NumericRange<400, 599>, e.toJSON().error);
  }

  return body as T
}

export const load_call = async <T>(
  fn: () => Promise<
    | { data: T, error?: undefined }
    | { data?: undefined, error: { error: { status: number, code: import("$defs/error/PublicErrorCode").PublicErrorCode, message: string } } }
  >,
  { redirectToLoginOnAuthErrors =  true } = {}
): Promise<NonNullable<T>> => {
  try {
    const result = await fn();
    if(result.error === undefined) {
      return result.data!;
    } else {
      if(result.error?.error?.status === StatusCode.UNAUTHORIZED && redirectToLoginOnAuthErrors) {
        const to = typeof location === "undefined" ? "/" : `${location.pathname}${location.search}`;
        const login_url = to === "/" ? "/login" : `/login#${to}`
        redirect(302, login_url);
      } else {
        const api_error = ApiError.from_error_payload(result.error);
        error(api_error.status as NumericRange<400, 599>, api_error.toJSON().error);
      }
    }
  } catch(e: any) {
    console.log(e);
    const api_error = new ApiError(502, "FRONT_GATEWAY_FETCH", `Bad gateway (fetch)`);
    error(api_error.status as NumericRange<400, 599>, api_error.toJSON().error);
  }
}

export const not_found_load = () => {
  error(404, { status: 404, message: "Page not found", code: "CLIENT_PAGE_NOT_FOUND" });
}