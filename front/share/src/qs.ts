import type { JsonValue } from "$server/util/collections";
import { formatISO } from "date-fns";

export const encode_key = (k: string): string => {
  return k.replace(/[#&=\s\+]/g, encodeURIComponent)
}

export const encode_value = encodeURIComponent;

const internal_stringify = <V extends JsonValue<Date>>(v: V, k: string = "", buf: {k: string, v: string}[] = []): {k: string, v: string}[] => {
  const t = typeof v;
  if(v === undefined || t === "function" || t === "symbol") {}
  else if(t === "boolean" || t === "number" || t === "bigint") buf.push({ k, v: String(v)});
  else if(t === "string") buf.push({k, v: encode_value(v as string)})
  else if(t === "object") {
    if(v === null) buf.push({ k, v: "null" });
    else if(v instanceof Date) buf.push({ k, v: encode_value(formatISO(v)) })
    else if(v instanceof Array) {
      for(const item of v) internal_stringify(item, k, buf);
    } else {
      for(const [new_key, value] of Object.entries(v)) {
        const composed_key = k === "" ? encode_key(new_key) : `${k}[${encode_key(new_key)}]`;
        internal_stringify(value, composed_key, buf);
      }
    }
  }

  return buf;
}

export const qss = <R extends Record<string, JsonValue<Date>>>(record: R): string => {
  const values = internal_stringify(record, "");
  const helper = values.map(({k, v}) => `${k}=${v}`).join("&")
  return helper === "" ? "" : `?${helper}`;
}
