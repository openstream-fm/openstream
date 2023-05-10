const num = (KEY: string): number => {
  const v = Number(process.env[KEY]);
  if(typeof v !== "number") throw new Error(`env variable '${KEY}' must be a number`)
  return v
}

export const env = {
  ADMIN_API_PORT: num("ADMIN_API_PORT"),
}