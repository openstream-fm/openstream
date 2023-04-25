module.exports = {
  apps: {
    namespace: "s1",
    script: "./dist/cli.js",
    args: ["start"],
    interpreter_args: ["--no-warnings", "--experimental-specifier-resolution=node"],
    name: "openstream-front-1",
    env: { 
      FORCE_COLOR: "1",
      LOG_TS: "1",
    },
    instances: 4,
    mode: "cluster",
  }
}