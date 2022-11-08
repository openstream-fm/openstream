export type Data = {
  storage: { avail: number, used: number },
  listeners: { avail: number, used: number },
  bandwidth: { avail: number, used: number },
  on_air: boolean,
  live_streaming: boolean,
  ice: {
    host: string
    port: number
    mount: string
    user: string
    password: string
    encoding: string,
  }
}