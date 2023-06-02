import util from "util";

export const assert_never = (v: never, message: string): never => {
  throw new Error(`assert never called with message: ${message} and value: ${util.inspect(v)}`)
}
