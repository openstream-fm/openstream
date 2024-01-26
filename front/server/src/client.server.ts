import { Client } from "@openstream/client";

export { ClientError } from "@openstream/client";
export type { ClientErrorCode } from "@openstream/client";
export { Client }

export const get_client = (base_url: string) => {
  return new Client(base_url);
}