import type { ErrorCode } from "$server/types";
import { _error } from "./notify";

export type ClientErrorCode = ErrorCode | "CLIENT_FETCH" | "CLIENT_JSON" | "CLIENT_MISSING_CODE" | "CLIENT_PAGE_NOT_FOUND";

export class ClientError extends Error {

  status: number;
  code: ClientErrorCode;

  constructor(status: number, code: ClientErrorCode, message: string) {
    super(message);
    this.status = status;
    this.code = code;
  }
} 

export const action = <A extends any[], T>(fn: (...args: A) => T | Promise<T>) => {
  return async (...args: A) => {
    try {
      await fn(...args)
      return true;
    } catch(e: any) {
      _error(e?.message ?? "error");
      return false;
    }
  }
}

export const _request = async <T>(url: string, init: RequestInit = {}): Promise<T> => {
  const res = await fetch(url, init).catch(e => {
    throw new ClientError(0, "CLIENT_FETCH", "Could not connect with server (fetch)")
  });

  const body = await res.json().catch(e => {
    throw new ClientError(res.status, "CLIENT_JSON", "Invalid response from server (json)");
  });

  if(typeof body?.error?.message === "string") {
    throw new ClientError(res.status, (body.error.code as ErrorCode) || "CLIENT_MISSING_CODE", body.error.message);
  }

  return body as T;
}

export const _get = async <T>(url: string): Promise<T> => {
  return await _request(url);
}

export const _delete = async <T>(url: string): Promise<T> => {
  return await _request(url, {
    method: "DELETE",
  })
}

export const _post = async <T>(url: string, payload: any): Promise<T> => {
  return await _request(url, {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify(payload),
  })
}

export const _put = async <T>(url: string, payload: any): Promise<T> => {
  return await _request(url, {
    method: "PUT",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify(payload),
  })
}

export const _patch = async <T>(url: string, payload: any): Promise<T> => {
  return await _request(url, {
    method: "PATCH",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify(payload),
  })
}
