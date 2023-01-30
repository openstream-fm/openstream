// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

// and what to do when importing types
declare namespace App {

  interface Locals {
    ip: string,
    protocol: "http" | "https",
  }

  interface Error {
    status: number
    code: import("$lib/net.client").ClientErrorCode,
    message: string
  }

  interface PageData {
    config: import("$server/config").Config["public"],
  }
}
