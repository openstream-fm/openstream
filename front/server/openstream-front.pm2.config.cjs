module.exports = {
  apps: {
    script: "./openstream-front.js",
    args: ["start"],
    interpreter_args: ["--no-warnings", "--experimental-specifier-resolution=node"],
    name: "openstream-front",
    env: { 
      FORCE_COLOR: "1",
      LOG_TS: "1",
    },
    instances: 2,
    mode: "cluster",
  }
}