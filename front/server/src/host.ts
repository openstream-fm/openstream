import type { Request } from "express";
import type { Config, HostConfig } from "./config";

export const host = (mode: "studio" | "site" | "admin", config: Config["hosts"], req: Request): HostConfig & { id: string } => {
  const host = req.hostname;
  
  const { default: root, ...other } = config;

  for(const [ id, item ] of Object.entries(other)) {
    if(item?.[mode].host === host) {
      return { id, ...item };
    }
  }

  return { id: "default", ...root };
}