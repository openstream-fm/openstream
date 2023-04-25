module.exports = {
  apps: {
    namespace: "s2",
    script: "./dist/cli.js",
    args: ["start", "-c", "./openstream-front-2.toml"],
    interpreter_args: ["--no-warnings", "--experimental-specifier-resolution=node"],
    name: "openstream-front-2",
    env: { 
      FORCE_COLOR: "1",
      LOG_TS: "1",
    },
    instances: 4,
    mode: "cluster",
  }
}