export const assert_never = (v: never, message: string): never => {
  throw new Error(`assert never called with message: ${message} and value: ${JSON.stringify(v)}`)
}
