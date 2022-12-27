const num = (KEY: string): number => {
  const v = Number(process.env[KEY]);
  if(!v) throw new Error(`env variable '${KEY}' must be a number`)
  return v
}

export const env = {
  APP_API_PORT: num("APP_API_PORT"),
}