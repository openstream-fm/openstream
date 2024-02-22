import type { paths } from "../../../../openapi";
import createClient from "openapi-fetch";

export const { GET, POST, PUT, PATCH, DELETE } = createClient<paths>({
  baseUrl: "/api/"
})

export type Unwrap<R> = R extends { data: infer T } ? Exclude<T, undefined> : never;

export const unwrap = <T>(
  result: 
    | { data: T, error?: undefined }
    | { data?: undefined, error: { error: { status: number, code: string, message: string } } }
): NonNullable<T> => {
  if (result.error) throw new Error(result.error.error.message)
  return result.data!;
}