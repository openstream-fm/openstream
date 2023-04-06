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
    config: import("$server/config").Config["public"]
    user?: import("$server/api/me/GET/Output").Output["user"] | null
    accounts?: import("$server/api/accounts/GET/Output").Output | null 
    account?: import("$server/api/accounts/[account]/GET/Output").Output["account"] | null
    stations?: import("$server/api/stations/GET/Output").Output | null
    station?: import("$server/api/stations/[station]/GET/Output").Output["station"] | null
  }
}
