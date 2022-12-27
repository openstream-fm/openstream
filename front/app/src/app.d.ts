// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
declare namespace App {
  interface Error {
    status: number
    code: import("$server/types").ErrorCode,
    message: string
  }
}
