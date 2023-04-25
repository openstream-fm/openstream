module.exports = {
  apps: [{
    namespace: "s2",
    name: "openstream-2",
    exec_mode: "fork",
    time: false,
    merge_logs: true,
    kill_timeout: 60_000,
    script: "./target/release/openstream",
    args: ["start", "-c", "./openstream-2.toml"],
    instances: 1,
    env: {
      RUST_LOG_STYLE: "always",
      FORCE_COLOR: 1,
    }
  }, {
    namespace: "s2",
    script: "./front/server/dist/cli.js",
    interpreter_args: ["--no-warnings", "--experimental-specifier-resolution=node"],
    args: ["start", "-c", "./front/server/openstream-front-2.toml"],
    time: false,
    merge_logs: true,
    name: "openstream-front-2",
    env: { 
      FORCE_COLOR: "1",
      LOG_TS: "1",
    },
    instances: 4,
    mode: "cluster",
  }]
}