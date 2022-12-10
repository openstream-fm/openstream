import { Chalk } from "chalk";

const env_level = (): 0 | 1 | 2 | 3 => {
  const l = process.env.FORCE_COLOR;
  if (l === "true" || l === "1") return 2;
  if (l === "false" || l === "0") return 0;
  return 2;
}

export const color = new Chalk({ level: env_level() });

