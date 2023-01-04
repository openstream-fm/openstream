import { error } from "@sveltejs/kit"
import type { ClientError } from "./net.client";

export const load_catch_all = async () => {
  const status = 404;
  const code: ClientError["code"] = "CLIENT_PAGE_NOT_FOUND";
  const message = "This page does not exist";
  // @ts-ignore
  throw error(status, { status, code, message });
}