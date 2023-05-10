// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types

declare namespace App {

  interface Locals {
    ip: string,
    protocol: "http" | "https",
    // set_cookie: Set<string>,
    // cookie: Set<string>
  }

  interface Error {
    status: number
    code: import("$lib/net.client").ClientErrorCode,
    message: string
  }

  interface PageData {
    config?: import("$server/admin-api").PublicConfig,
    maybe_admin?: (import("$server/defs/PublicAdmin").PublicAdmin & { media_key: string }) | null,
    admin?: (import("$server/defs/PublicAdmin").PublicAdmin & { media_key: string }) | null,
    admins?: import("$server/api/admins/GET/Output").Output | null 
    users?: import("$server/api/users/GET/Output").Output | null 
    accounts?: import("$server/api/accounts/GET/Output").Output | null 
    stations?: import("$server/api/stations/GET/Output").Output | null
  }
}
