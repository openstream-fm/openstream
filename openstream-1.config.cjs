module.exports = {
  apps: [{
    namespace: "s1",
    name: "openstream-1",
    exec_mode: "fork",
    time: false,
    merge_logs: true,
    kill_timeout: 60_000,
    script: "./target/release/openstream",
    args: ["start", "-c", "./openstream.toml"],
    instances: 1,
    env: {
      RUST_LOG_STYLE: "always",
      FORCE_COLOR: 1,
    }
  }, {
    namespace: "s1",
    script: "./front/server/dist/cli.js",
    interpreter_args: ["--no-warnings", "--experimental-specifier-resolution=node"],
    args: ["start", "-c", "./front/server/openstream-front.toml"],
    time: false,
    merge_logs: true,
    name: "openstream-front-1",
    env: { 
      FORCE_COLOR: "1",
      LOG_TS: "1",
    },
    instances: 4,
    mode: "cluster",
  }]
}