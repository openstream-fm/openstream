// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

// and what to do when importing types
declare namespace App {
  interface Error {
    status: number
    code: import("$lib/net.client").ClientErrorCode,
    message: string
  }

  interface Data {
    config: import("$server/config").Config["public"]
  }
}
